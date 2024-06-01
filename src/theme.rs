use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

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
}

pub fn get_theme(theme_name: &String) -> Option<Theme> {
    match theme_name.as_str() {
        "colored" => Some(colored_theme()),
        "border" => Some(border_theme()),
        "dark_border" => Some(dark_border_theme()),
        "colored_borderless" => Some(colored_borderless_theme()),
        "borderless" => Some(borderless_theme()),
        _ => None
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
    }
}

pub fn dark_border_theme() -> Theme {
    let foreground: String = SetForegroundColor(Color::DarkGrey).to_string();
    let background: String = SetBackgroundColor(Color::Grey).to_string();
    let reset_color = ResetColor.to_string();

    let mut t = border_theme();
    t.name = "dark_border".to_owned();
    t.add_color_before_lines(&foreground);
    t.add_color_after_lines(&reset_color);

    t.bomb = format!(
        "{}{}{}",
        background, t.bomb, reset_color
    );
    t.flag = format!(
        "{}{}{}",
        background, t.flag, reset_color
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
    pub fn add_color_before_lines(&mut self, color: &String) {
        self.line_horizontal = format!("{}{}", color, self.line_horizontal);
        self.line_vertical = format!("{}{}", color, self.line_vertical);
        self.line_cross = format!("{}{}", color, self.line_cross);
        self.corner_top_left = format!("{}{}", color, self.corner_top_left);
        self.corner_top_right = format!("{}{}", color, self.corner_top_right);
        self.corner_bottom_left = format!("{}{}", color, self.corner_bottom_left);
        self.corner_bottom_right = format!("{}{}", color, self.corner_bottom_right);
        self.edge_top = format!("{}{}", color, self.edge_top);
        self.edge_bottom = format!("{}{}", color, self.edge_bottom);
        self.edge_left = format!("{}{}", color, self.edge_left);
        self.edge_right = format!("{}{}", color, self.edge_right);
    }

    pub fn add_color_after_lines(&mut self, color: &String) {
        self.line_horizontal = format!("{}{}", self.line_horizontal, color);
        self.line_vertical = format!("{}{}", self.line_vertical, color);
        self.line_cross = format!("{}{}", self.line_cross, color);
        self.corner_top_left = format!("{}{}", self.corner_top_left, color);
        self.corner_top_right = format!("{}{}", self.corner_top_right, color);
        self.corner_bottom_left = format!("{}{}", self.corner_bottom_left, color);
        self.corner_bottom_right = format!("{}{}", self.corner_bottom_right, color);
        self.edge_top = format!("{}{}", self.edge_top, color);
        self.edge_bottom = format!("{}{}", self.edge_bottom, color);
        self.edge_left = format!("{}{}", self.edge_left, color);
        self.edge_right = format!("{}{}", self.edge_right, color);
    }

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
                    _ => SetForegroundColor(Color::DarkMagenta).to_string()
                }), number_of_adjusted_bombs.to_string(), ResetColor.to_string()
            )
        } else {
            number_of_adjusted_bombs.to_string()
        }
    }
}
