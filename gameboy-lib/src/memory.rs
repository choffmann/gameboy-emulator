pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = ECHO_RAM_END - ECHO_RAM_BEGIN + 1;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const UNUSED_BEGIN: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;
pub const UNUSED_SIZE: usize = UNUSED_END - UNUSED_BEGIN + 1;

pub const IO_REGISTERS_BEGIN: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = IO_REGISTERS_END - IO_REGISTERS_BEGIN + 1;

pub const HIGH_RAM_BEGIN: usize = 0xFF80;
pub const HIGH_RAM_END: usize = 0xFFFE;
pub const HIGH_RAM_SIZE: usize = HIGH_RAM_END - HIGH_RAM_BEGIN + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

pub struct Memory {
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    vram: [u8; VRAM_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    echo_ram: [u8; ECHO_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    unused: [u8; UNUSED_SIZE],
    io_registers: [u8; IO_REGISTERS_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],
    interrupt_enable_register: u8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom_bank_0: [0; ROM_BANK_0_SIZE],
            rom_bank_n: [0; ROM_BANK_N_SIZE],
            vram: [0; VRAM_SIZE],
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            oam: [0; OAM_SIZE],
            unused: [0; UNUSED_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable_register: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        println!("[MEM] Reading from memory address: 0x{:X}", address);
        let address = address as usize;
        match address as usize {
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => self.rom_bank_0[address - ROM_BANK_0_BEGIN],
            ROM_BANK_N_BEGIN..=ROM_BANK_N_END => self.rom_bank_n[address - ROM_BANK_N_BEGIN],
            VRAM_BEGIN..=VRAM_END => self.vram[address - VRAM_BEGIN],
            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => self.external_ram[address - EXTERNAL_RAM_BEGIN],
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[address - WORKING_RAM_BEGIN],
            ECHO_RAM_BEGIN..=ECHO_RAM_END => self.echo_ram[address - ECHO_RAM_BEGIN],
            OAM_BEGIN..=OAM_END => self.oam[address - OAM_BEGIN],
            UNUSED_BEGIN..=UNUSED_END => self.unused[address - UNUSED_BEGIN],
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => self.io_registers[address - IO_REGISTERS_BEGIN],
            HIGH_RAM_BEGIN..=HIGH_RAM_END => self.high_ram[address - HIGH_RAM_BEGIN],
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable_register,
            _ => panic!("Invalid memory address: 0x{:X}", address),
        }
    }

    pub fn read_16(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write(&mut self, address: u16, value: u8) {
        println!("[MEM] Writing to memory address: 0x{:X} value: 0x{:X}", address, value);
        let address = address as usize;
        match address {
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => self.rom_bank_0[address - ROM_BANK_0_BEGIN] = value,
            ROM_BANK_N_BEGIN..=ROM_BANK_N_END => self.rom_bank_n[address - ROM_BANK_N_BEGIN] = value,
            VRAM_BEGIN..=VRAM_END => self.vram[address - VRAM_BEGIN] = value,
            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => self.external_ram[address - EXTERNAL_RAM_BEGIN] = value,
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[address - WORKING_RAM_BEGIN] = value,
            ECHO_RAM_BEGIN..=ECHO_RAM_END => self.echo_ram[address - ECHO_RAM_BEGIN] = value,
            OAM_BEGIN..=OAM_END => self.oam[address - OAM_BEGIN] = value,
            UNUSED_BEGIN..=UNUSED_END => self.unused[address - UNUSED_BEGIN] = value,
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => self.io_registers[address - IO_REGISTERS_BEGIN] = value,
            HIGH_RAM_BEGIN..=HIGH_RAM_END => self.high_ram[address - HIGH_RAM_BEGIN] = value,
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable_register = value,
            _ => panic!("Invalid memory address: 0x{:X}", address),
        }
    }

    pub fn write_vec(&mut self, start_address: u16, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write(start_address + i as u16, *byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let memory = Memory::new();
        assert_eq!(memory.read(0x0000), 0);
    }

    #[test]
    fn test_write_byte() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0x01);
        assert_eq!(memory.read(0x0000), 0x01);
    }

    #[test]
    fn test_read_write_boot_rom() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0x01);
        assert_eq!(memory.read(0x0000), 0x01);
    }

    #[test]
    fn test_read_write_rom_bank_0() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0x01);
        assert_eq!(memory.read(0x0000), 0x01);
    }

    #[test]
    fn test_read_write_rom_bank_n() {
        let mut memory = Memory::new();
        memory.write(0x4000, 0x01);
        assert_eq!(memory.read(0x4000), 0x01);
    }

    #[test]
    fn test_read_write_vram() {
        let mut memory = Memory::new();
        memory.write(0x8000, 0x01);
        assert_eq!(memory.read(0x8000), 0x01);
    }

    #[test]
    fn test_read_write_external_ram() {
        let mut memory = Memory::new();
        memory.write(0xA000, 0x01);
        assert_eq!(memory.read(0xA000), 0x01);
    }

    #[test]
    fn test_read_write_working_ram() {
        let mut memory = Memory::new();
        memory.write(0xC000, 0x01);
        assert_eq!(memory.read(0xC000), 0x01);
    }

    #[test]
    fn test_read_write_echo_ram() {
        let mut memory = Memory::new();
        memory.write(0xE000, 0x01);
        assert_eq!(memory.read(0xE000), 0x01);
    }

    #[test]
    fn test_read_write_oam() {
        let mut memory = Memory::new();
        memory.write(0xFE00, 0x01);
        assert_eq!(memory.read(0xFE00), 0x01);
    }

    #[test]
    fn test_read_write_unused() {
        let mut memory = Memory::new();
        memory.write(0xFEA0, 0x01);
        assert_eq!(memory.read(0xFEA0), 0x01);
    }

    #[test]
    fn test_read_write_io_registers() {
        let mut memory = Memory::new();
        memory.write(0xFF00, 0x01);
        assert_eq!(memory.read(0xFF00), 0x01);
    }

    #[test]
    fn test_read_write_high_ram() {
        let mut memory = Memory::new();
        memory.write(0xFF80, 0x01);
        assert_eq!(memory.read(0xFF80), 0x01);
    }

    #[test]
    fn test_read_write_interrupt_enable_register() {
        let mut memory = Memory::new();
        memory.write(0xFFFF, 0x01);
        assert_eq!(memory.read(0xFFFF), 0x01);
    }

    #[test]
    fn test_read_write_multiple() {
        let mut memory = Memory::new();
        memory.write(0x0000, 0x01);
        memory.write(0x4000, 0x02);
        memory.write(0x8000, 0x03);
        memory.write(0xA000, 0x04);
        memory.write(0xC000, 0x05);
        memory.write(0xE000, 0x06);
        memory.write(0xFE00, 0x07);
        memory.write(0xFEA0, 0x08);
        memory.write(0xFF00, 0x09);
        memory.write(0xFF80, 0x0A);
        memory.write(0xFFFF, 0x0B);
        assert_eq!(memory.read(0x0000), 0x01);
        assert_eq!(memory.read(0x4000), 0x02);
        assert_eq!(memory.read(0x8000), 0x03);
        assert_eq!(memory.read(0xA000), 0x04);
        assert_eq!(memory.read(0xC000), 0x05);
        assert_eq!(memory.read(0xE000), 0x06);
        assert_eq!(memory.read(0xFE00), 0x07);
        assert_eq!(memory.read(0xFEA0), 0x08);
        assert_eq!(memory.read(0xFF00), 0x09);
        assert_eq!(memory.read(0xFF80), 0x0A);
        assert_eq!(memory.read(0xFFFF), 0x0B);
    }
}
