use crate::cpu::{
    instructions::{FlagCondition, ReturnInstruction},
    Cpu,
};

use super::Command;

pub struct ReturnCommand<'a> {
    instruction: &'a ReturnInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> ReturnCommand<'a> {
    pub fn new(instruction: &'a ReturnInstruction, cpu: &'a mut Cpu) -> ReturnCommand<'a> {
        ReturnCommand { instruction, cpu }
    }

    fn rst(&mut self, address: &u8) -> u16 {
        let current_address = self.cpu.memory.read_16(self.cpu.pc);
        let next_sp = self.cpu.registers.sp.get().wrapping_sub(2);
        self.cpu.registers.sp.set(next_sp);
        self.cpu
            .memory
            .write_16(self.cpu.registers.sp.get(), current_address);

        0x0000 + (*address as u16)
    }

    fn ret(&mut self) -> u16 {
        let address = self.cpu.memory.read_16(self.cpu.registers.sp.get());
        self.cpu
            .registers
            .sp
            .set(self.cpu.registers.sp.get().wrapping_add(2));

        address
    }

    fn ret_conditional(&mut self, condition: &FlagCondition) -> u16 {
        if self.cpu.resolve_flag_condition(&condition) {
            let address = self.cpu.memory.read_16(self.cpu.registers.sp.get());
            self.cpu
                .registers
                .sp
                .set(self.cpu.registers.sp.get().wrapping_add(2));
            address
        } else {
            self.cpu.pc.wrapping_add(1)
        }
    }
}

impl Command for ReturnCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            ReturnInstruction::Rst(address) => self.rst(&address),
            ReturnInstruction::Ret => self.ret(),
            ReturnInstruction::RetCond(condition) => self.ret_conditional(condition),
            _ => unimplemented!(),
        }
    }
}
