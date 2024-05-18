extern crate gameboy_lib;

use gameboy_lib::Gameboy;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let boot_rom = load_boot_rom();
    let rom = load_rom("./roms/tetris.gb");
    let mut gameboy: Gameboy = Gameboy::new(boot_rom, rom);
    gameboy.start();
}

fn load_boot_rom() -> Vec<u8> {
    return std::fs::read("./boot_roms/dmg_boot.bin").expect("Error while reading boot rom");
}

fn load_rom(file: &str) -> Vec<u8> {
    return std::fs::read(file).expect("Error while reading rom");
}
