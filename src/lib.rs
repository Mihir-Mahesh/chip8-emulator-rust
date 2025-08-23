mod display;

use display::Display;

struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16], // Stored as 16 16 bit values
    
    // Registers the program can directly access and modify (Except for F)
    gen_purpose_registers: [u8; 16],
    register_i: u16, // Special 16 bit register

    delay_timer: u8,
    sound_timer: u8,

    
    program_counter: u16,
    stack_pointer: usize, 

    time: u8, // Will increase by 1 at a rate of 60 hz

    display: Display
}

impl Chip8 {

    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            stack: [0; 16],
            gen_purpose_registers: [0; 16],
            register_i: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            time: 0,
            display: Display::new()
        }
    }

    // Clears screen
    pub fn cls(&mut self) {
        self.display.clear();
    }

    // Returns from function
    pub fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    // Jumps to addr
    pub fn jp(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    // Calls function at addr
    pub fn call(&mut self, addr: u16) {
        self.stack[self.stack_pointer] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = addr;
    }

    // Skips next instruction if value at register == byte
    pub fn se(&mut self, reg: usize, byte: u8){
        if self.gen_purpose_registers[reg] == byte {
            self.program_counter += 2;
        } 
    }

    // Skips next instruction if value at register != byte
    pub fn sne(&mut self, reg: usize, byte: u8){
        if self.gen_purpose_registers[reg] != byte {
            self.program_counter += 2;
        } 
    }

    // Skips next instruction if value at reg1 == value at reg2
    pub fn se_reg(&mut self, reg1: usize, reg2: usize) {
        if self.gen_purpose_registers[reg1] == self.gen_purpose_registers[reg2] {
            self.program_counter += 2;
        } 
    }

    // Puts byte into reg1
    pub fn ld(&mut self, reg1: usize, byte: u8){
        self.gen_purpose_registers[reg1] = byte;
    }

    // Adds byte onto reg1
    pub fn add(&mut self, reg1: usize, byte: u8){
        self.gen_purpose_registers[reg1] += byte;
    }

    // Sets reg1 to reg2
    pub fn ld_reg(&mut self, reg1: usize, reg2: usize) {
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg2]
    }

    // Ors the value of reg2 to reg1
    pub fn or(&mut self, reg1: usize, reg2: usize) {
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg1] | self.gen_purpose_registers[reg2]
    }

    // Ands the value of reg2 to reg1
    pub fn and(&mut self, reg1: usize, reg2: usize) {
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg1] & self.gen_purpose_registers[reg2]
    }

    // Xors the value of reg2 to reg1
    pub fn xor(&mut self, reg1: usize, reg2: usize) {
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg1] ^ self.gen_purpose_registers[reg2]
    }

    // Adds reg2 to reg1 then sets Vf to 1 if reg1 + reg2 > 255
    pub fn add_reg(&mut self, reg1: usize, reg2: usize){
        let add1: u16 = self.gen_purpose_registers[reg1] as u16;
        let add2: u16 = self.gen_purpose_registers[reg2] as u16;

        if (add1 + add2 > 255){
            self.gen_purpose_registers[15] = 1;
            self.gen_purpose_registers[reg1] = ((add1 + add2) % 256) as u8;
        }
        else{
            self.gen_purpose_registers[15] = 0;
            self.gen_purpose_registers[reg1] = (add1 + add2) as u8;
        }
    }
}