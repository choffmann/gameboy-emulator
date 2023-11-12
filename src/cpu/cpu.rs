use crate::memory::Memory;

use super::instructions::ArithmeticTarget;
use super::instructions::Instruction;
use super::registers::Registers;

pub struct CPU {
    pub register: Registers,
    pub pc: u16,
    pub memory: Memory,
}

impl CPU {
    pub fn execute(&mut self, instructions: Instruction) {
        match instructions {
            Instruction::ADD(target) => self.match_add(target, false),
            Instruction::ADC(target) => self.match_add(target, true),
            Instruction::SUB(target) => self.match_sub(target, false),
            Instruction::SBC(target) => self.match_sub(target, true),
            Instruction::INC(target) => self.match_inc(target),
            _ => panic!("Not implemented instructions {:?}", instructions),
        }
    }

    fn match_add(&mut self, target: ArithmeticTarget, with_carry: bool) {
        match target {
            ArithmeticTarget::B => {
                let value = self.register.b;
                let new_value = self.add(value, with_carry);
                self.register.a = new_value;
            }
            _ => println!(
                "Missing regsiter on ADD Instruction, target register: {:?}",
                target
            ),
        };
    }

    fn match_inc(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::B => {
                let value = self.register.b;
                self.register.b = self.inc(value);
            }
            _ => println!(
                "Missing regsiter on INC Instruction, target register: {:?}",
                target
            ),
        }
    }

    fn match_sub(&mut self, target: ArithmeticTarget, with_carry: bool) {
        match target {
            ArithmeticTarget::B => {
                let value = self.register.b;
                let new_value = self.sub(value, with_carry);
                self.register.a = new_value;
            }
            _ => println!(
                "Missing regsiter on SUB Instruction, target register: {:?}",
                target
            ),
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.memory.read_byte(self.pc);
    }

    fn add(&mut self, value: u8, add_carry: bool) -> u8 {
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (add, frist_did_overflow) = self.register.a.overflowing_add(value);
        let (new_value, result_did_overflow) = add.overflowing_add(carry_value);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.carry = frist_did_overflow || result_did_overflow;
        self.register.f.half_carry = ((self.register.a & 0xf) + (value & 0xf) + carry_value) > 0xf;
        return new_value;
    }

    fn sub(&mut self, value: u8, add_carry: bool) -> u8 {
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (sub, firt_did_overflow) = self.register.a.overflowing_sub(value);
        let (new_value, result_did_overflow) = sub.overflowing_sub(sub);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = true;
        self.register.f.carry = firt_did_overflow || result_did_overflow;
        self.register.f.half_carry = (self.register.a & 0xf) < (value & 0xf) + carry_value;
        return new_value;
    }

    fn inc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_add(1);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.half_carry = value & 0xf == 0xf;
        return new_value;
    }

    fn get_opt_carry_flag(&self, add_carry: bool) -> u8 {
        if add_carry && self.register.f.carry {
            1
        } else {
            0
        }
    }
}
