use crate::cpu::cpu::CPU;

mod cpu;
mod memory;

fn main() {
    let boot_rom = load_boot_rom();
    let mut cpu = CPU::boot(boot_rom);
    loop {
        cpu.step();
    }
}

fn load_boot_rom() -> Vec<u8> {
    return std::fs::read("./rom/dmg_boot.bin").expect("Can't read file");
}
