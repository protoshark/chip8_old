pub mod cpu;
mod font;

// Display info
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

// Memory info
pub const RAM_SIZE: usize = 0x1000;
pub const STACK_SIZE: usize = 0x10;
pub const VRAM_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

#[derive(Debug)]
pub struct Chip8 {
    cpu: cpu::Cpu,
    stack: Vec<u16>,
    ram: Vec<u8>,
    vram: Vec<u8>,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let stack = vec![0; STACK_SIZE];
        let vram = vec![0; VRAM_SIZE];
        let mut ram = vec![0; RAM_SIZE];

        // load font
        for (i, &byte) in font::FONT_SET.iter().enumerate() {
            ram[i] = byte;
        }

        Chip8 {
            cpu: cpu::Cpu::default(),
            stack,
            ram,
            vram,
        }
    }

    pub fn load_binary(&mut self, bin: Vec<u8>, offset: usize) {
        // insert binary at offset
        self.ram.splice(offset..offset, bin.iter().cloned());
    }

    pub fn execute(&mut self, word: u16) {
        let op = word >> 0xC & 0xF;

        let x: usize = (word >> 0x8 & 0x0F00) as usize;
        let y: usize = (word >> 0x4 & 0x00F0) as usize;
        let nn = (word & 0x00FF) as u8;

        // TODO: move instruction to their own function?
        match op {
            0x0 => {
                // 00NN
                match nn {
                    0xE0 => {
                        // 00E0 (CLS)
                        self.vram.iter_mut().for_each(|v| *v = 0);
                    }
                    0xEE => {
                        // 00EE (RET)
                        self.cpu.pc = self.stack.pop().unwrap();
                    }
                    _ => panic!("unknown opcode 0x00{:02x}", nn),
                }
            }
            0x1 => {
                // 1NNN (JP NNN)
                let nnn = word & 0x0FFF;
                self.cpu.pc = nnn;
            }
            0x2 => {
                // 2NNN (CALL NNN)
                let nnn = word & 0xFFF;
                self.stack.push(self.cpu.pc);

                self.cpu.pc = nnn;
            }
            0x3 | 0x4 | 0x5 | 0x9 => {
                if
                    // 3XNN (SE VX,NN)
                           (op == 0x3 && self.cpu.registers[x] == nn) 
                    // 4XNN (SNE VX,NN)
                        || (op == 0x4 && self.cpu.registers[x] != nn) 
                    // 5XY0 (SE VX,VY)
                        || (op == 0x5 && self.cpu.registers[x] == self.cpu.registers[y])
                    // 9XY0 (SNE VX,VY)
                        || (op == 0x9 && self.cpu.registers[x] != self.cpu.registers[y])
                {
                    self.cpu.pc += 2;
                }
            }
            0x6 => {
                // 6XNN (LD VX,NN)
                self.cpu.registers[x] = nn;
            }
            0x7 => {
                // 7XNN (ADD VX,NN)
                let result = self.cpu.registers[x].wrapping_add(nn);
                self.cpu.registers[x] = result;
            }
            // Logical and arithmetic group
            //
            0xA => {
                // ANNN (LD I,NNN)
                let nnn = word & 0xFFF;
                self.cpu.i = nnn;
            }
            0xD => {
                // DXYN (DRW VX,VY,N)
                let coords = (
                    self.cpu.registers[x] as usize,
                    self.cpu.registers[y] as usize
                );
                let height = (word & 0xF) as usize;
                
                self.cpu.registers[0xF] = 0;
                for i in 0..height {
                    let pix = self.ram[self.cpu.i as usize + i];
                    for j in 0..8 {
                        if (pix & (0x80 >> j)) != 0 {
                            let offset = (coords.1 + i) * DISPLAY_WIDTH + (coords.0 + j);

                            if offset > VRAM_SIZE { break }

                            if self.vram[offset] == 1 {
                                self.cpu.registers[0xF] = 1;
                            }
                            self.vram[offset] ^= 1;
                        }
                    }
                }
            }
            _ => panic!("unknown opcode 0x{:04x}", word),
        }
    }

    pub fn run(&mut self) {
        loop {
            let word = match self.cpu.fetch(&self.ram) {
                Some(word) => word,
                None => panic!("CHIP-8 got a buffer overflow"),
            };
            self.execute(word);
        }
    }
}
