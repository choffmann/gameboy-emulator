use crate::cpu::instructions::{JumpCondition, Target16Bit};
use crate::cpu::registers::Registers;
use crate::memory::Memory;

use super::instructions::Target8Bit;
use super::instructions::{Instruction, Source16Bit};
use super::registers::Register16BitName;

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
            Instruction::DEC16(target) => self.match_dec16(target),
            Instruction::ADD16(target) => self.match_add16(target),
            Instruction::INC16(target) => self.match_inc16(target),
            Instruction::LD8(target, source) => {
                println!("[CPU] LD {:?} to {:?}", source, target);
                return self.pc.wrapping_add(1);
            }
            Instruction::LD16(target, source) => {
                let value = self.match_16bit_load_source(source);
                return self.match_16bit_load(target, value);
            }
            Instruction::LDD => {
                let value = self.register.read_a();
                let address_value = self.register.get_16bit(&Register16BitName::HL);
                self.memory.write_byte(address_value, value);
                self.match_dec16(Target16Bit::HL);
                return self.pc.wrapping_add(1);
            }
        }
    }

    fn match_8bit_load(&mut self, target: Target8Bit, value: u8) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.register.set_8bit(target.into(), value),
            Target8Bit::D8 => {}
            Target8Bit::HLI => {}
        }
        return self.pc.wrapping_add(1);
    }

    fn match_16bit_load(&mut self, target: Target16Bit, value: u16) -> u16 {
        match target {
            Target16Bit::BC => self.register.set_16bit(target.into(), value),
            Target16Bit::DE => self.register.set_16bit(target.into(), value),
            Target16Bit::HL => self.register.set_16bit(target.into(), value),
            Target16Bit::SP => self.register.set_16bit(target.into(), value),
        }

        return self.pc.wrapping_add(3);
    }

    fn match_16bit_load_source(&self, source: Source16Bit) -> u16 {
        match source {
            Source16Bit::SP
            | Source16Bit::BC
            | Source16Bit::DE
            | Source16Bit::HL => self.register.get_16bit(source.into()),
            Source16Bit::D16 => self.memory.read_next_word(self.pc),
        }
    }

    fn match_jmp_condition(&mut self, jump_condition: JumpCondition) -> u16 {
        let should_jump = match jump_condition {
            JumpCondition::NotZero => !self.register.f.zero,
            JumpCondition::NotCarry => !self.register.f.carry,
            JumpCondition::Zero => self.register.f.zero,
            JumpCondition::Carry => self.register.f.carry,
            JumpCondition::Always => true,
        };
        return self.jump(should_jump);
    }

    fn match_add(&mut self, target: Target8Bit, with_carry: bool) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_add(self.register.get_8bit(target.into()), with_carry),
            Target8Bit::D8 => self.exec_add(self.read_next_mem(), with_carry),
            Target8Bit::HLI => self.exec_add(self.get_memory_by_hl(), with_carry),
        }
    }

    fn match_add16(&mut self, target: Target16Bit) -> u16 {
        match target {
            Target16Bit::BC |
            Target16Bit::DE |
            Target16Bit::HL |
            Target16Bit::SP => {
                let register: &Register16BitName = target.into();
                let value = self.register.get_16bit(register);
                self.register.set_16bit(register, value);
            }
        }
        return self.pc.wrapping_add(1);
    }

    fn match_inc(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_inc(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_inc(self.read_next_mem()),
            Target8Bit::HLI => self.exec_inc(self.get_memory_by_hl()),
        }
    }

    fn match_dec(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_dec(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_dec(self.read_next_mem()),
            Target8Bit::HLI => self.exec_dec(self.get_memory_by_hl()),
        }
    }

    fn match_dec16(&mut self, target: Target16Bit) -> u16 {
        match target {
            Target16Bit::BC |
            Target16Bit::DE |
            Target16Bit::HL |
            Target16Bit::SP => {
                let register: &Register16BitName = target.into();
                let value = self.register.get_16bit(register).wrapping_sub(1);
                self.register.set_16bit(register, value)
            }
        }
        return self.pc.wrapping_add(1);
    }

    fn match_inc16(&mut self, target: Target16Bit) -> u16 {
        match target {
            Target16Bit::BC |
            Target16Bit::DE |
            Target16Bit::HL |
            Target16Bit::SP => {
                let register: &Register16BitName = target.into();
                let value = self.register.get_16bit(register).wrapping_add(1);
                self.register.set_16bit(register, value);
            }
        }
        return self.pc.wrapping_add(1);
    }

    fn match_sub(&mut self, target: Target8Bit, with_carry: bool) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_sub(self.register.get_8bit(target.into()), with_carry),
            Target8Bit::D8 => self.exec_sub(self.read_next_mem(), with_carry),
            Target8Bit::HLI => self.exec_sub(self.get_memory_by_hl(), with_carry),
        }
    }

    fn match_and(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_and(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_and(self.read_next_mem()),
            Target8Bit::HLI => self.exec_and(self.get_memory_by_hl()),
        }
    }

    fn match_or(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_or(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_or(self.read_next_mem()),
            Target8Bit::HLI => self.exec_or(self.get_memory_by_hl()),
        }
    }

    fn match_xor(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_xor(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_xor(self.read_next_mem()),
            Target8Bit::HLI => self.exec_xor(self.get_memory_by_hl()),
        }
    }

    fn match_cp(&mut self, target: Target8Bit) -> u16 {
        match target {
            Target8Bit::A
            | Target8Bit::B
            | Target8Bit::C
            | Target8Bit::D
            | Target8Bit::E
            | Target8Bit::H
            | Target8Bit::L => self.exec_compare(self.register.get_8bit(target.into())),
            Target8Bit::D8 => self.exec_compare(self.read_next_mem()),
            Target8Bit::HLI => self.exec_compare(self.get_memory_by_hl()),
        }
    }

    fn get_memory_by_hl(&self) -> u8 {
        let value = self.register.get_16bit(&Register16BitName::HL);
        return self.memory.read_byte(value);
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.memory.read_byte(self.pc);
        let prefixed = instruction_byte == 0xcb;
        if prefixed {
            instruction_byte = self.memory.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
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
        let value = self.add(value_from_register, with_carry);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_sub(&mut self, value_from_register: u8, with_carry: bool) -> u16 {
        let value = self.sub(value_from_register, with_carry);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_inc(&mut self, value_from_register: u8) -> u16 {
        let value = self.inc(value_from_register);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_dec(&mut self, value_from_register: u8) -> u16 {
        let value = self.dec(value_from_register);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_and(&mut self, value_from_register: u8) -> u16 {
        let value = self.and(value_from_register);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_or(&mut self, value_from_register: u8) -> u16 {
        let value = self.or(value_from_register);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_xor(&mut self, value_from_register: u8) -> u16 {
        let value = self.xor(value_from_register);
        self.register.write_a(value);
        return self.pc.wrapping_add(1);
    }

    fn exec_compare(&mut self, value_from_register: u8) -> u16 {
        self.compare(value_from_register);
        return self.pc.wrapping_add(1);
    }

    fn add(&mut self, value: u8, add_carry: bool) -> u8 {
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (add, frist_did_overflow) = self.register.read_a().overflowing_add(value);
        let (new_value, result_did_overflow) = add.overflowing_add(carry_value);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = frist_did_overflow || result_did_overflow;
        self.register.f.half_carry =
            ((self.register.read_a() & 0xf) + (value & 0xf) + carry_value) > 0xf;
        return new_value;
    }

    fn add_16(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.register.get_16bit(&Register16BitName::HL).overflowing_add(value);
        self.register.f.subtract = false;
        self.register.f.carry = did_overflow;
        self.register.f.half_carry = (self.register.get_16bit(&Register16BitName::HL) & 0xff) + (value & 0xff) > 0xff;
        return new_value;
    }

    fn sub(&mut self, value: u8, add_carry: bool) -> u8 {
        let carry_value = self.get_opt_carry_flag(add_carry);
        let (sub, first_did_overflow) = self.register.read_a().overflowing_sub(value);
        let (new_value, result_did_overflow) = sub.overflowing_sub(sub);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = true;
        self.register.f.carry = first_did_overflow || result_did_overflow;
        self.register.f.half_carry = (self.register.read_a() & 0xf) < (value & 0xf) + carry_value;
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
        let new_value = self.register.read_a() & value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = true;
        self.register.f.carry = false;
        return new_value;
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.register.read_a() | value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.register.read_a() ^ value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        return new_value;
    }

    fn compare(&mut self, value: u8) {
        let register_a = self.register.read_a();
        self.register.f.zero = register_a == value;
        self.register.f.subtract = true;
        self.register.f.half_carry = (register_a & 0xf) < (value & 0xf);
        self.register.f.carry = register_a < value;
    }

    fn get_opt_carry_flag(&self, add_carry: bool) -> u8 {
        if add_carry && self.register.f.carry {
            1
        } else {
            0
        }
    }
}
