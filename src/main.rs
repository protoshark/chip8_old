/*
    === CHIP-8 Specs ===
    memory: 4kB
    cpu clock: 1MHz

    display: 64x32 pixels
    font sprites: 4x5 pixels

    === Memory Map ===
    interpreter:    000-1FF (leave empty)
    font:           050-09F (by convention, but can be anywhere between 000 and 1FF)
*/

use std::env;
use std::fs;
use std::path::Path;
use std::io::Read;

// const MEM_SIZE: usize = 4_000;

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
