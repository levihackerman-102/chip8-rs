use std::{fs::File, io::Read};

mod chip8;
mod cpu;
mod ram;
mod display;
mod keyboard;
mod bus;

use chip8::Chip8;

fn main() {
    let mut rom = File::open("test_roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    let _ = rom.read_to_end(&mut data);

    // println!("Data: {:?}", data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.run_instruction();
    }
}
