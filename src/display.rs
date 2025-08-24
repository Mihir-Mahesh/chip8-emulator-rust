pub struct Display {
    pub gui: [[bool; 64]; 32]
}

impl Display {
    pub fn new() -> Self {
        Display{
            gui: [[false; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        self.gui = [[false; 64]; 32];
    }

    pub fn draw(&mut self, coord1: u8, coord2: u8, mem_loc: u16, num_bytes: u8, memory: &[u8; 4096]) -> u8{
        let x: usize = coord1 as usize;
        let y: usize = coord2 as usize;
        let mem: usize = mem_loc as usize;

        let mut flag: u8 = 0;

        for i in 0..num_bytes {
            let curr_line: usize = i as usize;
            let curr_byte: u8 = memory[mem + curr_line];
            
            let mut j: u8 = 128;
            let mut ind: usize = 0;
            while j > 0{
                let bit: u8 = curr_byte & j;
                if bit != 0 {
                    let x_coord: usize = (x + ind) % 64;
                    let y_coord: usize = (y + curr_line) % 32;
                    if self.gui[x_coord][y_coord]{
                        flag = 1;
                        self.gui[x_coord][y_coord] = false;
                    }
                    else{
                        self.gui[x_coord][y_coord] = true;
                    }
                }

                j = j >> 1;
                ind += 1;
            }
        }

        flag
    }
}