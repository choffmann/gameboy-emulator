extern crate gameboy;

fn main() {
    let boot_rom = load_boot_rom();
    boot_rom.iter().for_each(|n|println!("{:x}", n));
    gameboy::start_gameboy(boot_rom);
}

fn load_boot_rom() -> Vec<u8> {
    return std::fs::read("/mnt/datahub/projects/emulation/gameboy/rom/dmg_boot.bin").expect("Error while reading boot rom")
}