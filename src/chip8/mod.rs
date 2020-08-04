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
    ram: Vec<u8>,
    vram: Vec<u8>,
    stack: [u8; STACK_SIZE],
    cpu: cpu::Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut ram = vec![0; RAM_SIZE];

        // load font
        for (i, &byte) in font::FONT_DATA.iter().enumerate() {
            ram[i] = byte;
        }

        let vram = vec![0; VRAM_SIZE];
        let stack = [0; STACK_SIZE];

        Chip8 {
            ram,
            vram,
            stack,
            cpu: cpu::Cpu::default(),
        }
    }

    pub fn load_binary(&mut self, bin: Vec<u8>, offset: usize) {
        // insert binary at offset
        self.ram.splice(offset..offset, bin.iter().cloned());
    }

    pub fn execute(&mut self, opcode: u8) {
        cpu::Cpu::decode(opcode);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = match self.cpu.fetch(&self.ram) {
                Some(opcode) => opcode,
                None => panic!("The CHIP-8 got a buffer overflow"),
            };
            self.execute(opcode)
        }
    }
}
