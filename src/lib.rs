struct Chip8 {
    memory: [u8; 1000],
    stack: [u16; 16], // Stored as 16 16 bit values
    
    // Registers the program can directly access and modify (Except for F)
    gen_purpose_registers: [u8; 16],
    register_i: u16, // Special 16 bit register

    delay_timer: u8,
    sound_timer: u8,

    
    program_counter: u16,
    stack_pointer: u8, 

    time: u8 // Will increase by 1 at a rate of 60 hz
}