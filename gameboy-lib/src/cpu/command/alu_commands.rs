use crate::cpu::{instructions::ArithmeticInstruction, registers::Register, Cpu, FlagUpdate};

use super::Command;

pub struct ArithmeticCommand<'a> {
    instruction: &'a ArithmeticInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> ArithmeticCommand<'a> {
    pub fn new(instruction: &'a ArithmeticInstruction, cpu: &'a mut Cpu) -> ArithmeticCommand<'a> {
        ArithmeticCommand { instruction, cpu }
    }

    fn alu_operation16<F>(&mut self, instruction: &ArithmeticInstruction, op: F) -> u16
    where
        F: Fn(u16, u16) -> (u16, Vec<FlagUpdate>),
    {
        let (value, pc) = match instruction {
            ArithmeticInstruction::Add16(from) => {
                let value = self.cpu.registers.get_16(&from);
                (value, self.cpu.pc.wrapping_add(1))
            }
            ArithmeticInstruction::Add16SP => {
                let n = self.cpu.memory.read(self.cpu.pc + 1) as u16;
                let sp = self.cpu.registers.get_16(&Register::SP);
                let result = sp.wrapping_add(n);
                (result as u16, self.cpu.pc.wrapping_add(2))
            }
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        };

        let hl = self.cpu.registers.get_16(&Register::HL);
        let (result, flag_update) = op(hl, value);

        if let ArithmeticInstruction::Add16SP = instruction {
            self.cpu.registers.set_16(&Register::SP, value)
        } else {
            self.cpu.registers.set_16(&Register::HL, result);
        }

        for flag in flag_update {
            self.cpu.update_flag(flag);
        }

        return pc;
    }

    fn alu_operation<F>(&mut self, instruction: &ArithmeticInstruction, op: F) -> u16
    where
        F: Fn(u8, u8) -> (u8, Vec<FlagUpdate>),
    {
        let (value, pc) = match instruction {
            ArithmeticInstruction::Cp(from)
            | ArithmeticInstruction::Xor(from)
            | ArithmeticInstruction::And(from)
            | ArithmeticInstruction::Or(from)
            | ArithmeticInstruction::Add(from)
            | ArithmeticInstruction::Sub(from) => self.cpu.extract_operand(&from),
            ArithmeticInstruction::Adc(from) | ArithmeticInstruction::Sbc(from) => {
                let (mut value, pc) = self.cpu.extract_operand(&from);
                if self.cpu.registers.f.carry {
                    value = value.wrapping_add(1);
                }
                (value, pc)
            }
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        };

        let a = self.cpu.registers.get(&Register::A);
        let (result, flag_update) = op(a, value);
        self.cpu.registers.set(&Register::A, result);

        for flag in flag_update {
            self.cpu.update_flag(flag);
        }

        return pc;
    }

    fn and(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let result = a & b;
            (
                result,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(false),
                    FlagUpdate::HalfCarry(true),
                    FlagUpdate::Carry(false),
                ],
            )
        })
    }

    fn or(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let result = a | b;
            (
                result,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(false),
                    FlagUpdate::HalfCarry(false),
                    FlagUpdate::Carry(false),
                ],
            )
        })
    }

    fn xor(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let result = a ^ b;
            (
                result,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(false),
                    FlagUpdate::HalfCarry(false),
                    FlagUpdate::Carry(false),
                ],
            )
        })
    }

    fn compare(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let result = a.wrapping_sub(b);
            (
                a,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(true),
                    FlagUpdate::HalfCarry((a & 0xF) < (b & 0xF)),
                    FlagUpdate::Carry(a < b),
                ],
            )
        })
    }

    fn inc(&mut self, register: &Register) -> u16 {
        let value = match &register {
            Register::HL => self
                .cpu
                .memory
                .read(self.cpu.registers.get_16(&Register::HL)),
            _ => self.cpu.registers.get(&register),
        };

        let result = value.wrapping_add(1);

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = false;
        self.cpu.registers.f.half_carry = (value & 0xF) == 0xF;

        if let Register::HL = register {
            self.cpu
                .memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(&register, result);
        }

        self.cpu.pc.wrapping_add(1)
    }

    fn inc16(&mut self, register: &Register) -> u16 {
        let value = self.cpu.registers.get_16(&register);
        let result = value.wrapping_add(1);
        self.cpu.registers.set_16(&register, result);

        self.cpu.pc.wrapping_add(1)
    }

    fn dec(&mut self, register: &Register) -> u16 {
        let value = match &register {
            Register::HL => self
                .cpu
                .memory
                .read(self.cpu.registers.get_16(&Register::HL)),
            _ => self.cpu.registers.get(&register),
        };

        let result = value.wrapping_sub(1);

        self.cpu.registers.f.zero = result == 0;
        self.cpu.registers.f.subtract = true;
        self.cpu.registers.f.half_carry = (value & 0xF) == 0x0;

        if let Register::HL = register {
            self.cpu
                .memory
                .write(self.cpu.registers.get_16(&Register::HL), result);
        } else {
            self.cpu.registers.set(&register, result);
        }

        self.cpu.pc.wrapping_add(1)
    }

    fn dec16(&mut self, register: &Register) -> u16 {
        let value = self.cpu.registers.get_16(&register);
        let result = value.wrapping_sub(1);
        self.cpu.registers.set_16(&register, result);

        self.cpu.pc.wrapping_add(1)
    }

    fn add(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let (result, did_overflow) = a.overflowing_add(b);
            (
                result,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(false),
                    FlagUpdate::HalfCarry(((a & 0x0F) + (b & 0x0F)) & 0x10 == 0x10),
                    FlagUpdate::Carry(did_overflow),
                ],
            )
        })
    }

    fn add16(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation16(&instruction, |a, b| {
            let (result, did_overflow) = a.overflowing_add(b);
            (
                result,
                vec![
                    FlagUpdate::Zero(false),
                    FlagUpdate::Subtract(false),
                    FlagUpdate::HalfCarry(((a & 0x0FFF) + (b & 0x0FFF)) & 0x1000 == 0x1000),
                    FlagUpdate::Carry(did_overflow),
                ],
            )
        })
    }

    fn sub(&mut self, instruction: &ArithmeticInstruction) -> u16 {
        self.alu_operation(instruction, |a, b| {
            let (result, did_overflow) = a.overflowing_sub(b);
            (
                result,
                vec![
                    FlagUpdate::Zero(result == 0),
                    FlagUpdate::Subtract(true),
                    FlagUpdate::HalfCarry((a & 0xF) < (b & 0xF)),
                    FlagUpdate::Carry(did_overflow),
                ],
            )
        })
    }
}

impl Command for ArithmeticCommand<'_> {
    fn execute(&mut self) -> u16 {
        let instruction = &self.instruction;
        match instruction {
            ArithmeticInstruction::Add(_) | ArithmeticInstruction::Adc(_) => self.add(instruction),
            ArithmeticInstruction::Add16(_) | ArithmeticInstruction::Add16SP => {
                self.add16(instruction)
            }
            ArithmeticInstruction::Sub(_) | ArithmeticInstruction::Sbc(_) => self.sub(instruction),
            ArithmeticInstruction::And(_) => self.and(instruction),
            ArithmeticInstruction::Or(_) => self.or(instruction),
            ArithmeticInstruction::Xor(_) => self.xor(instruction),
            ArithmeticInstruction::Cp(_) => self.compare(instruction),
            ArithmeticInstruction::Inc(register) => self.inc(&register),
            ArithmeticInstruction::Inc16(register) => self.inc16(&register),
            ArithmeticInstruction::Dec(register) => self.dec(&register),
            ArithmeticInstruction::Dec16(register) => self.dec16(&register),
        }
    }
}
