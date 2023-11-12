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
    pub fn execute(&mut self, instructions: Instruction) -> u16 {
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

    fn match_add(&mut self, target: Logic8BitRegister, with_carry: bool) -> u16 {
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

    fn match_inc(&mut self, target: Logic8BitRegister) -> u16 {
        match target {
            Logic8BitRegister::A => self.exec_inc(self.register.a),
            Logic8BitRegister::B => self.exec_inc(self.register.b),
            Logic8BitRegister::C => self.exec_inc(self.register.c),
            Logic8BitRegister::D => self.exec_inc(self.register.d),
            Logic8BitRegister::E => self.exec_inc(self.register.e),
            Logic8BitRegister::H => self.exec_inc(self.register.h),
            Logic8BitRegister::L => self.exec_inc(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_dec(&mut self, target: Logic8BitRegister) -> u16 {
        match target {
            Logic8BitRegister::A => self.exec_dec(self.register.a),
            Logic8BitRegister::B => self.exec_dec(self.register.b),
            Logic8BitRegister::C => self.exec_dec(self.register.c),
            Logic8BitRegister::D => self.exec_dec(self.register.d),
            Logic8BitRegister::E => self.exec_dec(self.register.e),
            Logic8BitRegister::H => self.exec_dec(self.register.h),
            Logic8BitRegister::L => self.exec_dec(self.register.l),
            Logic8BitRegister::D8 => todo!("Not implemented"),
            Logic8BitRegister::HLI => todo!("Not implemented")
        }
    }

    fn match_sub(&mut self, target: Logic8BitRegister, with_carry: bool) -> u16 {
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

    fn match_and(&mut self, target: Logic8BitRegister) -> u16 {
        match target {
            Logic8BitRegister::A => self.exec_and(self.register.a),
            Logic8BitRegister::B => self.exec_and(self.register.b),
            Logic8BitRegister::C => self.exec_and(self.register.c),
            Logic8BitRegister::D => self.exec_and(self.register.d),
            Logic8BitRegister::E => self.exec_and(self.register.e),
            Logic8BitRegister::H => self.exec_and(self.register.h),
            Logic8BitRegister::L => self.exec_and(self.register.l),
            Logic8BitRegister::D8 => {}
            Logic8BitRegister::HLI => {}
        }
    }

    fn match_or(&mut self, target: Logic8BitRegister) -> u16 {
        match target {
            Logic8BitRegister::A => self.exec_or(self.register.a),
            Logic8BitRegister::B => self.exec_or(self.register.b),
            Logic8BitRegister::C => self.exec_or(self.register.c),
            Logic8BitRegister::D => self.exec_or(self.register.d),
            Logic8BitRegister::E => self.exec_or(self.register.e),
            Logic8BitRegister::H => self.exec_or(self.register.h),
            Logic8BitRegister::L => self.exec_or(self.register.l),
            Logic8BitRegister::D8 => {}
            Logic8BitRegister::HLI => {}
        }
    }

    fn match_xor(&mut self, target: Logic8BitRegister) -> u16 {
        match target {
            Logic8BitRegister::A => self.exec_xor(self.register.a),
            Logic8BitRegister::B => self.exec_xor(self.register.b),
            Logic8BitRegister::C => self.exec_xor(self.register.c),
            Logic8BitRegister::D => self.exec_xor(self.register.d),
            Logic8BitRegister::E => self.exec_xor(self.register.e),
            Logic8BitRegister::H => self.exec_xor(self.register.h),
            Logic8BitRegister::L => self.exec_xor(self.register.l),
            Logic8BitRegister::D8 => {}
            Logic8BitRegister::HLI => {}
        }
    }

    fn match_cp(&mut self, target: Logic8BitRegister) -> u16 {
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
        let instruction_byte = self.memory.read_byte(self.pc);
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn exec_add(&mut self, value_from_register: u8, with_carry: bool) -> u16 {
        self.register.a = self.add(value_from_register, with_carry);
        return self.pc.wrapping_add(1);
    }

    fn exec_sub(&mut self, value_from_register: u8, with_carry: bool) -> u16 {
        self.register.a = self.sub(value_from_register, with_carry);
        return self.pc.wrapping_add(1);
    }

    fn exec_inc(&mut self, value_from_register: u8) -> u16 {
        self.register.a = self.inc(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn exec_dec(&mut self, value_from_register: u8) -> u16 {
        self.register.a = self.dec(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn exec_and(&mut self, value_from_register: u8) -> u16 {
        self.register.a = self.and(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn exec_or(&mut self, value_from_register: u8) -> u16 {
        self.register.a = self.or(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn exec_xor(&mut self, value_from_register: u8) -> u16 {
        self.register.a = self.xor(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn exec_compare(&mut self, value_from_register: u8) -> u16 {
        self.compare(value_from_register);
        return self.pc.wrapping_add(1);
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
