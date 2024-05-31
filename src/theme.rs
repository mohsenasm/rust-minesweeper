use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

pub struct Theme {
    pub cell_horizontal_padding_enabled: bool,
    pub cell_horizontal_padding: String,

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
}

pub fn default_theme() -> Theme {
    let black_foreground = SetForegroundColor(Color::Black).to_string();
    let grey_background = SetBackgroundColor(Color::DarkGrey).to_string();
    let reset_color = ResetColor.to_string();

    Theme {
        cell_horizontal_padding_enabled: true,
        cell_horizontal_padding: ' '.to_string(),

        line_horizontal: format!("{}{}{}", black_foreground, '─'.to_string(), reset_color),
        line_vertical: format!("{}{}{}", black_foreground, '│'.to_string(), reset_color),
        line_cross: format!("{}{}{}", black_foreground, '┼'.to_string(), reset_color),
        corner_top_left: format!("{}{}{}", black_foreground, '┌'.to_string(), reset_color),
        corner_top_right: format!("{}{}{}", black_foreground, '┐'.to_string(), reset_color),
        corner_bottom_left: format!("{}{}{}", black_foreground, '└'.to_string(), reset_color),
        corner_bottom_right: format!("{}{}{}", black_foreground, '┘'.to_string(), reset_color),
        edge_top: format!("{}{}{}", black_foreground, '┬'.to_string(), reset_color),
        edge_bottom: format!("{}{}{}", black_foreground, '┴'.to_string(), reset_color),
        edge_left: format!("{}{}{}", black_foreground, '├'.to_string(), reset_color),
        edge_right: format!("{}{}{}", black_foreground, '┤'.to_string(), reset_color),

        bomb: format!("{}{}{}{}", black_foreground, grey_background, 'B'.to_string(), reset_color),
        flag: format!("{}{}{}{}", black_foreground, grey_background, 'F'.to_string(), reset_color),
        empty: ' '.to_string(),
        unknown: '█'.to_string(),
    }
}
