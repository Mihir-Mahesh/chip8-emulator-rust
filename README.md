# chip8-emulator-rust

A simple Chip-8 Emulator written in Rust
Supports loading and running most .ch8 rom files (note that some games, due to the varied requirements and behaviors of chip8, may not run on this one specifically.

## Features
- Fully implemented CHIP-8 instruction set  
- 64×32 monochrome display  
- Hexadecimal keypad (0–F)  
- 60Hz timers (delay + sound)  
- Beep sound when the sound timer is active  
- Cross-platform (Linux, macOS, Windows)

---

## Dependencies
This project uses the following Rust crates:
- `minifb` → windowing and display  
- `rodio` → audio output  
- `rand` → random numbers for `RND Vx, byte`

(These are already included in `Cargo.toml`.)

---
To run, load a file onto your computer, change the path string in main, then run cargo build and cargo run.
