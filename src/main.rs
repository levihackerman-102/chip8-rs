use std::{fs::File, io::Read};
use chip8::Chip8;

mod chip8;
mod ram;
fn main() {
    let mut rom = File::open("test_roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    let _ = rom.read_to_end(&mut data);

    // println!("Data: {:?}", data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
}
