pub struct Keypad {
    pub keys: [bool; 16],
    past_keys: [bool; 16]
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
            past_keys: [false; 16]
        }
    }

    pub fn keys_changed(&self) -> Option<usize> {
        for i in 0..16 {
            if self.keys[i] != self.past_keys[i]{
                return Some(i)
            }
        }
        None
    }

    pub fn change_keys(&mut self, new_keys: [bool; 16]) {
        self.past_keys = self.keys;
        self.keys = new_keys;
        if let Some(key_code) = self.keys_changed() {
        }
    }
}
