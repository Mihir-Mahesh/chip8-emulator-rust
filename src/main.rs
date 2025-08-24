use std::time::{Instant, Duration};
use std::thread;
mod lib;
use lib::Chip8;
mod io;
use minifb

fn main() {
    let mut emulator: Chip8 = Chip8::new();

    path = "../roms/Keypad Test [Hap, 2006].ch8"
    emulator.load_fontset_and_roms(path);

    let mut timer_update = Instant::now();

    while emulator.window.is_open() {
        update_screen(&emulator.window, &emulator.Chip8.cpu.memory);
    }
}



pub fn ld_key(&mut self, reg1: usize, window: &minifb::Window){
        while true {
            log_keys(window, &mut self.keypad);

            let new_key = self.keypad.keys_changed();
            let mut something = false;
            match new_key {
                None => continue,
                Some(k) => something = true
            }
            if (something) {
                self.gen_purpose_registers[reg1] = new_key.unwrap();
                break;
            }
        }
    }