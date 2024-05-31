pub struct Board {
    pub size: (u16, u16),
}

impl Board {
    pub fn init_random_game(&self) {
        println!("init_random_game \r")
    }

    pub fn mouse_hover(&self, row: u16, column: u16) {
        println!("mouse_hover: {} {} \r", row, column)
    }

    pub fn mouse_down(&self, row: u16, column: u16) {
        println!("mouse_down: {} {} \r", row, column)
    }
}
