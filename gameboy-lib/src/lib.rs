pub mod cpu;
pub mod memory;

pub struct Gameboy {
    cpu: cpu::Cpu,
    boot_rom: Vec<u8>,
    game_rom: Vec<u8>,
}

impl Gameboy {
    pub fn new(boot_rom: Vec<u8>, game_rom: Vec<u8>) -> Gameboy {
        Gameboy {
            cpu: cpu::Cpu::new(),
            boot_rom,
            game_rom,
        }
    }

    pub fn start(&mut self) {
        println!("Starting Gameboy");
        self.cpu.boot(self.boot_rom.clone(), self.game_rom.clone());
        self.cpu.run();
    }

    pub fn dump_memory(&self) -> Vec<u8> {
        self.cpu.memory.dump()
        // let _ = File::create("memory.bin").unwrap();
        // fs::write("memory.bin", &buffer).unwrap();
    }
}

#[cfg(test)]
mod tests {

}
