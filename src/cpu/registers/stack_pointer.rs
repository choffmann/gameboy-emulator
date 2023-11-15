use super::Register;

pub struct StackPointer {
    name: String,
    value: u16
}

impl Register<u16> for StackPointer {

    fn get(&self) -> u16 {
        println!("[REG {}] Reading value: 0x{:x}", self.name, self.value);
        return self.value;
    }

    fn set(&mut self, value: u16) {
        println!("[REG {}] Writing value: 0x{:x}", self.name, value);
        self.value = value;
    }
}

impl StackPointer {

    pub fn new(name: &str) -> Self {
        StackPointer {
            name: String::from(name),
            value: 0x0
        }
    }
}