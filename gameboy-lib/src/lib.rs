use crate::cpu::cpu::CPU;
use crate::cpu::registers::Registers;
use crate::memory::memory::Memory;

pub mod cpu;
pub mod memory;

pub struct GameBoy {
    pub cpu: CPU,
    boot_rom: Vec<u8>,
    //game_rom: Vec<u8>
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy {
            cpu: CPU::default(),
            boot_rom: Vec::new(),
        }
    }

    pub fn start_gameboy(&mut self, boot_rom: Vec<u8>) {
        self.cpu.boot(boot_rom);
        loop {
            self.cpu.step();
        }
    }
}