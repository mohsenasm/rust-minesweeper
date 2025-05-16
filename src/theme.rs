use crossterm::style::{Color, ResetColor, SetForegroundColor};

#[derive(PartialEq)]
pub struct Theme {
    pub name: String,
    // all other string fileds should take at most one space
    pub cell_horizontal_padding_enabled: bool,
    pub cell_horizontal_padding: String,

    pub outer_border_enabled: bool,
    pub inner_border_row_enabled: bool,
    pub inner_border_column_enabled: bool,
    pub line_horizontal: String,
    pub line_vertical: String,
    pub line_cross: String,
    pub corner_top_left: String,
    pub corner_top_right: String,
    pub corner_bottom_left: String,
    pub corner_bottom_right: String,
    pub edge_top: String,
    pub edge_bottom: String,
    pub edge_left: String,
    pub edge_right: String,

    pub bomb: String,
    pub flag: String,
    pub empty: String,
    pub unknown: String,

    pub colored_numbers: bool,
    pub highlight_corner_on_selection: bool,

    pub line_color: Option<Color>,
}

pub fn get_theme(theme_name: &String) -> Option<Theme> {
    match theme_name.as_str() {
        "colored" => Some(colored_theme()),
        "border" => Some(border_theme()),
        "dark_border" => Some(dark_border_theme()),
        "colored_borderless" => Some(colored_borderless_theme()),
        "borderless" => Some(borderless_theme()),
        _ => None,
    }
}

pub fn rotate_theme_name(theme_name: &String) -> String {
    match theme_name.as_str() {
        "colored" => "border".to_owned(),
        "border" => "dark_border".to_owned(),
        "dark_border" => "borderless".to_owned(),
        "borderless" => "colored_borderless".to_owned(),
        _ => "colored".to_owned(),
    }
}

pub fn border_theme() -> Theme {
    Theme {
        name: "border".to_owned(),
        cell_horizontal_padding_enabled: true,
        cell_horizontal_padding: ' '.to_string(),

        outer_border_enabled: true,
        inner_border_row_enabled: true,
        inner_border_column_enabled: true,
        line_horizontal: '─'.to_string(),
        line_vertical: '│'.to_string(),
        line_cross: '┼'.to_string(),
        corner_top_left: '┌'.to_string(),
        corner_top_right: '┐'.to_string(),
        corner_bottom_left: '└'.to_string(),
        corner_bottom_right: '┘'.to_string(),
        edge_top: '┬'.to_string(),
        edge_bottom: '┴'.to_string(),
        edge_left: '├'.to_string(),
        edge_right: '┤'.to_string(),

        bomb: 'B'.to_string(),
        flag: 'F'.to_string(),
        empty: ' '.to_string(),
        unknown: '█'.to_string(),

        colored_numbers: false,
        highlight_corner_on_selection: false,
        line_color: None,
    }
}

pub fn borderless_theme() -> Theme {
    Theme {
        name: "borderless".to_owned(),
        cell_horizontal_padding_enabled: false,
        cell_horizontal_padding: "".to_string(),

        outer_border_enabled: false,
        inner_border_row_enabled: false,
        inner_border_column_enabled: true,
        line_horizontal: "".to_string(),
        line_vertical: ' '.to_string(),
        line_cross: "".to_string(),
        corner_top_left: "".to_string(),
        corner_top_right: "".to_string(),
        corner_bottom_left: "".to_string(),
        corner_bottom_right: "".to_string(),
        edge_top: "".to_string(),
        edge_bottom: "".to_string(),
        edge_left: "".to_string(),
        edge_right: "".to_string(),

        bomb: 'B'.to_string(),
        flag: 'F'.to_string(),
        empty: ' '.to_string(),
        unknown: '-'.to_string(),

        colored_numbers: false,
        highlight_corner_on_selection: false,
        line_color: None,
    }
}

pub fn dark_border_theme() -> Theme {
    let mut t = border_theme();
    t.name = "dark_border".to_owned();
    t.line_color = Some(Color::DarkGrey);
    t.flag = format!(
        "{}{}{}",
        crossterm::style::SetBackgroundColor(Color::DarkGrey).to_string(),
        t.flag,
        crossterm::style::ResetColor.to_string()
    );

    t
}

pub fn colored_theme() -> Theme {
    let mut t = dark_border_theme();
    t.name = "colored".to_owned();
    t.colored_numbers = true;

    t
}

pub fn colored_borderless_theme() -> Theme {
    let mut t = borderless_theme();
    t.name = "colored_borderless".to_owned();
    t.colored_numbers = true;

    t
}

impl Theme {
    pub fn format_number_of_adjusted_bombs(&self, number_of_adjusted_bombs: u8) -> String {
        if self.colored_numbers {
            format!(
                "{}{}{}",
                (match number_of_adjusted_bombs {
                    1 => SetForegroundColor(Color::Blue).to_string(),
                    2 => SetForegroundColor(Color::Green).to_string(),
                    3 => SetForegroundColor(Color::Red).to_string(),
                    4 => SetForegroundColor(Color::DarkBlue).to_string(),
                    5 => SetForegroundColor(Color::DarkRed).to_string(),
                    _ => SetForegroundColor(Color::DarkMagenta).to_string(),
                }),
                number_of_adjusted_bombs.to_string(),
                ResetColor.to_string()
            )
        } else {
            number_of_adjusted_bombs.to_string()
        }
    }

    // Returns a colored vertical border string, using yellow if selected, otherwise theme color
    pub fn format_vertical_border(&self, selected: bool) -> String {
        self.format_border(&self.line_vertical, selected)
    }

    pub fn format_horizontal_border(&self, selected: bool) -> String {
        self.format_border(&self.line_horizontal, selected)
    }

    pub fn format_cross(&self, selected: bool) -> String {
        self.format_border(
            &self.line_cross,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_corner_top_left(&self, selected: bool) -> String {
        self.format_border(
            &self.corner_top_left,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_corner_top_right(&self, selected: bool) -> String {
        self.format_border(
            &self.corner_top_right,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_corner_bottom_left(&self, selected: bool) -> String {
        self.format_border(
            &self.corner_bottom_left,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_corner_bottom_right(&self, selected: bool) -> String {
        self.format_border(
            &self.corner_bottom_right,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_edge_top(&self, selected: bool) -> String {
        self.format_border(&self.edge_top, self.highlight_corner_on_selection &&selected)
    }

    pub fn format_edge_bottom(&self, selected: bool) -> String {
        self.format_border(&self.edge_bottom, self.highlight_corner_on_selection &&selected)
    }

    pub fn format_edge_left(&self, selected: bool) -> String {
        self.format_border(&self.edge_left, self.highlight_corner_on_selection &&selected)
    }

    pub fn format_edge_right(&self, selected: bool) -> String {
        self.format_border(&self.edge_right, self.highlight_corner_on_selection &&selected)
    }

    fn format_border(&self, symbol: &str, selected: bool) -> String {
        if selected {
            format!(
                "{}{}{}",
                SetForegroundColor(Color::Yellow),
                symbol,
                ResetColor
            )
        } else if let Some(color) = self.line_color {
            format!("{}{}{}", SetForegroundColor(color), symbol, ResetColor)
        } else {
            symbol.to_string()
        }
    }

    // Returns a colored cell content string, using yellow if selected, otherwise normal
    pub fn format_cell_content(&self, content: &str, selected: bool) -> String {
        if selected {
            format!(
                "{}{}{}",
                SetForegroundColor(Color::Yellow),
                content,
                ResetColor
            )
        } else {
            content.to_string()
        }
    }
}
