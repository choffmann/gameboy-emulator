use crate::cpu::{instructions::MiscInstruction, registers::Register, Cpu};

use super::Command;


pub struct MiscCommand<'a> {
    instruction: &'a MiscInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> MiscCommand<'a> {
    pub fn new(instruction: &'a MiscInstruction, cpu: &'a mut Cpu) -> MiscCommand<'a> {
        MiscCommand { instruction, cpu }
    }

    fn nop(&mut self) -> u16 {
        self.cpu.pc.wrapping_add(1)
    }

    fn swap(&mut self, from: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(from);
        let upper = value >> 4;
        let lower = value << 4;
        let result = upper | lower;

        if let Register::HL = from {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(from, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = false;

        return pc;
    }

    fn ccf(&mut self) -> u16 {
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = !self.cpu.registers.f.carry;

        self.cpu.pc.wrapping_add(1)
    }

    fn scf(&mut self) -> u16 {
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = true;

        self.cpu.pc.wrapping_add(1)
    }
}

impl Command for MiscCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            MiscInstruction::Nop => self.nop(),
            MiscInstruction::Swap(from) => self.swap(from),
            MiscInstruction::CCF => self.ccf(),
            MiscInstruction::SCF => self.scf(),
            _ => unimplemented!(),
        }
    }
}
