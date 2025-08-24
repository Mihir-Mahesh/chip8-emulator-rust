pub mod io;
pub mod cpu;
pub mod keypad;
pub mod display;

use cpu::CPU;

mod audio;
use audio::Audio;

use crate::io::load_rom;


pub struct Chip8 {
    pub cpu : CPU,
    pub window: minifb::Window,
    pub audio: Audio
}

impl Chip8 {

    pub fn new() -> Self {
        Chip8 {
            cpu : CPU::new(),
            window : io::create_window(25),
            audio : Audio::new()
        }
    }

     pub fn load_fontset_and_roms(&mut self, path: &str) {
        // Standard Chip-8 fontset (0-F, 4x5 pixels each)
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        // Most interpreters load fonts starting at 0x50 (80)
        let start = 0x50;
        self.cpu.memory[start..start + fontset.len()].copy_from_slice(&fontset);
        load_rom(&mut self.cpu.memory, path);
    }

    pub fn update_timers(&mut self, time: i32) -> bool{
        if (time < 16){
            return false;
        }
        if (self.cpu.sound_timer > 1){
            self.cpu.sound_timer -= 1;
        }
        else if (self.cpu.sound_timer == 1) {
            self.audio.stop();
            self.cpu.sound_timer -= 1;
        }

        if (self.cpu.delay_timer > 0) {
            self.cpu.delay_timer -= 1;
        }
        return true;
    }

    pub fn check_sound(&mut self) {
        if (self.cpu.sound_timer > 0 && !self.audio.is_playing) {
            self.audio.play();
        }
    }
}