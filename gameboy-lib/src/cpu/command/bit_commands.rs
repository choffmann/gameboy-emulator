use crate::cpu::{instructions::BitInstruction, registers::Register, Cpu};

use super::Command;

pub struct BitCommand<'a> {
    instruction: &'a BitInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> BitCommand<'a> {
    pub fn new(instruction: &'a BitInstruction, cpu: &'a mut Cpu) -> BitCommand<'a> {
        BitCommand { instruction, cpu }
    }

    fn bit(&mut self, bit: &u8, from: &Register) -> u16 {
        let value = self.cpu.registers.get(from);
        let result = value & (1 << bit);

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = true;

        self.cpu.pc.wrapping_add(2)
    }

    fn res(&mut self, bit: &u8, from: &Register) -> u16 {
        let value = self.cpu.registers.get(from);
        let result = value & !(1 << bit);

        self.cpu.registers.set(from, result);

        self.cpu.pc.wrapping_add(2)
    }

    fn set(&mut self, bit: &u8, from: &Register) -> u16 {
        let value = self.cpu.registers.get(from);
        let result = value | (1 << bit);

        self.cpu.registers.set(from, result);

        self.cpu.pc.wrapping_add(2)
    }
}

impl Command for BitCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            BitInstruction::Bit(bit, from) => self.bit(bit, from),
            BitInstruction::Res(bit, from) => self.res(bit, from),
            BitInstruction::Set(bit, from) => self.set(bit, from),
        }
    }
}
