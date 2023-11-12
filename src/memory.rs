pub struct Memory {
    pub memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
