use crate::cpu::cpu::CPU;

pub mod cpu;
mod memory;

pub fn start_gameboy(boot_rom: Vec<u8>) {
    let mut cpu = CPU::boot(boot_rom);
    loop {
        cpu.step();
    }
}