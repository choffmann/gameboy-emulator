extern crate gameboy;

use gameboy::GameBoy;
use gameboy::memory::observer::Event;


fn main() {
    let boot_rom = load_boot_rom();
    let mut gameboy: GameBoy = GameBoy::new();
    let mut memory = &mut gameboy.cpu.memory;

    memory.events().subscribe(Event::Write, |subject| {
        println!("[MEM] Writing value: 0x{:x}: 0x{:x}", subject.address, subject.value)
    });

    memory.events().subscribe(Event::Read, |subject| {
        println!("[MEM] Reading value: 0x{:x}: 0x{:x}", subject.address, subject.value)
    });

    gameboy.start_gameboy(boot_rom);
}

fn load_boot_rom() -> Vec<u8> {
    return std::fs::read("/mnt/datahub/projects/emulation/gameboy/rom/dmg_boot.bin").expect("Error while reading boot rom")
}