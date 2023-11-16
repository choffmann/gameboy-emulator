use super::register::Register;

#[derive(Clone)]
pub struct Register8Bit {
    name: String,
    value: u8,
}

impl Register<u8> for Register8Bit {
    fn get(&self) -> u8 {
        println!("[REG {}] Reading value: 0x{:x}", self.name, self.value);
        return self.value;
    }

    fn set(&mut self, value: u8) {
        println!("[REG {}] Writing value: 0x{:x}", self.name, value);
        self.value = value;
    }
}

impl Register8Bit {
    pub fn new(name: &str) -> Self {
        return Register8Bit {
            name: String::from(name),
            value: 0x0,
        };
    }
}