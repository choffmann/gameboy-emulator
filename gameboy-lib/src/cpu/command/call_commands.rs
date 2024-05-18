use crate::cpu::{instructions::{CallInstruction, FlagCondition}, Cpu};

use super::Command;


pub struct CallCommand<'a> {
    instruction: &'a CallInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> CallCommand<'a> {
    pub fn new(instruction: &'a CallInstruction, cpu: &'a mut Cpu) -> CallCommand<'a> {
        CallCommand { instruction, cpu }
    }

    fn call(&mut self) -> u16 {
        let address = self.cpu.memory.read_16(self.cpu.pc + 1);
        let next_pc = self.cpu.pc.wrapping_add(3);
        let next_sp = self.cpu.registers.sp.get().wrapping_sub(2);
        self.cpu.registers.sp.set(next_sp);
        self.cpu.memory.write_16(self.cpu.registers.sp.get(), next_pc);

        address
    }

    fn call_conditional(&mut self, condition: &FlagCondition) -> u16 {
        let address = self.cpu.memory.read_16(self.cpu.pc + 1);
        let next_pc = self.cpu.pc.wrapping_add(3);

        if self.cpu.resolve_flag_condition(&condition) {
            let next_sp = self.cpu.registers.sp.get().wrapping_sub(2);
            self.cpu.registers.sp.set(next_sp);
            self.cpu.memory.write_16(self.cpu.registers.sp.get(), next_pc);

            self.cpu.pc = address;
        }

        next_pc
    }
}

impl Command for CallCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            CallInstruction::Call => self.call(),
            CallInstruction::CallCond(condition) => self.call_conditional(condition),
        }
    }
}
