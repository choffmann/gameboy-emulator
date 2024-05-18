use crate::cpu::{instructions::RotateInstruction, registers::Register, Cpu};

use super::Command;


pub struct RotateCommand<'a> {
    instruction: &'a RotateInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> RotateCommand<'a> {
    pub fn new(instruction: &'a RotateInstruction, cpu: &'a mut Cpu) -> RotateCommand<'a> {
        RotateCommand { instruction, cpu }
    }

    fn rlca(&mut self) -> u16 {
        let value = self.cpu.registers.a;
        let carry = value >> 7;
        let result = (value << 1) | carry;

        self.cpu.registers.a = result;
        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        self.cpu.pc.wrapping_add(1)
    }

    fn rla(&mut self) -> u16 {
        let value = self.cpu.registers.a;
        let carry = value >> 7;
        let result = (value << 1) | self.cpu.registers.f.carry as u8;

        self.cpu.registers.a = result;
        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        self.cpu.pc.wrapping_add(1)
    }

    fn rrca(&mut self) -> u16 {
        let value = self.cpu.registers.a;
        let carry = value & 1;
        let result = (value >> 1) | (carry << 7);

        self.cpu.registers.a = result;
        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        self.cpu.pc.wrapping_add(1)
    }

    fn rra(&mut self) -> u16 {
        let value = self.cpu.registers.a;
        let carry = value & 1;
        let result = (value >> 1) | ((self.cpu.registers.f.carry as u8) << 7);

        self.cpu.registers.a = result;
        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        self.cpu.pc.wrapping_add(1)
    }

    fn rlc(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value >> 7;
        let result = (value << 1) | carry;

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn rl(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value >> 7;
        let result = (value << 1) | self.cpu.registers.f.carry as u8;

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn rrc(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value & 1;
        let result = (value >> 1) | (carry << 7);

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn rr(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value & 1;
        let result = (value >> 1) | ((self.cpu.registers.f.carry as u8) << 7);

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn sla(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value >> 7;
        let result = value << 1;

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn sra(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value & 1;
        let result = (value >> 1) | (value & 0x80);

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }

    fn srl(&mut self, register: &Register) -> u16 {
        let (value, pc) = self.cpu.extract_operand(register);
        let carry = value & 1;
        let result = value >> 1;

        if let Register::HL = register {
            self.cpu.memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(register, result);
        }

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = false;
        self.cpu.registers.f.carry = carry == 1;

        return pc;
    }
}

impl Command for RotateCommand<'_> {
    fn execute(&mut self) -> u16 {
        match &self.instruction {
            RotateInstruction::RLCA => self.rlca(),
            RotateInstruction::RLA => self.rla(),
            RotateInstruction::RRCA => self.rrca(),
            RotateInstruction::RRA => self.rra(),
            RotateInstruction::RLC(register) => self.rlc(register),
            RotateInstruction::RL(register) => self.rl(register),
            RotateInstruction::RRC(register) => self.rrc(register),
            RotateInstruction::RR(register) => self.rr(register),
            RotateInstruction::SLA(register) => self.sla(register),
            RotateInstruction::SRA(register) => self.sra(register),
            RotateInstruction::SRL(register) => self.srl(register),
        }
    }
}
