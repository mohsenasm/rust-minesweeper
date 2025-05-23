use std::io::{Error, ErrorKind, Result, Stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{Clear, ClearType},
};

use rand::Rng;

use crate::theme::{get_theme, rotate_theme_name, Theme};

#[derive(Clone)]
pub struct Cell {
    pub is_bomb: bool,
    pub number_of_adjusted_bombs: u8,

    pub is_flagged: bool,
    pub is_discovered: bool,
}

pub fn init_blank_cell() -> Cell {
    Cell {
        is_bomb: false,
        number_of_adjusted_bombs: 0,
        is_flagged: false,
        is_discovered: false,
    }
}

impl Cell {
    pub fn content_to_show(&self, theme: &Theme, selected: bool) -> String {
        if self.is_discovered {
            if self.is_bomb {
                return theme.bomb.clone();
            } else {
                if self.number_of_adjusted_bombs == 0 {
                    return theme.empty.clone();
                } else {
                    return theme
                        .format_number_of_adjusted_bombs(self.number_of_adjusted_bombs, selected);
                }
            }
        } else {
            if self.is_flagged {
                return theme.flag.clone();
            } else {
                return theme.unknown.clone();
            }
        }
    }
}

pub struct Board {
    pub theme: Theme,
    pub size: (usize, usize),
    pub cells: Vec<Vec<Cell>>,
    pub number_of_bombs: usize,
    pub remaining_flags: usize,
    pub selected_cell: Option<(usize, usize)>,
    need_to_draw: bool,
}

pub fn init_random_game(size: (usize, usize), bomb_percentage: f32, theme: Theme) -> Board {
    let mut game_board = Board {
        theme,
        size,
        cells: vec![vec![init_blank_cell(); size.1]; size.0],
        number_of_bombs: 0,
        remaining_flags: 0,
        selected_cell: None,
        need_to_draw: true,
    };

    // generate bombs
    game_board.number_of_bombs = ((((size.0 * size.1) as f32) * bomb_percentage).round()) as usize;
    game_board.remaining_flags = game_board.number_of_bombs;
    let mut remaning_bombs = game_board.number_of_bombs;
    let mut random = rand::thread_rng();
    while remaning_bombs > 0 {
        let x = random.gen_range(0..size.0);
        let y = random.gen_range(0..size.1);
        if !game_board.cells[x][y].is_bomb {
            game_board.cells[x][y].is_bomb = true;
            remaning_bombs -= 1;
        }
    }
    game_board.fill_numbers();

    // make a starting point
    game_board.hint();

    game_board
}

impl Board {
    fn fill_numbers(&mut self) {
        for row in 0..self.size.0 {
            for column in 0..self.size.1 {
                for index in self.get_adjusted_indices((row, column)) {
                    if self.cells[index.0][index.1].is_bomb {
                        self.cells[row][column].number_of_adjusted_bombs += 1;
                    }
                }
            }
        }
    }

    pub fn hint(&mut self) {
        let mut random = rand::thread_rng();
        let mut non_bomb_cells: Vec<(u8, (usize, usize))> = Vec::new();
        for row in 0..self.size.0 {
            for column in 0..self.size.1 {
                if !self.cells[row][column].is_bomb && !self.cells[row][column].is_discovered {
                    non_bomb_cells.push((
                        self.cells[row][column].number_of_adjusted_bombs,
                        (row, column),
                    ));
                }
            }
        }
        if non_bomb_cells.len() > 0 {
            non_bomb_cells.sort_by_key(|x: &(u8, (usize, usize))| x.0);
            let min_number_of_adjusted_bombs = non_bomb_cells[0].0;
            let mut last_index: usize = 0;
            for (index, cell) in non_bomb_cells.iter().enumerate() {
                if cell.0 == min_number_of_adjusted_bombs {
                    last_index = index
                } else {
                    break;
                }
            }
            let i = random.gen_range(0..=last_index);
            self.discover_cell(non_bomb_cells[i].1);
        }
    }

    pub fn mouse_hover(&mut self, mouse_row: usize, mouse_column: usize) {
        let index = self.convert_mouse_to_index(mouse_row, mouse_column);
        if let Some((row, column)) = index {
            if self.selected_cell != Some((row, column)) {
                self.selected_cell = Some((row, column));
                self.need_to_draw = true;
            }
        }
    }

    pub fn mouse_down(&mut self, mouse_row: usize, mouse_column: usize, left_key: bool) {
        let index = self.convert_mouse_to_index(mouse_row, mouse_column);
        if let Some((row, column)) = index {
            self.intract_with_cell(row, column, !left_key);
        }
    }

