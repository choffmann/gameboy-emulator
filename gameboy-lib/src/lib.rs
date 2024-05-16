pub mod cpu;
pub mod memory;

pub struct Gameboy {
    cpu: cpu::Cpu,
    boot_rom: Vec<u8>,
}

impl Gameboy {
    pub fn new(boot_rom: Vec<u8>) -> Gameboy {
        Gameboy {
            cpu: cpu::Cpu::new(),
            boot_rom,
        }
    }

    pub fn start(&mut self) {
        println!("Starting Gameboy");
        self.cpu.boot(self.boot_rom.clone());
        self.cpu.run();
    }
}

#[cfg(test)]
mod tests {

}
