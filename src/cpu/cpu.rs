use super::instructions::Instruction;
use super::instructions::RegisterTarget;
use super::registers::Registers;

pub struct CPU {
    pub register: Registers,
}

impl CPU {
    pub fn execute(&mut self, instructions: Instruction) {
        match instructions {
            Instruction::ADD(target) => match target {
                RegisterTarget::B => {
                    let value = self.register.b;
                    let new_value = self.add(value);
                    self.register.a = new_value;
                }
                _ => todo!("Missing regsiter on ADD Instruction"),
            },
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.register.a.overflowing_add(value);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.carry = did_overflow;
        self.register.f.half_carry = (self.register.a & 0xf) + (value & 0xf) > 0xf;
        return new_value;
    }
}