    pub fn intract_with_cell(&mut self, row: usize, column: usize, alternate_key: bool) {
        if self.cells[row][column].is_discovered {
            // fill flags for adjusted cells, if possible
            self.discover_or_flag_adjusted_cells((row, column));
        } else {
            if !alternate_key {
                // discover or undo flag
                if self.cells[row][column].is_flagged {
                    self.set_cell_flag((row, column), false);
                } else {
                    self.discover_cell((row, column));
                }
            } else {
                // flag cell
                if !self.cells[row][column].is_flagged {
                    self.set_cell_flag((row, column), true);
                }
            }
        }
    }

    // move at direction specified with dr and dc while skipping the blank cells
    pub fn move_selection(&mut self, dr: isize, dc: isize) {
        let mut next = if let Some((r, c)) = self.selected_cell {
            (r as isize, c as isize)
        } else {
            (0, 0)
        };

        loop {
            next.0 = next.0 + dr;
            next.1 = next.1 + dc;

            // If out of bounds, stop
            if next.0 < 0
                || next.1 < 0
                || next.0 >= self.size.0 as isize
                || next.1 >= self.size.1 as isize
            {
                break;
            }

            let cell = &self.cells[next.0 as usize][next.1 as usize];

            if cell.is_discovered && cell.number_of_adjusted_bombs == 0 {
                continue;
            } else {
                self.selected_cell = Some((next.0 as usize, next.1 as usize));
                self.need_to_draw = true;
                break;
            }
        }
    }

    pub fn flag_selected(&mut self) {
        if let Some((r, c)) = self.selected_cell {
            if !self.cells[r][c].is_discovered {
                let flag = !self.cells[r][c].is_flagged;
                self.set_cell_flag((r, c), flag);
                self.need_to_draw = true;
            }
        }
    }

    pub fn open_selected(&mut self) {
        if let Some((r, c)) = self.selected_cell {
            self.intract_with_cell(r, c, false);
        }
    }

