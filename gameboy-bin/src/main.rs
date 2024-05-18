extern crate gameboy_lib;

use std::{fs::{self, File}, io::Write};

use gameboy_lib::Gameboy;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let boot_rom = load_boot_rom();
    let _rom = load_rom("./roms/tetris.gb");
    let mut gameboy: Gameboy = Gameboy::new(boot_rom, vec![]);
    gameboy.start();

    let mem_dump = gameboy.dump_memory();
    let mut file = File::create("memory.bin").expect("Error while creating memory.bin");
    file.write_all(&mem_dump.as_slice()).expect("Error while writing memory.bin");
}

fn load_boot_rom() -> Vec<u8> {
    return fs::read("./boot_roms/dmg_boot.bin").expect("Error while reading boot rom");
}

fn load_rom(file: &str) -> Vec<u8> {
    return fs::read(file).expect("Error while reading rom");
}
