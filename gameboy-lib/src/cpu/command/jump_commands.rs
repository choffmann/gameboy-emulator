use crate::cpu::{
    instructions::{FlagCondition, JumpInstruction},
    registers::Register,
    Cpu,
};

use super::Command;

pub struct JumpCommand<'a> {
    instruction: &'a JumpInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> JumpCommand<'a> {
    pub fn new(instruction: &'a JumpInstruction, cpu: &'a mut Cpu) -> JumpCommand<'a> {
        JumpCommand { instruction, cpu }
    }

    fn jp(&mut self) -> u16 {
        self.cpu.memory.read_16(self.cpu.pc + 1)
    }

    fn jp_cc(&mut self, condition: &FlagCondition) -> u16 {
        let address = self.cpu.memory.read_16(self.cpu.pc + 1);
        if self.cpu.resolve_flag_condition(condition) {
            address
        } else {
            self.cpu.pc.wrapping_add(3)
        }
    }

    fn jp_hl(&mut self) -> u16 {
        self.cpu.registers.get_16(&Register::HL)
    }

    fn jr(&mut self) -> u16 {
        let offset = self.cpu.memory.read(self.cpu.pc + 1) as u16;
        self.cpu.pc.wrapping_add(offset)
    }

    fn jr_cc(&mut self, condition: &FlagCondition) -> u16 {
        let offset = self.cpu.memory.read(self.cpu.pc + 1) as u16;
        if self.cpu.resolve_flag_condition(condition) {
            self.cpu.pc.wrapping_add(offset)
        } else {
            self.cpu.pc.wrapping_add(2)
        }
    }
}

impl Command for JumpCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            JumpInstruction::Jp => self.jp(),
            JumpInstruction::JpCond(condition) => self.jp_cc(condition),
            JumpInstruction::JpHL => self.jp_hl(),
            JumpInstruction::Jr => self.jr(),
            JumpInstruction::JrCond(condition) => self.jr_cc(condition),
        }
    }
}