    pub fn draw(&mut self, mut stdout: &Stdout) -> Result<()> {
        if !self.need_to_draw {
            return Ok(());
        } else {
            self.need_to_draw = false;
        }
        // clear terminal
        queue!(stdout, Clear(ClearType::All))?;
        queue!(stdout, Clear(ClearType::Purge))?;
        queue!(stdout, MoveTo(0, 0))?;

        for row in 0..self.size.0 {
            // each row has two parts, one for border and one for the content
            // outer/inner border row
            let mut line1 = String::new();
            for column in 0..self.size.1 {
                let selected = Some((row, column)) == self.selected_cell;
                let selected_on_left = column > 0 && Some((row, column - 1)) == self.selected_cell;
                let selected_on_top = row > 0 && Some((row - 1, column)) == self.selected_cell;
                let selected_on_top_left =
                    row > 0 && column > 0 && Some((row - 1, column - 1)) == self.selected_cell;
                if row == 0 && column == 0 {
                    line1 += &self.theme.format_corner_top_left(selected);
                } else if row == 0 && column != 0 {
                    line1 += &self.theme.format_edge_top(selected || selected_on_left);
                } else if row != 0 && column == 0 {
                    line1 += &self.theme.format_edge_left(selected || selected_on_top);
                } else if row != 0 && column != 0 {
                    line1 += &self.theme.format_cross(
                        selected || selected_on_left || selected_on_top || selected_on_top_left,
                    );
                }
                // line (for space) + line (for content) + line (for space)
                line1 += &self
                    .theme
                    .format_horizontal_border(selected || selected_on_top);
                if self.theme.cell_horizontal_padding_enabled {
                    line1 += &self
                        .theme
                        .format_horizontal_border(selected || selected_on_top);
                    line1 += &self
                        .theme
                        .format_horizontal_border(selected || selected_on_top);
                }
            }
            // outer border of the last column
            let selected = Some((row, self.size.1 - 1)) == self.selected_cell;
            let selected_on_top = row > 0 && Some((row - 1, self.size.1 - 1)) == self.selected_cell;
            if row == 0 {
                line1 += &self.theme.format_corner_top_right(selected);
            } else {
                line1 += &self.theme.format_edge_right(selected || selected_on_top);
            }
            if (row == 0 && self.theme.outer_border_enabled)
                || (row != 0 && self.theme.inner_border_row_enabled)
            {
                println!("{}\r", line1);
            }
            // content row
            let mut line2 = String::new();
            for column in 0..self.size.1 {
                let selected = Some((row, column)) == self.selected_cell;
                let selected_on_left = column > 0 && Some((row, column - 1)) == self.selected_cell;

                if (column == 0 && self.theme.outer_border_enabled)
                    || (column != 0 && self.theme.inner_border_column_enabled)
                {
                    line2 += &self
                        .theme
                        .format_vertical_border(selected || selected_on_left);
                }
                if self.theme.cell_horizontal_padding_enabled {
                    line2 += &self.theme.cell_horizontal_padding;
                }
                let cell_content = self.cells[row][column].content_to_show(&self.theme, selected);
                line2 += &self.theme.format_cell_content(&cell_content, selected);
                if self.theme.cell_horizontal_padding_enabled {
                    line2 += &self.theme.cell_horizontal_padding;
                }
            }
            if self.theme.outer_border_enabled {
                let sel = Some((row, self.size.1 - 1)) == self.selected_cell;
                line2 += &self.theme.format_vertical_border(sel);
            }
            println!("{}\r", line2);
        }

        // outer border of the last row
        let mut line3 = String::new();
        for column in 0..self.size.1 {
            let selected = Some((self.size.0 - 1, column)) == self.selected_cell;
            let selected_on_left =
                column > 0 && Some((self.size.0 - 1, column - 1)) == self.selected_cell;
            if column == 0 {
                line3 += &self.theme.format_corner_bottom_left(selected);
            } else {
                line3 += &self.theme.format_edge_bottom(selected || selected_on_left);
            }
            // line (for space) + line (for content) + line (for space)
            line3 += &self.theme.format_horizontal_border(selected);
            if self.theme.cell_horizontal_padding_enabled {
                line3 += &self.theme.format_horizontal_border(selected);
                line3 += &self.theme.format_horizontal_border(selected);
            }
        }
        let selected = Some((self.size.0 - 1, self.size.1 - 1)) == self.selected_cell;
        line3 += &self.theme.format_corner_bottom_right(selected);
        if self.theme.outer_border_enabled {
            println!("{}\r", line3);
        }

        println!("remaining flags: {}\r", self.remaining_flags);

        let mut bombs_without_flag = 0;
        let mut non_bomb_cells_undicovered = 0;

        for row in 0..self.size.0 {
            for column in 0..self.size.1 {
                if self.cells[row][column].is_bomb && self.cells[row][column].is_discovered {
                    return Err(Error::new(ErrorKind::BrokenPipe, "Boom!!"));
                }
                if self.cells[row][column].is_bomb && !self.cells[row][column].is_flagged {
                    bombs_without_flag += 1;
                }
                if !self.cells[row][column].is_bomb && !self.cells[row][column].is_discovered {
                    non_bomb_cells_undicovered += 1;
                }
            }
        }

        if bombs_without_flag == 0 || non_bomb_cells_undicovered == 0 {
            return Err(Error::new(ErrorKind::BrokenPipe, "You Won :)"));
        }

        Ok(())
    }

    // it is O(n), TODO: can be O(1) but may make themes complicated.
    fn convert_mouse_to_index(
        &self,
        mouse_row: usize,
        mouse_column: usize,
    ) -> Option<(usize, usize)> {
        let column: Option<usize> = {
            let mut result: Option<usize> = None;

            let mut start_index: usize;
            let mut end_index: usize = 0;
            for test_column in 0..self.size.1 {
                start_index = end_index;
                if (test_column == 0 && self.theme.outer_border_enabled)
                    || (test_column != 0 && self.theme.inner_border_column_enabled)
                {
                    start_index += 1;
                    end_index += 1;
                }
                if self.theme.cell_horizontal_padding_enabled {
                    end_index += 1;
                }
                end_index += 1;
                if self.theme.cell_horizontal_padding_enabled {
                    end_index += 1;
                }
                if start_index <= mouse_column && mouse_column < end_index {
                    result = Some(test_column);
                    break;
                }
            }
            result
        };

        let row: Option<usize> = {
            let mut result: Option<usize> = None;

            let mut start_index: usize;
            let mut end_index: usize = 0;
            for test_row in 0..self.size.0 {
                start_index = end_index;
                if (test_row == 0 && self.theme.outer_border_enabled)
                    || (test_row != 0 && self.theme.inner_border_row_enabled)
                {
                    start_index += 1;
                    end_index += 1;
                }
                end_index += 1;
                if start_index <= mouse_row && mouse_row < end_index {
                    result = Some(test_row);
                    break;
                }
            }

            result
        };

        if let Some(r) = row {
            if let Some(c) = column {
                return Some((r, c));
            }
        }
        None
    }

