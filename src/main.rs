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

mod chip8;

extern crate ncurses;

use std::env;
use std::fs;
use std::{io::Read, path::Path};

use chip8::game::Game;

fn main() {
    let mut args = env::args();
    let rom_file_name = args.nth(1).unwrap();

    let rom = read_file(rom_file_name);

    let mut game = Game::new();
    game.load_game(rom);
    game.run();
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path.as_ref()).unwrap();

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    file_buffer
}
