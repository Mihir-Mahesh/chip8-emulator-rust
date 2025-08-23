struct Display {
    gui: [[bool; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Display{
            gui: [[false; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        self.gui = [[false; 64]; 32];
    }
}