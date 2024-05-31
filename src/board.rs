use std::io::{Error, ErrorKind, Result, Stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    terminal::{Clear, ClearType},
};

use rand::Rng;

use crate::theme::Theme;

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
    pub fn content_to_show(&self, theme: &Theme) -> String {
        if self.is_discovered {
            if self.is_bomb {
                return theme.bomb.clone();
            } else {
                if self.number_of_adjusted_bombs == 0 {
                    return theme.empty.clone();
                } else {
                    return self.number_of_adjusted_bombs.to_string();
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
}

pub fn init_random_game(size: (usize, usize), bomb_percentage: f32, theme: Theme) -> Board {
    let mut game_board = Board {
        theme,
        size,
        cells: vec![vec![init_blank_cell(); size.1]; size.0],
        number_of_bombs: 0,
        remaining_flags: 0,
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

    // fill numbers
    for row in 0..size.0 {
        for column in 0..size.1 {
            for index in game_board.get_adjusted_indices((row, column)) {
                if game_board.cells[index.0][index.1].is_bomb {
                    game_board.cells[row][column].number_of_adjusted_bombs += 1;
                }
            }
        }
    }

    // make a starting point
    // TODO: sort and get a number_of_adjusted_bombs==0
    if game_board.number_of_bombs < size.0 * size.1 {
        loop {
            let x = random.gen_range(0..size.0);
            let y = random.gen_range(0..size.1);
            if game_board.cells[x][y].is_bomb == false {
                game_board.discover_cell((x, y));
                break;
            }
        }
    }

    game_board
}

impl Board {
    pub fn mouse_hover(&mut self, _row: usize, _column: usize) {
        // println!("mouse_hover: {} {} \r", row, column)
    }

    pub fn mouse_down(&mut self, mouse_row: usize, mouse_column: usize, left_key: bool) {
        let index = self.convet_mouse_to_index(mouse_row, mouse_column);
        if let Some((row, column)) = index {
            if self.cells[row][column].is_discovered {
                // fill flags for adjusted cells, if possible
                self.discover_or_flag_adjusted_cells((row, column));
            } else {
                // discover cell
                if left_key {
                    if self.cells[row][column].is_flagged {
                        self.set_cell_flag((row, column), false);
                    } else {
                        self.discover_cell((row, column));
                    }
                } else {
                    self.set_cell_flag((row, column), true);
                }
            }
        }
    }

    pub fn draw(&self, mut stdout: &Stdout) -> Result<()> {
        // clear terminal
        queue!(stdout, Clear(ClearType::All))?;
        queue!(stdout, Clear(ClearType::Purge))?;
        queue!(stdout, MoveTo(0, 0))?;

        for row in 0..self.size.0 {
            // each row has two lines, one for border and one for the content
            // border line
            let mut line1 = String::new();
            for column in 0..self.size.1 {
                if row == 0 && column == 0 {
                    line1 += &self.theme.corner_top_left;
                } else if row == 0 && column != 0 {
                    line1 += &self.theme.edge_top;
                } else if row != 0 && column == 0 {
                    line1 += &self.theme.edge_left;
                } else if row != 0 && column != 0 {
                    line1 += &self.theme.line_cross;
                }
                line1 += &self.theme.line_horizontal;
                if self.theme.cell_horizontal_padding_enabled {
                    line1 += &self.theme.line_horizontal;
                    line1 += &self.theme.line_horizontal;
                }
            }
            // border of the last column
            if row == 0 {
                line1 += &self.theme.corner_top_right;
            } else {
                line1 += &self.theme.edge_right;
            }
            println!("{}\r", line1);
            // content line
            let mut line2 = String::new();
            for column in 0..self.size.1 {
                line2 += &self.theme.line_vertical;
                if self.theme.cell_horizontal_padding_enabled {
                    line2 += &self.theme.cell_horizontal_padding;
                }
                line2 += &self.cells[row][column].content_to_show(&self.theme);
                if self.theme.cell_horizontal_padding_enabled {
                    line2 += &self.theme.cell_horizontal_padding;
                }
            }
            line2 += &self.theme.line_vertical;
            println!("{}\r", line2);
        }
        // border of the last row
        let mut line3 = String::new();
        for column in 0..self.size.1 {
            if column == 0 {
                line3 += &self.theme.corner_bottom_left;
            } else {
                line3 += &self.theme.edge_bottom;
            }
            line3 += &self.theme.line_horizontal;
            if self.theme.cell_horizontal_padding_enabled {
                line3 += &self.theme.line_horizontal;
                line3 += &self.theme.line_horizontal;
            }
        }
        line3 += &self.theme.corner_bottom_right;
        println!("{}\r", line3);

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

    fn convet_mouse_to_index(
        &self,
        mouse_row: usize,
        mouse_column: usize,
    ) -> Option<(usize, usize)> {
        let row: usize;
        let column: usize;

        if mouse_row % 2 == 0 {
            return None;
        }
        row = ((mouse_row + 1) / 2) - 1;

        if self.theme.cell_horizontal_padding_enabled {
            if mouse_column % 4 == 0 {
                return None;
            }
            column = ((mouse_column + (4 - (mouse_column % 4))) / 4) - 1;
        } else {
            if mouse_column % 2 == 0 {
                return None;
            }
            column = ((mouse_column + 1) / 2) - 1;
        }

        if row >= self.size.0 {
            return None;
        }
        if column >= self.size.1 {
            return None;
        }

        Some((row, column))
    }

    fn discover_cell(&mut self, (row, column): (usize, usize)) {
        if !self.cells[row][column].is_discovered && !self.cells[row][column].is_flagged {
            self.cells[row][column].is_discovered = true;
            if self.cells[row][column].number_of_adjusted_bombs == 0 {
                for index in self.get_adjusted_indices((row, column)) {
                    self.discover_cell(index)
                }
            }
        }
    }

    fn set_cell_flag(&mut self, (row, column): (usize, usize), flag: bool) {
        if self.cells[row][column].is_flagged != flag { // need to change
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
            - number_of_flagged_adjusted_cells
            == number_of_unknown_adjusted_cells
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
}

#[cfg(test)]
mod tests {
    use crate::theme::default_theme;

    use super::*;

    #[test]
    fn convet_mouse_to_index() {
        let game_board = init_random_game((5, 10), 0.3, default_theme());

        assert_eq!(game_board.convet_mouse_to_index(0, 0), None);
        assert_eq!(game_board.convet_mouse_to_index(0, 1), None);
        assert_eq!(game_board.convet_mouse_to_index(0, 2), None);
        assert_eq!(game_board.convet_mouse_to_index(0, 3), None);
        assert_eq!(game_board.convet_mouse_to_index(0, 4), None);

        assert_eq!(game_board.convet_mouse_to_index(1, 0), None);
        assert_eq!(game_board.convet_mouse_to_index(1, 1), Some((0, 0)));
        assert_eq!(game_board.convet_mouse_to_index(1, 2), Some((0, 0)));
        assert_eq!(game_board.convet_mouse_to_index(1, 3), Some((0, 0)));
        assert_eq!(game_board.convet_mouse_to_index(1, 4), None);
        assert_eq!(game_board.convet_mouse_to_index(1, 5), Some((0, 1)));
        assert_eq!(game_board.convet_mouse_to_index(1, 6), Some((0, 1)));
        assert_eq!(game_board.convet_mouse_to_index(1, 7), Some((0, 1)));
        assert_eq!(game_board.convet_mouse_to_index(1, 8), None);
    }
}
