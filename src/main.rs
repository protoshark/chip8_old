/*
    === CHIP-8 Specs ===
    memory: 4kB
    cpu clock: 1MHz

    display: 64x32 pixels
    font sprites: 4x5 pixels

    === Memory Map ===
    interpreter:    0x000-0x1FF [Leave empty]
    font:           0x050-0x0A0 [By convention. It can be anywhere between 000 and 1FF]
    rom+ram:        0x200-0xFFF
*/

use std::env;
use std::fs;
use std::{io::Read, path::Path};

// TODO: move to own file
mod cpu {
    const NUM_REG: usize = 0xF;
    #[derive(Default)]
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

    const MEM_SIZE: usize = 0x1000;

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
