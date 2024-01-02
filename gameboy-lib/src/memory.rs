use serde_derive::Serialize;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;


#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Memory {
    pub memory: Vec<u8>,
}


impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0; 0xFFFF],
        }
    }
}

impl Memory {
    pub fn boot(&mut self, boot_room: Vec<u8>) {
        for (idx, value) in boot_room.iter().enumerate() {
            self.write_byte(idx as u16, value.clone());
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // println!("[MEM] Writing memory: address: 0x{:x} value: 0x{:x}", address, value);
        self.memory[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let mem_value = self.memory[address as usize];
        // println!("[MEM] Reading memory: address: 0x{:x} value: 0x{:x}", address, mem_value);
        return mem_value;
    }

    pub fn read_next_word(&self, pc: u16) -> u16 {
        ((self.read_byte(pc + 2) as u16) << 8) | (self.read_byte(pc + 1) as u16)
    }
}
