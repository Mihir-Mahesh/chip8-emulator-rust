use minifb;
use crate::keypad::Keypad;
use crate::display::Display;
use std::fs::File;
use std::io::Read;
use minifb::Key;

pub fn create_window(scale: usize) -> minifb::Window {
    minifb::Window::new("Chip-8 Emulator", 64 * scale, 32 * scale, minifb::WindowOptions::default()).unwrap()
}

pub fn log_keys(window: & minifb::Window, keypad: &mut Keypad) {
    let mut new_keys = [false; 16];

    new_keys[0x1] = window.is_key_down(Key::Key1);
    new_keys[0x2] = window.is_key_down(Key::Key2);
    new_keys[0x3] = window.is_key_down(Key::Key3);
    new_keys[0xC] = window.is_key_down(Key::Key4);

    new_keys[0x4] = window.is_key_down(Key::Q);
    new_keys[0x5] = window.is_key_down(Key::W);
    new_keys[0x6] = window.is_key_down(Key::E);
    new_keys[0xD] = window.is_key_down(Key::R);

    new_keys[0x7] = window.is_key_down(Key::A);
    new_keys[0x8] = window.is_key_down(Key::S);
    new_keys[0x9] = window.is_key_down(Key::D);
    new_keys[0xE] = window.is_key_down(Key::F);

    new_keys[0xA] = window.is_key_down(Key::Z);
    new_keys[0x0] = window.is_key_down(Key::X);
    new_keys[0xB] = window.is_key_down(Key::C);
    new_keys[0xF] = window.is_key_down(Key::V);

    keypad.change_keys(new_keys);

}

pub fn update_screen(window: &minifb::Window, display: &Display) {
    let mut buff: Vec<u32> = vec![0; 2048];
    let mut ind = 0;

    for i in 0..64 {
        for j in 0..32 {
            if display.gui[j][i]{
                buff[ind] = 0xFFFFFF
            }
            ind += 1;
        }
    }

    window.update_with_buffer(buff, 64, 32).unwrap();
}


pub fn load_rom(memory: &mut [u8; 4096], path: &str) {
    let mut file = File::open(path)?;
    let mut buff = Vec::new();

    file.read_to_end(&mut buff)?;

    let start = 0x200;
    let end = start + buff.len();

    memory[start..end].copy_from_slice(&buff);
}