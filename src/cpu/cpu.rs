use crate::cpu::instructions::JumpCondition;
use crate::memory::Memory;

use super::instructions::Register8Bit;
use super::instructions::Instruction;
use super::registers::Registers;

pub struct CPU {
    pub register: Registers,
    pub pc: u16,
    pub memory: Memory,
}

impl CPU {
    pub fn boot(boot_rom: Vec<u8>) -> CPU {
        println!("[CPU] Starting CPU...");
        CPU {
            register: Registers::new(),
            pc: 0x0,
            memory: Memory::new(boot_rom),
        }
    }
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
            Instruction::JP(jmp_condition) => self.match_jmp_condition(jmp_condition),
            Instruction::JR(jmp_condition) => self.match_jmp_condition(jmp_condition),
            Instruction::JPI => self.jump(true),
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::LD(source, target) => {
                println!("[CPU] LD {} to {}", source.to_string(), target.to_string());
                return self.pc.wrapping_add(1);
            }
        }
    }

    fn match_jmp_condition(&mut self, jump_condition: JumpCondition) -> u16 {
        let should_jump = match jump_condition {
            JumpCondition::NotZero => !self.register.f.zero,
            JumpCondition::NotCarry => !self.register.f.carry,
            JumpCondition::Zero => self.register.f.zero,
            JumpCondition::Carry => self.register.f.carry,
            JumpCondition::Always => true
        };
        return self.jump(should_jump);
    }

    fn match_add(&mut self, target: Register8Bit, with_carry: bool) -> u16 {
        match target {
            Register8Bit::A => self.exec_add(self.register.a, with_carry),
            Register8Bit::B => self.exec_add(self.register.b, with_carry),
            Register8Bit::C => self.exec_add(self.register.c, with_carry),
            Register8Bit::D => self.exec_add(self.register.d, with_carry),
            Register8Bit::E => self.exec_add(self.register.e, with_carry),
            Register8Bit::H => self.exec_add(self.register.h, with_carry),
            Register8Bit::L => self.exec_add(self.register.l, with_carry),
            Register8Bit::D8 => self.exec_add(self.read_next_mem(), with_carry),
            Register8Bit::HLI => todo!("Not implemented"),
        }
    }

    fn match_inc(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_inc(self.register.a),
            Register8Bit::B => self.exec_inc(self.register.b),
            Register8Bit::C => self.exec_inc(self.register.c),
            Register8Bit::D => self.exec_inc(self.register.d),
            Register8Bit::E => self.exec_inc(self.register.e),
            Register8Bit::H => self.exec_inc(self.register.h),
            Register8Bit::L => self.exec_inc(self.register.l),
            Register8Bit::D8 => self.exec_inc(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_dec(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_dec(self.register.a),
            Register8Bit::B => self.exec_dec(self.register.b),
            Register8Bit::C => self.exec_dec(self.register.c),
            Register8Bit::D => self.exec_dec(self.register.d),
            Register8Bit::E => self.exec_dec(self.register.e),
            Register8Bit::H => self.exec_dec(self.register.h),
            Register8Bit::L => self.exec_dec(self.register.l),
            Register8Bit::D8 => self.exec_dec(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_sub(&mut self, target: Register8Bit, with_carry: bool) -> u16 {
        match target {
            Register8Bit::A => self.exec_sub(self.register.a, with_carry),
            Register8Bit::B => self.exec_sub(self.register.b, with_carry),
            Register8Bit::C => self.exec_sub(self.register.c, with_carry),
            Register8Bit::D => self.exec_sub(self.register.d, with_carry),
            Register8Bit::E => self.exec_sub(self.register.e, with_carry),
            Register8Bit::H => self.exec_sub(self.register.h, with_carry),
            Register8Bit::L => self.exec_sub(self.register.l, with_carry),
            Register8Bit::D8 => self.exec_sub(self.read_next_mem(), with_carry),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_and(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_and(self.register.a),
            Register8Bit::B => self.exec_and(self.register.b),
            Register8Bit::C => self.exec_and(self.register.c),
            Register8Bit::D => self.exec_and(self.register.d),
            Register8Bit::E => self.exec_and(self.register.e),
            Register8Bit::H => self.exec_and(self.register.h),
            Register8Bit::L => self.exec_and(self.register.l),
            Register8Bit::D8 => self.exec_and(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_or(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_or(self.register.a),
            Register8Bit::B => self.exec_or(self.register.b),
            Register8Bit::C => self.exec_or(self.register.c),
            Register8Bit::D => self.exec_or(self.register.d),
            Register8Bit::E => self.exec_or(self.register.e),
            Register8Bit::H => self.exec_or(self.register.h),
            Register8Bit::L => self.exec_or(self.register.l),
            Register8Bit::D8 => self.exec_or(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_xor(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_xor(self.register.a),
            Register8Bit::B => self.exec_xor(self.register.b),
            Register8Bit::C => self.exec_xor(self.register.c),
            Register8Bit::D => self.exec_xor(self.register.d),
            Register8Bit::E => self.exec_xor(self.register.e),
            Register8Bit::H => self.exec_xor(self.register.h),
            Register8Bit::L => self.exec_xor(self.register.l),
            Register8Bit::D8 => self.exec_xor(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    fn match_cp(&mut self, target: Register8Bit) -> u16 {
        match target {
            Register8Bit::A => self.exec_compare(self.register.a),
            Register8Bit::B => self.exec_compare(self.register.b),
            Register8Bit::C => self.exec_compare(self.register.c),
            Register8Bit::D => self.exec_compare(self.register.d),
            Register8Bit::E => self.exec_compare(self.register.e),
            Register8Bit::H => self.exec_compare(self.register.h),
            Register8Bit::L => self.exec_compare(self.register.l),
            Register8Bit::D8 => self.exec_compare(self.read_next_mem()),
            Register8Bit::HLI => todo!("Not implemented")
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.memory.read_byte(self.pc);
        let prefixed = instruction_byte == 0xcb;
        if prefixed {
            instruction_byte = self.memory.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction: {}", description);
        };

        self.pc = next_pc;
    }

    fn jump(&self, should_jump: bool) -> u16 {
        return if should_jump {
            let least_sign_byte = self.memory.read_byte(self.pc + 1) as u16;
            let most_sign_byte = self.memory.read_byte(self.pc + 2) as u16;
            (most_sign_byte << 8) | least_sign_byte
        } else {
            // If not jump, need counter to froward by 3
            // (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        };
    }

    fn read_next_mem(&self) -> u8 {
        return self.memory.read_byte(self.pc.wrapping_add(1));
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
        println!("[CPU] Execute ADD: {:x} + {:x}", self.register.a, value);
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (add, frist_did_overflow) = self.register.a.overflowing_add(value);
        let (new_value, result_did_overflow) = add.overflowing_add(carry_value);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = frist_did_overflow || result_did_overflow;
        self.register.f.half_carry = ((self.register.a & 0xf) + (value & 0xf) + carry_value) > 0xf;
        println!("[CPU] Calculated value: {:x}", value);
        return new_value;
    }

    fn sub(&mut self, value: u8, add_carry: bool) -> u8 {
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (sub, first_did_overflow) = self.register.a.overflowing_sub(value);
        let (new_value, result_did_overflow) = sub.overflowing_sub(sub);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = true;
        self.register.f.carry = first_did_overflow || result_did_overflow;
        self.register.f.half_carry = (self.register.a & 0xf) < (value & 0xf) + carry_value;
        return new_value;
    }

    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = value & 0xf == 0xf;
        return new_value;
    }

    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = true;
        self.register.f.half_carry = value & 0xf == 0x0;
        return new_value;
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.register.a & value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = true;
        self.register.f.carry = false;
        return new_value;
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.register.a | value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.register.a ^ value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn compare(&mut self, value: u8) {
        self.register.f.zero = self.register.a == value;
        self.register.f.subtract = true;
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

fn print_missing_register(instruction: &str, target: Register8Bit) {
    unimplemented!("Missing instruction {} for register {:?}", instruction, target);
}
