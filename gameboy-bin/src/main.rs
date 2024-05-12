extern crate gameboy_lib;

use gameboy_lib::Gameboy;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    let boot_rom = load_boot_rom();
    let mut gameboy: Gameboy = Gameboy::new(boot_rom);
    gameboy.start();
}

fn load_boot_rom() -> Vec<u8> {
    return std::fs::read("./boot_roms/dmg_boot.bin").expect("Error while reading boot rom");
}
