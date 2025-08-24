use std::time::{Instant, Duration};
use std::thread;
use chip8_emulator_rust::Chip8;
use minifb;
use chip8_emulator_rust::io;

fn main() {
    let mut emulator: Chip8 = Chip8::new();

    let path: &str = "roms/Pong (1 player).ch8";
    emulator.load_fontset_and_roms(&path);

    let mut timer_update = Instant::now();
    while emulator.window.is_open() {
        if(emulator.update_timers(timer_update.elapsed().as_millis() as i32)) {
            timer_update = Instant::now();
        }
        if emulator.cpu.screen_dirty {
            io::update_screen(&mut emulator.window, &emulator.cpu.display);
            emulator.cpu.screen_dirty = false;
        }
        if(emulator.update_timers(timer_update.elapsed().as_millis() as i32)) {
            timer_update = Instant::now();
        }
        timer_update = run_instruction(&mut emulator, timer_update);
        thread::sleep(Duration::from_millis(2));
    }
}

fn run_instruction(emulator: &mut Chip8, timer_update: std::time::Instant) -> std::time::Instant{
    let mut curr_timer = timer_update;

    let curr_instr = ((emulator.cpu.memory[emulator.cpu.program_counter as usize] as u16) << 8) | (emulator.cpu.memory[emulator.cpu.program_counter as usize + 1] as u16);
    emulator.cpu.program_counter += 2;
    match curr_instr & 0xF000 {
        0x0000 => match curr_instr {
            0x00E0 => {emulator.cpu.cls();} //cls
            0x00EE => {emulator.cpu.ret();} //ret
            _ => {}
        }
        0x1000 => {
            emulator.cpu.jp(curr_instr & 0x0FFF); //jp
        }
        0x2000 => {
            emulator.cpu.call(curr_instr & 0x0FFF); //call
        }
        0x3000 => { //se
            let reg = ((curr_instr & 0x0FFF) >> 8) as usize;
            let byte = ((curr_instr & 0x00FF)) as u8;
            emulator.cpu.se(reg, byte);
        }
        0x4000 => {//sne
            let reg = ((curr_instr & 0x0F00) >> 8) as usize;
            let byte = ((curr_instr & 0x00FF)) as u8;
            emulator.cpu.sne(reg, byte);
        }
        0x5000 => {
            match curr_instr & 0x000F {
                0x0000 => {//se
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    let reg2 = ((curr_instr & 0x00F0) >> 4) as usize;
                    emulator.cpu.se_reg(reg1, reg2);
                }
                _ => {}
            }
        }
        0x6000 => {//ld
            let reg = ((curr_instr & 0x0F00) >> 8) as usize;
            let byte = ((curr_instr & 0x00FF)) as u8;
            emulator.cpu.ld(reg, byte);
        }
        0x7000 => {//add
            let reg = ((curr_instr & 0x0F00) >> 8) as usize;
            let byte = ((curr_instr & 0x00FF)) as u8;
            emulator.cpu.add(reg, byte);
        }
        0x8000 => {
            let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
            let reg2 = ((curr_instr & 0x00F0) >> 4) as usize;
            match curr_instr & 0x000F {
                0x0000 => {//ld
                    emulator.cpu.ld_reg(reg1, reg2);
                }
                0x0001 => {//or
                    emulator.cpu.or(reg1, reg2);
                }
                0x0002 => {//and
                    emulator.cpu.and(reg1, reg2);    
                }
                0x0003 => {//xor
                    emulator.cpu.xor(reg1, reg2);    
                }
                0x0004 => {//add
                    emulator.cpu.add_reg(reg1, reg2);    
                }
                0x0005 => {//sub
                    emulator.cpu.sub_reg(reg1, reg2);    
                }
                0x0006 => {//shr
                    emulator.cpu.shr(reg1, reg2);    
                }
                0x0007 => {//subn
                    emulator.cpu.sub_rev(reg1, reg2);    
                }
                0x000E => {//shl
                    emulator.cpu.shl(reg1, reg2);    
                }
                _ => {}
            }
        }

        0x9000 => {
            match curr_instr & 0x000F {
                0x0000 => {//sne
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    let reg2 = ((curr_instr & 0x00F0) >> 4) as usize;
                    emulator.cpu.sne_reg(reg1, reg2);
                }
                _ => {}
            }
        }

        0xA000 => {//ld i
            let byte = ((curr_instr & 0x0FFF)) as u16;
            emulator.cpu.ld_i(byte);
        }

        0xB000 => {//jp v0
            let byte = ((curr_instr & 0x0FFF)) as u16;
            emulator.cpu.jp_v0(byte);
        }

        0xC000 => { //rnd
            let reg = ((curr_instr & 0x0FFF) >> 8) as usize;
            let byte = ((curr_instr & 0x00FF)) as u8;
            emulator.cpu.rnd(reg, byte);
        }

        0xD000 => { //drw
            let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
            let reg2 = ((curr_instr & 0x00F0) >> 4) as usize;
            let n = ((curr_instr & 0x000F) as u8);
            emulator.cpu.drw(reg1, reg2, n);
        }

        0xE000 => {
            match curr_instr & 0x00FF {
                0x009E => {//skp
                    io::log_keys(&emulator.window, &mut emulator.cpu.keypad);
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.skp(reg1);
                }
                0x00A1 => {//sknp
                    io::log_keys(&emulator.window, &mut emulator.cpu.keypad);
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.sknp(reg1);
                }
                _ => {}
            }
        }

        0xF000 => {
            match curr_instr & 0x00FF {
                0x0007 => { //ld dt
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_delay_toreg(reg1);
                }
                0x000A => { //ld k
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    curr_timer = ld_key( emulator, reg1, curr_timer);
                }
                0x0015 => { //ld dt
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_delay(reg1);
                }
                0x0018 => { //ld sound
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_sound(reg1);
                    emulator.check_sound();
                }

                0x001E => { //add i
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.add_i(reg1);
                }
                0x0029 => { //ld sprite
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_sprite(reg1);
                }
                0x0033 => { //ld bcd
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_bcd(reg1);
                }
                0x0055 => { //ld reg to i
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_regtoI(reg1);
                }
                0x0065 => { //ld i to reg
                    let reg1 = ((curr_instr & 0x0F00) >> 8) as usize;
                    emulator.cpu.ld_Itoreg(reg1);
                }

                _ => {}
            }
        }

        _ => {}
    }
    return curr_timer;
}



pub fn ld_key(emulator: &mut Chip8, reg1: usize, timer_update: std::time::Instant) -> std::time::Instant{
    let mut curr_timer = timer_update;
    while true {
        if(emulator.update_timers(curr_timer.elapsed().as_millis() as i32)) {
            curr_timer = Instant::now();
        }
        if !emulator.window.is_open() {
            return curr_timer;
        }
        emulator.window.update();
        io::log_keys(&emulator.window, &mut emulator.cpu.keypad);

        
        if let Some(key_code) = emulator.cpu.keypad.keys_changed() {
            emulator.cpu.gen_purpose_registers[reg1] = key_code as u8;
            break; // Exit the loop once a key is found
        }
        thread::sleep(Duration::from_millis(2));
    }
    return curr_timer;
}