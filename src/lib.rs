mod display;
use display::Display;

mod keypad;
use keypad::Keypad;

use rand::Rng;

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
    keypad: Keypad
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
            display: Display::new(),
            keypad: Keypad::new()
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
            self.gen_purpose_registers[16] = 1;
            self.gen_purpose_registers[reg1] = ((add1 + add2) % 256) as u8;
        }
        else{
            self.gen_purpose_registers[16] = 0;
            self.gen_purpose_registers[reg1] = (add1 + add2) as u8;
        }
    }

    // Subtracts reg2 from reg1 and sets the flag to 1 if reg2 > reg1 
    pub fn sub_reg(&mut self, reg1: usize, reg2: usize) {
        let sub1: u16 = self.gen_purpose_registers[reg1] as u16;
        let sub2: u16 = self.gen_purpose_registers[reg2] as u16;
        if (sub1 < sub2) {
            self.gen_purpose_registers[16] = 1;
            self.gen_purpose_registers[reg1] = ((sub1 - sub2) % 256) as u16;
        }
        else{
            self.gen_purpose_registers[16] = 0;
            self.gen_purpose_registers[reg1] = (sub1 - sub2) as u16;
        }
    }

    // Sets reg1 to reg2 shifted to the right by one. Sets Vf to 1 if reg2 is odd and zero if even
    pub fn shr(&mut self, reg1: usize, reg2: usize) {
        self.gen_purpose_registers[16] = self.gen_purpose_registers[reg2] % 2;
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg2] >> 1;
    }

    // Subtracts reg1 from reg2 and sets the flag to 1 if reg1 > reg2 
    pub fn sub_rev(&mut self, reg1: usize, reg2: usize) {
        let sub1: u16 = self.gen_purpose_registers[reg1] as u16;
        let sub2: u16 = self.gen_purpose_registers[reg2] as u16;
        if (sub1 > sub2) {
            self.gen_purpose_registers[16] = 1;
            self.gen_purpose_registers[reg1] = ((sub2 - sub1) % 256) as u16;
        }
        else{
            self.gen_purpose_registers[16] = 0;
            self.gen_purpose_registers[reg1] = (sub2 - sub1) as u16;
        }
    }

    // Sets reg1 to reg2 shifted to the left by one. Sets Vf to 1 if reg2 is most sig bit is 1 and zero if otherwise
    pub fn shr(&mut self, reg1: usize, reg2: usize) {
        if (self.gen_purpose_registers[reg2] >= 128){
            self.gen_purpose_registers[16] = 1;
        }
        else{
            self.gen_purpose_registers[16] = 0;
        }
        self.gen_purpose_registers[reg1] = self.gen_purpose_registers[reg2] << 1;
    }


    // Skips next instruction if value at reg1 != value at reg2
    pub fn sne_reg(&mut self, reg1: usize, reg2: usize) {
        if self.gen_purpose_registers[reg1] != self.gen_purpose_registers[reg2] {
            self.program_counter += 2;
        } 
    }

    // Sets I to the addr value
    pub fn ld_i(&mut self, addr: u16){
        self.register_i = addr;
    }


    // Jumps to address at V0 + addr
    pub fn jp_v0(&mut self, addr: u16) {
        self.program_counter = addr + (self.gen_purpose_registers[0] as u16);
    }


    // Sets reg1 to a random byte and Ands it with byte
    pub fn rnd(&mut self, reg1: usize, byte) {
        let mut rng = rand::thread_rng();
        let random_u8: u8 = rng.gen_range(0..=255);

        self.gen_purpose_registers[reg1] = random_u8 & byte;
    }

    // Display n-byte sprite starting at memory location I at (reg1, reg2), set VF = 1 if a pixel is erased and zero otherwise
    pub fn drw(&mut self, reg1: usize, reg2: usize, num_bytes: u8) {
        let flag: u8 = display.draw(self.gen_purpose_registers[reg1], self.gen_purpose_registers[reg2], self.register_i, num_bytes, &self.memory);
        self.gen_purpose_registers[16] = flag;
    }

    // Checks if a key corresponding to reg1 is pressed and if so skips the next instruction
    pub fn skp(&mut self, reg1: usize) {
        if (self.keypad[(self.gen_purpose_registers[reg1] as usize)]) {
            self.program_counter += 2;
        }
    }

    // Checks if a key corresponding to reg1 is pressed and if not skips the next instruction
    pub fn sknp(&mut self, reg1: usize) {
        if (!self.keypad[(self.gen_purpose_registers[reg1] as usize)]) {
            self.program_counter += 2;
        }
    }

    // Moves most recent key press into reg1 (waits until then)
    pub fn ld_key(&mut self, reg1: usize){

    }

    // Moves delay timer value into reg1
    pub fn ld_delay(&mut self, reg1: usize){
        self.gen_purpose_registers[reg1] = self.delay_timer;
    }

    // Moves sound timer value into reg1
    pub fn ld_sound(&mut self, reg1: usize){
        self.gen_purpose_registers[reg1] = self.sound_timer;
    }

    pub fn add_i(&mut self, reg1) {
        self.register_i += self.gen_purpose_registers[reg1] as u16;
    }

    pub fn 
}