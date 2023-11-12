pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;


pub struct Memory {
    pub memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF]
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
