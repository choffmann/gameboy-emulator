use crate::memory::Memory;

use super::instructions::Logic8BitRegister;
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
            Instruction::DEC(target) => self.match_dec(target),
            Instruction::AND(target) => self.match_and(target),
            Instruction::OR(target) => self.match_or(target),
            Instruction::XOR(target) => self.match_xor(target),
            Instruction::CP(target) => self.match_cp(target),
        }
    }

    fn match_add(&mut self, target: Logic8BitRegister, with_carry: bool) {
        match target {
            Logic8BitRegister::A => self.exec_add(self.register.a, with_carry),
            Logic8BitRegister::B => self.exec_add(self.register.b, with_carry),
            Logic8BitRegister::C => self.exec_add(self.register.c, with_carry),
            Logic8BitRegister::D => self.exec_add(self.register.d, with_carry),
            Logic8BitRegister::E => self.exec_add(self.register.e, with_carry),
            Logic8BitRegister::H => self.exec_add(self.register.h, with_carry),
            Logic8BitRegister::L => self.exec_add(self.register.l, with_carry),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented"),
        }
    }

    fn match_inc(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.register.a = self.inc(self.register.a),
            Logic8BitRegister::B => self.register.b = self.inc(self.register.b),
            Logic8BitRegister::C => self.register.c = self.inc(self.register.c),
            Logic8BitRegister::D => self.register.d = self.inc(self.register.d),
            Logic8BitRegister::E => self.register.e = self.inc(self.register.e),
            Logic8BitRegister::H => self.register.h = self.inc(self.register.h),
            Logic8BitRegister::L => self.register.l = self.inc(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_dec(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.register.a = self.dec(self.register.a),
            Logic8BitRegister::B => self.register.b = self.dec(self.register.b),
            Logic8BitRegister::C => self.register.c = self.dec(self.register.c),
            Logic8BitRegister::D => self.register.d = self.dec(self.register.d),
            Logic8BitRegister::E => self.register.e = self.dec(self.register.e),
            Logic8BitRegister::H => self.register.h = self.dec(self.register.h),
            Logic8BitRegister::L => self.register.l = self.dec(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_sub(&mut self, target: Logic8BitRegister, with_carry: bool) {
        match target {
            Logic8BitRegister::A => self.exec_sub(self.register.a, with_carry),
            Logic8BitRegister::B => self.exec_sub(self.register.b, with_carry),
            Logic8BitRegister::C => self.exec_sub(self.register.c, with_carry),
            Logic8BitRegister::D => self.exec_sub(self.register.d, with_carry),
            Logic8BitRegister::E => self.exec_sub(self.register.e, with_carry),
            Logic8BitRegister::H => self.exec_sub(self.register.h, with_carry),
            Logic8BitRegister::L => self.exec_sub(self.register.l, with_carry),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_and(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.register.a = self.and(self.register.a),
            Logic8BitRegister::B => self.register.b = self.and(self.register.b),
            Logic8BitRegister::C => self.register.c = self.and(self.register.c),
            Logic8BitRegister::D => self.register.d = self.and(self.register.d),
            Logic8BitRegister::E => self.register.e = self.and(self.register.e),
            Logic8BitRegister::H => self.register.h = self.and(self.register.h),
            Logic8BitRegister::L => self.register.l = self.and(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_or(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.register.a = self.or(self.register.a),
            Logic8BitRegister::B => self.register.b = self.or(self.register.b),
            Logic8BitRegister::C => self.register.c = self.or(self.register.c),
            Logic8BitRegister::D => self.register.d = self.or(self.register.d),
            Logic8BitRegister::E => self.register.e = self.or(self.register.e),
            Logic8BitRegister::H => self.register.h = self.or(self.register.h),
            Logic8BitRegister::L => self.register.l = self.or(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_xor(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.register.a = self.xor(self.register.a),
            Logic8BitRegister::B => self.register.b = self.xor(self.register.b),
            Logic8BitRegister::C => self.register.c = self.xor(self.register.c),
            Logic8BitRegister::D => self.register.d = self.xor(self.register.d),
            Logic8BitRegister::E => self.register.e = self.xor(self.register.e),
            Logic8BitRegister::H => self.register.h = self.xor(self.register.h),
            Logic8BitRegister::L => self.register.l = self.xor(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_cp(&mut self, target: Logic8BitRegister) {
        match target {
            Logic8BitRegister::A => self.exec_compare(self.register.a),
            Logic8BitRegister::B => self.exec_compare(self.register.b),
            Logic8BitRegister::C => self.exec_compare(self.register.c),
            Logic8BitRegister::D => self.exec_compare(self.register.d),
            Logic8BitRegister::E => self.exec_compare(self.register.e),
            Logic8BitRegister::H => self.exec_compare(self.register.h),
            Logic8BitRegister::L => self.exec_compare(self.register.l),
            Logic8BitRegister::D8 => {}
            Logic8BitRegister::HLI => {}
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.memory.read_byte(self.pc);
    }

    fn exec_add(&mut self, value_from_register: u8, with_carry: bool) {
        self.register.a = self.add(value_from_register, with_carry);
    }

    fn exec_sub(&mut self, value_from_register: u8, with_carry: bool) {
        self.register.a = self.sub(value_from_register, with_carry);
    }

    fn exec_inc(&mut self, value_from_register: u8) {
        self.register.a = self.inc(value_from_register);
    }

    fn exec_dec(&mut self, value_from_register: u8) {
        self.register.a = self.dec(value_from_register);
    }

    fn exec_and(&mut self, value_from_register: u8) {
        self.register.a = self.and(value_from_register);
    }

    fn exec_or(&mut self, value_from_register: u8) {
        self.register.a = self.or(value_from_register);
    }

    fn exec_xor(&mut self, value_from_register: u8) {
        self.register.a = self.xor(value_from_register);
    }

    fn exec_compare(&mut self, value_from_register: u8) {
       self.compare(value_from_register);
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
        let (sub, first_did_overflow) = self.register.a.overflowing_sub(value);
        let (new_value, result_did_overflow) = sub.overflowing_sub(sub);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = true;
        self.register.f.carry = first_did_overflow || result_did_overflow;
        self.register.f.half_carry = (self.register.a & 0xf) < (value & 0xf) + carry_value;
        return new_value;
    }

    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.half_carry = value & 0xf == 0xf;
        return new_value;
    }

    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.register.f.zero = new_value == 0;
        self.register.f.substract = true;
        self.register.f.half_carry = value & 0xf == 0x0;
        return new_value;
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.register.a & value;
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.half_carry = true;
        self.register.f.carry = false;
        return new_value;
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.register.a | value;
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.register.a ^ value;
        self.register.f.zero = new_value == 0;
        self.register.f.substract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn compare(&mut self, value: u8) {
        self.register.f.zero = self.register.a == value;
        self.register.f.substract = true;
        self.register.f.half_carry = (self.register.a & 0xf) < (value & 0xf);
        self.register.f.carry = self.register.a < value;
    }

    fn get_opt_carry_flag(&self, add_carry: bool) -> u8 {
        if add_carry && self.register.f.carry {
            1
        } else {
            0
        }
    }
}

fn print_missing_register(instruction: &str, target: Logic8BitRegister) {
    unimplemented!("Missing instruction {} for register {:?}", instruction, target);
}
