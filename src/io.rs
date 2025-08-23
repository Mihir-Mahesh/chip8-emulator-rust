use minifb
mod keypad;
use keypad::Keypad;

pub fn create_window(scale: usize) -> minifb::Window {
    minifb::Window::new("Chip-8 Emulator", 64 * scale, 32 * scale, minifb::WindowOptions::default()).unwrap()
}

pub fn log_keys(window: & minifb::Window, keypad: &mut Keypad) {
    let mut keys = [false; 16]

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

    keypad.change_keys(keys);
}