    fn discover_cell(&mut self, (row, column): (usize, usize)) {
        if !self.cells[row][column].is_discovered && !self.cells[row][column].is_flagged {
            self.cells[row][column].is_discovered = true;
            self.need_to_draw = true;
            if self.cells[row][column].number_of_adjusted_bombs == 0 {
                for index in self.get_adjusted_indices((row, column)) {
                    self.discover_cell(index)
                }
            }
        }
    }

    fn set_cell_flag(&mut self, (row, column): (usize, usize), flag: bool) {
        if self.cells[row][column].is_flagged != flag {
            self.need_to_draw = true;
            // need to change
            if flag {
                if self.remaining_flags > 0 {
                    self.cells[row][column].is_flagged = flag;
                    self.remaining_flags -= 1;
                }
            } else {
                self.cells[row][column].is_flagged = flag;
                self.remaining_flags += 1;
            }
        } else {
            panic!("set_cell_flag: is_flagged == flag")
        }
    }

    fn discover_or_flag_adjusted_cells(&mut self, (row, column): (usize, usize)) {
        let adjusted_indices = &self.get_adjusted_indices((row, column));
        let mut number_of_unknown_adjusted_cells = 0;
        let mut number_of_flagged_adjusted_cells = 0;
        for index in adjusted_indices {
            if !self.cells[index.0][index.1].is_discovered
                && !self.cells[index.0][index.1].is_flagged
            {
                number_of_unknown_adjusted_cells += 1;
            }
            if self.cells[index.0][index.1].is_flagged {
                number_of_flagged_adjusted_cells += 1;
            }
        }

        if self.cells[row][column].number_of_adjusted_bombs == number_of_flagged_adjusted_cells {
            for index in adjusted_indices {
                self.discover_cell(*index);
            }
        } else if self.cells[row][column].number_of_adjusted_bombs
            == number_of_flagged_adjusted_cells + number_of_unknown_adjusted_cells
        {
            for index in adjusted_indices {
                if !self.cells[index.0][index.1].is_discovered
                    && !self.cells[index.0][index.1].is_flagged
                {
                    self.set_cell_flag((index.0, index.1), true);
                }
            }
        }
    }

    fn get_adjusted_indices(&mut self, (row, column): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();

        // top
        if row > 0 {
            if column > 0 {
                result.push((row - 1, column - 1));
            }
            result.push((row - 1, column));
            if column + 1 < self.size.1 {
                result.push((row - 1, column + 1));
            }
        }
        // side
        if column > 0 {
            result.push((row, column - 1));
        }
        if column + 1 < self.size.1 {
            result.push((row, column + 1));
        }
        // bottom
        if row + 1 < self.size.0 {
            if column > 0 {
                result.push((row + 1, column - 1));
            }
            result.push((row + 1, column));

            if column + 1 < self.size.1 {
                result.push((row + 1, column + 1));
            }
        }

        result
    }

    pub fn change_theme(&mut self) {
        if let Some(theme) = get_theme(&rotate_theme_name(&self.theme.name)) {
            self.theme = theme;
            self.need_to_draw = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::border_theme;

    use super::*;

    #[test]
    fn convet_mouse_to_index() {
        let game_board = init_random_game((5, 10), 0.3, border_theme());

        assert_eq!(game_board.convert_mouse_to_index(0, 0), None);
        assert_eq!(game_board.convert_mouse_to_index(0, 1), None);
        assert_eq!(game_board.convert_mouse_to_index(0, 2), None);
        assert_eq!(game_board.convert_mouse_to_index(0, 3), None);
        assert_eq!(game_board.convert_mouse_to_index(0, 4), None);

        assert_eq!(game_board.convert_mouse_to_index(1, 0), None);
        assert_eq!(game_board.convert_mouse_to_index(1, 1), Some((0, 0)));
        assert_eq!(game_board.convert_mouse_to_index(1, 2), Some((0, 0)));
        assert_eq!(game_board.convert_mouse_to_index(1, 3), Some((0, 0)));
        assert_eq!(game_board.convert_mouse_to_index(1, 4), None);
        assert_eq!(game_board.convert_mouse_to_index(1, 5), Some((0, 1)));
        assert_eq!(game_board.convert_mouse_to_index(1, 6), Some((0, 1)));
        assert_eq!(game_board.convert_mouse_to_index(1, 7), Some((0, 1)));
        assert_eq!(game_board.convert_mouse_to_index(1, 8), None);
    }
}
