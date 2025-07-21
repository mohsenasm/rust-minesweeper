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

    pub number_colors: Option<[Color; 6]>,
    pub colored_numbers_on_selection: bool,
    pub highlight_corner_on_selection: bool,

    pub line_color: Option<Color>,
}



pub fn get_theme(theme_name: &String) -> Option<Theme> {
    match theme_name.as_str() {
        "border" => Some(border_theme()),
        "dark_border" => Some(dark_border_theme()),
        "borderless" => Some(borderless_theme()),
        _ => None,
    }
}

pub fn rotate_theme_name(theme_name: &String) -> String {
    match theme_name.as_str() {
        "dark_border" => "borderless".to_owned(),
        "borderless" => "border".to_owned(),
        _ => "dark_border".to_owned(),
    }
}

pub fn rotate_theme_color(theme_color: &Option<[Color; 6]>) -> Option<[Color; 6]> {
    match theme_color {
        Some(THEME_COLOR_LIST_1) => Some(THEME_COLOR_LIST_2),
        Some(THEME_COLOR_LIST_2) => None,
        _ => Some(THEME_COLOR_LIST_1),
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

        number_colors: Some(THEME_COLOR_LIST_1),
        colored_numbers_on_selection: true,
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

        number_colors: Some(THEME_COLOR_LIST_1),
        colored_numbers_on_selection: false,
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

const THEME_COLOR_LIST_1: [Color; 6] = [
    Color::Blue,
    Color::Green,
    Color::Red,
    Color::DarkBlue,
    Color::DarkRed,
    Color::DarkMagenta,
];

const THEME_COLOR_LIST_2: [Color; 6] = [
    Color::Rgb { r: 105, g: 201, b: 250 },
    Color::Rgb { r: 120, g: 218, b: 116 },
    Color::Rgb { r: 238, g: 127, b: 110 },
    Color::Rgb { r: 111, g: 191, b: 228 },
    Color::Rgb { r: 235, g: 129, b: 114 },
    Color::Rgb { r: 207, g: 152, b: 198 },
];

impl Theme {
    pub fn format_number_of_adjusted_bombs(
        &self,
        number_of_adjusted_bombs: u8,
        selected: bool,
    ) -> String {
        let use_color =
            (self.number_colors != None) && (!selected || self.colored_numbers_on_selection);

        if use_color {
            let number_colors = self.number_colors.unwrap();
            format!(
                "{}{}{}",
                (match number_of_adjusted_bombs {
                    1 => SetForegroundColor(number_colors[0]).to_string(),
                    2 => SetForegroundColor(number_colors[1]).to_string(),
                    3 => SetForegroundColor(number_colors[2]).to_string(),
                    4 => SetForegroundColor(number_colors[3]).to_string(),
                    5 => SetForegroundColor(number_colors[4]).to_string(),
                    _ => SetForegroundColor(number_colors[5]).to_string(),
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
        self.format_border(
            &self.edge_top,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_edge_bottom(&self, selected: bool) -> String {
        self.format_border(
            &self.edge_bottom,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_edge_left(&self, selected: bool) -> String {
        self.format_border(
            &self.edge_left,
            self.highlight_corner_on_selection && selected,
        )
    }

    pub fn format_edge_right(&self, selected: bool) -> String {
        self.format_border(
            &self.edge_right,
            self.highlight_corner_on_selection && selected,
        )
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
