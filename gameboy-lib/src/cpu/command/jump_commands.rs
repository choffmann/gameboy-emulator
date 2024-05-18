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
        if self.cpu.resolve_flag_condition(condition) {
            self.jp()
        } else {
            self.cpu.pc.wrapping_add(3)
        }
    }

    fn jp_hl(&mut self) -> u16 {
        self.cpu.registers.get_16(&Register::HL)
    }

    fn jr(&mut self) -> u16 {
        let offset = self.cpu.memory.read(self.cpu.pc + 1) as i8;
        let new_pc = self
            .cpu
            .pc
            .wrapping_add(2)
            .wrapping_add(offset as i16 as u16);
        println!("[CPU] Jump to address 0x{:x}", new_pc);
        new_pc
    }

    fn jr_cc(&mut self, condition: &FlagCondition) -> u16 {
        let next_step = self.cpu.pc.wrapping_add(2);
        if self.cpu.resolve_flag_condition(condition) {
            self.jr()
        } else {
            return next_step;
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
