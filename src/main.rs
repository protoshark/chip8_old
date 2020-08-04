/*
    === CHIP-8 Specs ===
    memory: 4kB
    cpu clock: 1MHz

    display:      64x32 pixels
    font sprites: 04x05 pixels

    === Memory Map ===
    interpreter:    0x000-0x1FF [Leave empty]
    font:           0x050-0x0A0 [By convention. It can be anywhere between 000 and 1FF]
    rom+ram:        0x200-0xFFF
*/

use std::env;
use std::fs;
use std::{io::Read, path::Path};

// TODO: split modules to other files
mod chip8 {
    // Display info
    pub const DISPLAY_WIDTH: usize = 64;
    pub const DISPLAY_HEIGHT: usize = 32;

    // Memory info
    pub const RAM_SIZE: usize = 0x1000;
    pub const STACK_SIZE: usize = 0x10;
    pub const VRAM_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;

    mod font {
        pub const FONT_DATA: [u8; 5 * 16] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
    }

    mod cpu {
        // number of registers
        const NUM_REG: usize = 16;

        #[derive(Default, Debug)]
        pub struct Cpu {
        registers: [u8; NUM_REG],
    }
            pub(super) pc: u16,
            pub(super) i: u16,
            pub(super) registers: [u8; NUM_REG],
        }
// TODO: move to own file
mod chip8 {
    use super::cpu;

        impl Cpu {
            pub fn fetch<B: AsRef<Vec<u8>>>(&mut self, buffer: B) -> u8 {
                let buffer = buffer.as_ref();

                let opcode = buffer[self.pc as usize];
                self.pc += 2;

                opcode
            }

    }
        }
    }
    pub struct Chip8 {
        memory: [u8; MEM_SIZE],
        cpu: cpu::Cpu,
    }
}

fn main() {
    let mut args = env::args();
    let rom_file_name = args.nth(1).unwrap();

    let rom = read_file(rom_file_name);
}

fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path.as_ref()).unwrap();

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer
}
