use crate::{cpu::instructions::Instruction, memory::Memory};

use self::{command::CommandFactory, instructions::FlagCondition, registers::Register};

pub mod command;
pub mod instructions;
pub mod registers;

enum FlagUpdate {
    Zero(bool),
    Subtract(bool),
    HalfCarry(bool),
    Carry(bool),
}

pub struct Cpu {
    pub interrupts_enabled: bool,
    pub registers: registers::Registers,
    pub pc: u16,
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            interrupts_enabled: false,
            registers: registers::Registers::new(),
            pc: 0,
            memory: Memory::new(),
        }
    }

    pub fn boot(&mut self, boot_rom: Vec<u8>, game_rom: Vec<u8>) {
        self.memory.write_vec(0x0, boot_rom);
        self.memory.write_vec(0x100, game_rom);
    }

    pub fn run(&mut self) {
        // Boot Sequence
        self.pc = 0x0;
        for _ in 0..0xFF {
            self.step();
        }

        // Program Counter default value
        self.pc = 0x100;

        // TODO: Add timing
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory.read(self.pc);
        let prefixed = opcode == 0xCB;
        let instruction = if prefixed {
            self.memory.read(self.pc + 1)
        } else {
            opcode
        };

        if !prefixed {
            println!("[CPU] PC: 0x{:x} Opcode: 0x{:x}", self.pc, opcode);
        } else {
            println!("[CPU] PC: 0x{:x} Prefixed: 0x{:x}", self.pc, instruction);
        }

        let next_pc = match Instruction::from_byte(instruction, prefixed) {
            Some(instruction) => self.execute(instruction, prefixed),
            None => panic!("[CPU] Invalid instruction 0x{:x}", instruction),
        };

        if next_pc == self.pc {
            panic!("[CPU] Instruction did not increment PC");
        }

        if prefixed {
            self.pc = next_pc + 1;
        } else {
            self.pc = next_pc;
        }
    }

    fn resolve_flag_condition(&mut self, condition: &FlagCondition) -> bool {
        match condition {
            FlagCondition::NZ => !self.registers.f.zero,
            FlagCondition::Z => self.registers.f.zero,
            FlagCondition::NC => !self.registers.f.carry,
            FlagCondition::C => self.registers.f.carry,
        }
    }

    fn update_flag(&mut self, flag: FlagUpdate) {
        match flag {
            FlagUpdate::Zero(value) => self.registers.f.zero = value,
            FlagUpdate::Subtract(value) => self.registers.f.subtract = value,
            FlagUpdate::HalfCarry(value) => self.registers.f.half_carry = value,
            FlagUpdate::Carry(value) => self.registers.f.carry = value,
        }
    }

    fn execute(&mut self, instruction: Instruction, _prefixed: bool) -> u16 {
        println!("[CPU] Executing {:?}", instruction);
        let mut factory = CommandFactory::new(self);
        let mut command = factory.create_command(&instruction);
        command.execute()
    }

    fn extract_operand(&mut self, from: &Register) -> (u8, u16) {
        match from {
            Register::D8 => (self.memory.read(self.pc + 1), self.pc.wrapping_add(2)),
            Register::HL => {
                let value = self.memory.read(self.registers.get_16(&Register::HL));
                (value, self.pc.wrapping_add(1))
            }
            _ => (self.registers.get(from), self.pc.wrapping_add(1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![0x00, 0x01, 0x02, 0x03], vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(cpu.memory.read(0x0), 0x00);
        assert_eq!(cpu.memory.read(0x1), 0x01);
        assert_eq!(cpu.memory.read(0x2), 0x02);
        assert_eq!(cpu.memory.read(0x3), 0x03);

        assert_eq!(cpu.memory.read(0x100), 0x00);
        assert_eq!(cpu.memory.read(0x101), 0x01);
        assert_eq!(cpu.memory.read(0x102), 0x02);
        assert_eq!(cpu.memory.read(0x103), 0x03);
    }

    #[test]
    fn step() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![0x00, 0x00, 0x00, 0x00], vec![]);
        cpu.step();
        assert_eq!(cpu.pc, 0x1);
        cpu.step();
        assert_eq!(cpu.pc, 0x2);
        cpu.step();
        assert_eq!(cpu.pc, 0x3);
        cpu.step();
        assert_eq!(cpu.pc, 0x4);
    }

    #[test]
    fn execute_nop() {
        let mut cpu = Cpu::new();
        let pc = cpu.pc;
        let next_pc = cpu.execute(Instruction::Misc(instructions::MiscInstruction::Nop), false);
        assert_eq!(next_pc, pc + 1);
    }

    #[test]
    fn execute_ld8_immediate() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![
            0x3E, 0x42, 0x06, 0x69, 0x0e, 0x42, 0x16, 0x69, 0x1e, 0x42, 0x26, 0x69, 0x2e, 0x42,
        ], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x42);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0x69);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0x42);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0x69);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0x42);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0x69);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0x42);
    }

    #[test]
    fn execute_ld8_to_hl_from_reg() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::HL, 0x5123);

        cpu.registers.set(&Register::B, 0x43);
        cpu.registers.set(&Register::C, 0x44);
        cpu.registers.set(&Register::D, 0x45);
        cpu.registers.set(&Register::E, 0x46);

        cpu.boot(vec![0x36, 0x42, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75], vec![]);
        cpu.step();

        // Load 0x42 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x42);
        cpu.step();

        // Load B 0x43 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x43);
        cpu.step();

        // Load C 0x44 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x44);
        cpu.step();

        // Load D 0x45 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x45);
        cpu.step();

        // Load E 0x46 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x46);
        cpu.step();

        // Load H 0x51 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x51);
        cpu.step();

        // Load L 0x23 into memory at 0x5123
        assert_eq!(cpu.memory.read(0x5123), 0x23);
    }

    #[test]
    fn execute_ld8_to_reg_from_hl() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::HL, 0x5123);

        cpu.memory.write(0x5123, 0x42);
        cpu.memory.write(0x4223, 0x42);
        cpu.memory.write(0x4242, 0x42);

        cpu.boot(vec![0x46, 0x4E, 0x56, 0x5E, 0x66, 0x6E, 0x7E], vec![]);
        assert_eq!(cpu.registers.get(&Register::B), 0x00);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0x42);

        assert_eq!(cpu.registers.get(&Register::C), 0x00);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0x42);

        assert_eq!(cpu.registers.get(&Register::D), 0x00);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0x42);

        assert_eq!(cpu.registers.get(&Register::E), 0x00);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0x42);

        assert_eq!(cpu.registers.get(&Register::H), 0x51);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0x42);

        assert_eq!(cpu.registers.get(&Register::L), 0x23);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0x42);

        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x42);
    }

    #[test]
    fn execute_ld8_to_a_from_reg() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0xAA);
        cpu.registers.set(&Register::C, 0xBB);
        cpu.registers.set(&Register::D, 0x03);
        cpu.registers.set(&Register::E, 0x04);
        cpu.registers.set(&Register::H, 0x05);
        cpu.registers.set(&Register::L, 0x06);

        cpu.memory.write(0x0506, 0x69); // [HL]
        cpu.memory.write(0xAABB, 0x69); // [BC]
        cpu.memory.write(0x0304, 0x69); // [DE]
        cpu.memory.write(0xABCD, 0x69); // [nn]

        cpu.boot(vec![
            0x7F, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x0A, 0x1A, 0xFA, 0xCD, 0xAB, 0x3E,
            0x42,
        ], vec![]);
        cpu.step();

        // Load 0x00 into A LD A, A
        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        cpu.step();

        // Load 0xAA into A LD A, B
        assert_eq!(cpu.registers.get(&Register::A), 0xAA);
        cpu.step();

        // Load 0xBB into A LD A, C
        assert_eq!(cpu.registers.get(&Register::A), 0xBB);
        cpu.step();

        // Load 0x03 into A LD A, D
        assert_eq!(cpu.registers.get(&Register::A), 0x03);
        cpu.step();

        // Load 0x04 into A LD A, E
        assert_eq!(cpu.registers.get(&Register::A), 0x04);
        cpu.step();

        // Load 0x05 into A LD A, H
        assert_eq!(cpu.registers.get(&Register::A), 0x05);
        cpu.step();

        // Load 0x06 into A LD A, L
        assert_eq!(cpu.registers.get(&Register::A), 0x06);
        cpu.step();

        // Load 0x69 into A LD A, (HL)
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
        cpu.step();

        // Load 0x69 into A LD A, (BC)
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
        cpu.step();

        // Load 0x69 into A LD A, (DE)
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
        cpu.step();

        // Load 0x69 into A LD A, (nn)
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
        cpu.step();

        // Load 0x42 into A LD A, 0x42
        assert_eq!(cpu.registers.get(&Register::A), 0x42);
    }

    #[test]
    fn execute_ld8_to_reg_from_a() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x42);
        cpu.registers.set(&Register::B, 0x00);
        cpu.registers.set(&Register::C, 0x00);
        cpu.registers.set(&Register::D, 0x00);
        cpu.registers.set(&Register::E, 0x00);
        cpu.registers.set(&Register::H, 0x00);
        cpu.registers.set(&Register::L, 0x00);

        cpu.boot(vec![
            0x47, 0x4f, 0x57, 0x5f, 0x67, 0x6f, 0x7f, 0x02, 0x12, 0x77, 0xEA, 0xCD, 0xAB,
        ], vec![]);

        // Load A 0x42 into B LD B, A
        cpu.registers.set(&Register::A, 0x42);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0x42);

        // Load A 0x43 into C LD C, A
        cpu.registers.set(&Register::A, 0x43);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0x43);

        // Load A 0x44 into D LD D, A
        cpu.registers.set(&Register::A, 0x44);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0x44);

        // Load A 0x45 into E LD E, A
        cpu.registers.set(&Register::A, 0x45);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0x45);

        // Load A 0x46 into H LD H, A
        cpu.registers.set(&Register::A, 0x46);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0x46);

        // Load A 0x47 into L LD L, A
        cpu.registers.set(&Register::A, 0x47);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0x47);

        // Load A 0x48 into A LD A, A
        cpu.registers.set(&Register::A, 0x48);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x48);

        // Load A 0x49 into (BC) LD (BC), A
        cpu.registers.set(&Register::A, 0x49);
        cpu.step();
        assert_eq!(cpu.memory.read(0x4243), 0x49);

        // Load A 0x4A into (DE) LD (DE), A
        cpu.registers.set(&Register::A, 0x4A);
        cpu.step();
        assert_eq!(cpu.memory.read(0x4445), 0x4A);

        // Load A 0x48 into (HL) LD (HL), A
        cpu.registers.set(&Register::A, 0x48);
        cpu.step();
        assert_eq!(cpu.memory.read(0x4647), 0x48);

        // Load A 0x4B into (nn) LD (nn), A
        cpu.registers.set(&Register::A, 0x4B);
        cpu.step();
        assert_eq!(cpu.memory.read(0xABCD), 0x4B);
    }

    #[test]
    fn execute_ldac() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::C, 0x42);
        cpu.memory.write(0xFF42, 0x69);

        cpu.boot(vec![0xF2], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
    }

    #[test]
    fn execute_ldca() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set(&Register::C, 0x42);

        cpu.boot(vec![0xE2], vec![]);
        cpu.step();
        assert_eq!(cpu.memory.read(0xFF42), 0x69);
    }

    #[test]
    fn execute_ldna() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.memory.write(0xFF42, 0x00);

        cpu.boot(vec![0xE0, 0x42], vec![]);
        cpu.step();
        assert_eq!(cpu.memory.read(0xFF42), 0x69);
    }

    #[test]
    fn execute_ldan() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.memory.write(0xFF42, 0x69);

        cpu.boot(vec![0xF0, 0x42], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
    }

    #[test]
    fn execute_ldhi() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set_16(&Register::HL, 0x1234);
        cpu.memory.write(0x1235, 0x00);

        cpu.boot(vec![0x22], vec![]);
        cpu.step();
        assert_eq!(cpu.memory.read(0x1235), 0x69);
    }

    #[test]
    fn execute_ldhd() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set_16(&Register::HL, 0x1234);
        cpu.memory.write(0x1233, 0x00);

        cpu.boot(vec![0x32], vec![]);
        cpu.step();
        assert_eq!(cpu.memory.read(0x1233), 0x69);
    }

    #[test]
    fn execute_ld16() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::BC, 0x0000);
        cpu.registers.set_16(&Register::DE, 0x0000);
        cpu.registers.set_16(&Register::HL, 0x0000);
        cpu.registers.sp.set(0x0000);

        cpu.boot(vec![
            0x01, 0x34, 0x12, 0x11, 0x56, 0x34, 0x21, 0x78, 0x56, 0x31, 0xCD, 0xAB,
        ], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::BC), 0x1234);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::DE), 0x3456);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x5678);
        cpu.step();
        assert_eq!(cpu.registers.sp.get(), 0xABCD);
    }

    #[test]
    fn execute_ld16_sp() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::HL, 0xFF69);
        cpu.registers.sp.set(0x0000);

        cpu.boot(vec![0xF9, 0xF8, 0xFF], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.sp.get(), 0xFF69);
        cpu.step();

        // test half carry true and carry true
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x68);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);

        // test half carry true and carry false
        cpu.registers.sp.set(0x0FFF);
        cpu.pc = 0x00;
        cpu.boot(vec![0xF8, 0x01], vec![]);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x1000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_push_pop() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::BC, 0x1234);
        cpu.registers.set_16(&Register::DE, 0x5678);
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.registers.set_16(&Register::AF, 0xAA55);
        cpu.registers.sp.set(0xFFFE);

        cpu.boot(vec![0xC5, 0xD5, 0xE5, 0xF5, 0xF1, 0xC1, 0xD1, 0xE1], vec![]);
        cpu.step();
        assert_eq!(cpu.memory.read_16(0xFFFC), 0x1234);
        cpu.step();
        assert_eq!(cpu.memory.read_16(0xFFFA), 0x5678);
        cpu.step();
        assert_eq!(cpu.memory.read_16(0xFFF8), 0x9ABC);
        cpu.step();
        assert_eq!(cpu.memory.read_16(0xFFF6), 0xAA55);

        cpu.registers.set_16(&Register::BC, 0x0000);
        cpu.registers.set_16(&Register::DE, 0x0000);
        cpu.registers.set_16(&Register::HL, 0x0000);
        cpu.registers.set_16(&Register::AF, 0x0000);

        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::AF), 0xAA55);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::BC), 0x9ABC);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::DE), 0x5678);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x1234);
    }

    #[test]
    fn execute_add() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0x01);
        cpu.registers.set(&Register::C, 0x02);
        cpu.registers.set(&Register::D, 0x03);
        cpu.registers.set(&Register::E, 0x04);
        cpu.registers.set(&Register::H, 0x05);
        cpu.registers.set(&Register::L, 0x06);
        cpu.memory.write(0x0506, 0x07);

        cpu.boot(vec![
            0x87, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0xC6, 0x42,
        ], vec![]);
        cpu.step();

        // Add A 0x00
        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        cpu.step();

        // Add A 0x01
        assert_eq!(cpu.registers.get(&Register::A), 0x01);
        cpu.step();

        // Add A 0x02
        assert_eq!(cpu.registers.get(&Register::A), 0x03);
        cpu.step();

        // Add A 0x03
        assert_eq!(cpu.registers.get(&Register::A), 0x06);
        cpu.step();

        // Add A 0x04
        assert_eq!(cpu.registers.get(&Register::A), 0x0A);
        cpu.step();

        // Add A 0x05
        assert_eq!(cpu.registers.get(&Register::A), 0x0F);
        cpu.step();

        // Add A 0x06
        assert_eq!(cpu.registers.get(&Register::A), 0x15);
        cpu.step();

        // Add A (HL)
        assert_eq!(cpu.registers.get(&Register::A), 0x1C);
    }

    #[test]
    fn execute_add_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0xFF);
        cpu.registers.set(&Register::C, 0x01);
        cpu.registers.set(&Register::D, 0x0F);
        cpu.registers.set(&Register::E, 0x10);
        cpu.registers.set(&Register::H, 0x0F);
        cpu.registers.set(&Register::L, 0x10);
        cpu.memory.write(0x0F10, 0x01);

        cpu.boot(vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0xC6, 0x42], vec![]);
        cpu.step();

        // Add A 0xFF from B
        assert_eq!(cpu.registers.get(&Register::A), 0xFF);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Add A 0x01 from C
        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
        cpu.step();

        // Add A 0x0F from D
        assert_eq!(cpu.registers.get(&Register::A), 0x0F);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Add A 0x10 from E
        assert_eq!(cpu.registers.get(&Register::A), 0x1F);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Add A 0x0F from H
        assert_eq!(cpu.registers.get(&Register::A), 0x2E);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Add A 0x10 from L
        assert_eq!(cpu.registers.get(&Register::A), 0x3E);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Add A (HL)
        assert_eq!(cpu.registers.get(&Register::A), 0x3F);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_adc() {
        let mut cpu = Cpu::new();

        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0xFF);
        cpu.registers.set(&Register::C, 0x01);
        cpu.registers.set(&Register::D, 0x0F);
        cpu.registers.set(&Register::E, 0x10);
        cpu.registers.set(&Register::H, 0x0F);
        cpu.registers.set(&Register::L, 0x10);
        cpu.memory.write(0x0F10, 0x01);

        cpu.boot(vec![
            0x8F, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0xCE, 0x42,
        ], vec![]);

        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x00); // Add A 0x00 from A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xFF); // Add A 0xFF from B
        cpu.step();
        println!("{:?}", cpu.registers);
        assert_eq!(cpu.registers.get(&Register::A), 0x00); // Add A 0x01 from C
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x10); // Add A 0x0F from D carry 1
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x20); // Add A 0x10 from E
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x2F); // Add A 0x0F from H
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x3F); // Add A 0x10 from L
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x40); // Add A (HL)
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x82); // Add A 0x42
    }

    #[test]
    fn execute_sub() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0x01);
        cpu.registers.set(&Register::C, 0x02);
        cpu.registers.set(&Register::D, 0x03);
        cpu.registers.set(&Register::E, 0x04);
        cpu.registers.set(&Register::H, 0x05);
        cpu.registers.set(&Register::L, 0x06);
        cpu.memory.write(0x0506, 0x07);

        cpu.boot(vec![
            0x97, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0xD6, 0x42,
        ], vec![]);
        cpu.step();

        // Sub A 0x00
        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        cpu.step();

        // Sub A 0x01
        assert_eq!(cpu.registers.get(&Register::A), 0xFF);
        cpu.step();

        // Sub A 0x02
        assert_eq!(cpu.registers.get(&Register::A), 0xFD);
        cpu.step();

        // Sub A 0x03
        assert_eq!(cpu.registers.get(&Register::A), 0xFA);
        cpu.step();

        // Sub A 0x04
        assert_eq!(cpu.registers.get(&Register::A), 0xF6);
        cpu.step();

        // Sub A 0x05
        assert_eq!(cpu.registers.get(&Register::A), 0xF1);
        cpu.step();

        // Sub A 0x06
        assert_eq!(cpu.registers.get(&Register::A), 0xEB);
        cpu.step();

        // Sub A (HL)
        assert_eq!(cpu.registers.get(&Register::A), 0xE4);
    }

    #[test]
    fn execute_sub_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0xFF);
        cpu.registers.set(&Register::C, 0x01);
        cpu.registers.set(&Register::D, 0x0F);
        cpu.registers.set(&Register::E, 0x10);
        cpu.registers.set(&Register::H, 0x0F);
        cpu.registers.set(&Register::L, 0x10);
        cpu.memory.write(0x0F10, 0x01);

        cpu.boot(vec![0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0xD6, 0x42], vec![]);
        cpu.step();

        // Sub A 0xFF from B
        assert_eq!(cpu.registers.get(&Register::A), 0x01);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
        cpu.step();

        // Sub A 0x01 from C
        assert_eq!(cpu.registers.get(&Register::A), 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Sub A 0x0F from D
        assert_eq!(cpu.registers.get(&Register::A), 0xF1);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
        cpu.step();

        // Sub A 0x10 from E
        assert_eq!(cpu.registers.get(&Register::A), 0xE1);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Sub A 0x0F from H
        assert_eq!(cpu.registers.get(&Register::A), 0xD2);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Sub A 0x10 from L
        assert_eq!(cpu.registers.get(&Register::A), 0xC2);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Sub A (HL)
        assert_eq!(cpu.registers.get(&Register::A), 0xC1);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
        cpu.step();

        // Sub A 0x42
        assert_eq!(cpu.registers.get(&Register::A), 0x7F);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_sbc() {
        let mut cpu = Cpu::new();

        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::B, 0xFF);
        cpu.registers.set(&Register::C, 0x01);
        cpu.registers.set(&Register::D, 0x0F);
        cpu.registers.set(&Register::E, 0x10);
        cpu.registers.set(&Register::H, 0x0F);
        cpu.registers.set(&Register::L, 0x10);
        cpu.memory.write(0x0F10, 0x01);

        cpu.boot(vec![
            0x9F, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0xDE, 0x42,
        ], vec![]);

        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x00); // Sub A 0x00 from A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x01); // Sub A 0xFF from B
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xFF); // Sub A 0x01 from C
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xEF); // Sub A 0x0F from D carry 1
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xDF); // Sub A 0x10 from E
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xD0); // Sub A 0x0F from H
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xC0); // Sub A 0x10 from L
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0xBF); // Sub A (HL)
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x7D); // Sub A 0x42
    }

    #[test]
    fn execute_and() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b00000000);
        cpu.registers.set(&Register::L, 0b11111111);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![
            0xA7, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xE6, 0b11001100,
        ], vec![]);

        // And A from A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from B
        // 10101010 & 11001100 = 10001000
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10001000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from C
        // 10001000 & 11110000 = 10000000
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10000000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from D
        // 10101010 & 00001111 = 00001010
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00001010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from E
        // 00001010 & 11111111 = b00001010
        cpu.registers.set(&Register::A, 0b00001010);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00001010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from H
        // 00001010 & 10101010 = 00000000
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from L
        // 10110100 & 11111111 = 10110100
        cpu.registers.set(&Register::A, 0b10110100);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10110100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A (HL)
        // 10110100 & 10101010 = 10100000
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10100000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // And A from D8
        // 10011111 & 11001100 = 10001100
        cpu.registers.set(&Register::A, 0b10011111);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10001100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_or() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b00000000);
        cpu.registers.set(&Register::L, 0b11111111);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![
            0xB7, 0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xF6, 0b11001100,
        ], vec![]);

        // Or A from A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Or A from B
        // 10101010 | 11001100 = 11101110
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11101110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Or A from C
        // 11101110 | 11110000 = 11111110
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11111110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Or A from D
        // 11111110 | 00001111 = 11111111
        cpu.registers.set(&Register::A, 0b11111110);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Or A from E
        // 11111111 | 11111111 = 11111111
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Or A from H
        // 11111111 | 00000000 = 11111111
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_xor() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b00000000);
        cpu.registers.set(&Register::L, 0b11111111);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![
            0xAF, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xEE, 0b11001100,
        ], vec![]);

        // Xor A from A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from B
        // 00000000 ^ 11001100 = 11001100
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11001100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from C
        // 11001100 ^ 11110000 = 00111100
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00111100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from D
        // 00111100 ^ 00001111 = 00110011
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00110011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from E
        // 00110011 ^ 11111111 = 11001100
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11001100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from H
        // 11001100 ^ 00000000 = 11001100
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11001100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from L
        // 11001100 ^ 11111111 = 00110011
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00110011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from (HL)
        // 00110011 ^ 10101010 = 10011001
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10011001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Xor A from D8
        // 10011001 ^ 11001100 = 11111111
        cpu.registers.set(&Register::A, 0b00110011);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_cp() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b00000000);
        cpu.registers.set(&Register::L, 0b11111111);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![
            0xBF, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xFE, 0b11001100,
        ], vec![]);

        // Cp A from A
        // 10101010 - 10101010 = 00000000
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Cp A from B
        // 10101010 - 11001100 = 11111110
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);

        // Cp A from C
        // 10101010 - 11110000 = 10011010
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn execute_inc() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b11111111);
        cpu.registers.set(&Register::L, 0b11111110);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![0x3C, 0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C, 0x34], vec![]);

        // Inc A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Inc B
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0b11001101);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Inc C
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0b11110001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Inc D
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0b00010000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);

        // Inc E
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);

        // Inc H
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);

        // Inc L
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Inc (HL)
        cpu.registers.set_16(&Register::HL, 0x00FF);
        cpu.step();
        assert_eq!(cpu.memory.read(0x00FF), 0b10101011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn execute_dec() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b11111111);
        cpu.registers.set(&Register::H, 0b11111111);
        cpu.registers.set(&Register::L, 0b11111110);
        cpu.memory.write(0x00FF, 0b10101010);

        cpu.boot(vec![0x3D, 0x05, 0x0D, 0x15, 0x1D, 0x25, 0x2D, 0x35], vec![]);

        // Dec A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Dec B
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0b11001011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Dec C
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0b11101111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);

        // Dec D
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0b00001110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Dec E
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0b11111110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Dec H
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0b11111110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);

        // Dec L
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0b11111101);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
    }

    #[test]
    fn execute_add_16() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::BC, 0x1234);
        cpu.registers.set_16(&Register::DE, 0x5678);
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.registers.set_16(&Register::SP, 0x0000);

        cpu.boot(vec![0x09, 0x19, 0x29, 0x39], vec![]);

        // Add HL, BC
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x1234 + 0x9ABC);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Add HL, DE
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x9ABC + 0x5678);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);

        // Add HL, HL
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::HL),
            0x09ABC_u16.wrapping_add(0x9ABC)
        );
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);

        // Add HL, SP
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x9ABC + 0x0000);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_add_sp() {
        let default_sp = 0x1234_u16;
        let mut cpu = Cpu::new();

        cpu.boot(vec![0xE8, 0x00, 0xE8, 0x01, 0xE8, 0x02, 0xE8, 0xFF], vec![]);

        // Add SP, 0x00
        cpu.registers.set_16(&Register::SP, default_sp.clone());
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::SP),
            default_sp.wrapping_add(0x00)
        );
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Add SP, 0x01
        cpu.registers.set_16(&Register::SP, default_sp.clone());
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::SP),
            default_sp.wrapping_add(0x01)
        );
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Add SP, 0x02
        cpu.registers.set_16(&Register::SP, default_sp.clone());
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::SP),
            default_sp.wrapping_add(0x02)
        );
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Add SP, 0xFF
        cpu.registers.set_16(&Register::SP, default_sp.clone());
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::SP),
            default_sp.wrapping_add(0xFF)
        );
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_inc16() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::BC, 0x1234);
        cpu.registers.set_16(&Register::DE, 0x5678);
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.registers.set_16(&Register::SP, 0x0000);

        cpu.boot(vec![0x03, 0x13, 0x23, 0x33], vec![]);

        // Inc BC
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::BC), 0x1234 + 1);

        // Inc DE
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::DE), 0x5678 + 1);

        // Inc HL
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x9ABC + 1);

        // Inc SP
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::SP), 0x0000 + 1);
    }

    #[test]
    fn execute_dec16() {
        let mut cpu = Cpu::new();
        cpu.registers.set_16(&Register::BC, 0x1234);
        cpu.registers.set_16(&Register::DE, 0x5678);
        cpu.registers.set_16(&Register::HL, 0x9ABC);
        cpu.registers.set_16(&Register::SP, 0x0000);

        cpu.boot(vec![0x0B, 0x1B, 0x2B, 0x3B], vec![]);

        // Dec BC
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::BC), 0x1234 - 1);

        // Dec DE
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::DE), 0x5678 - 1);

        // Dec HL
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x9ABC - 1);

        // Dec SP
        cpu.step();
        assert_eq!(
            cpu.registers.get_16(&Register::SP),
            0x0000_u16.wrapping_sub(1)
        );
    }

    #[test]
    fn execute_swap() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0b10101010);
        cpu.registers.set(&Register::B, 0b11001100);
        cpu.registers.set(&Register::C, 0b11110000);
        cpu.registers.set(&Register::D, 0b00001111);
        cpu.registers.set(&Register::E, 0b01100010);
        cpu.registers.set(&Register::H, 0b00000000);
        cpu.registers.set(&Register::L, 0b11111111);
        cpu.memory.write(0x00FF, 0b01001101);

        cpu.boot(vec![
            0xCB, 0x37, 0xCB, 0x30, 0xCB, 0x31, 0xCB, 0x32, 0xCB, 0x33, 0xCB, 0x34, 0xCB, 0x35,
            0xCB, 0x36,
        ], vec![]);

        // Swap A
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10101010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap B
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0b11001100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap C
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0b00001111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap D
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0b11110000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap E
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0b00100110);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap H
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap L
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0b11111111);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        // Swap (HL)
        cpu.registers.set_16(&Register::HL, 0x00FF);
        cpu.step();
        assert_eq!(cpu.memory.read(0x00FF), 0b11010100);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_ccf() {
        let mut cpu = Cpu::new();

        cpu.boot(vec![0x3F, 0x3F], vec![]);

        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);

        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
        cpu.step();
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn execute_scf() {
        let mut cpu = Cpu::new();

        cpu.boot(vec![0x37, 0x37], vec![]);

        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = true;
        cpu.step();
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);

        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
        cpu.step();
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn execute_rlca() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![0x07, 0x07], vec![]);

        cpu.registers.f.zero = true;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = true;
        cpu.registers.set(&Register::A, 0b10000000);
        cpu.step();

        assert_eq!(cpu.registers.get(&Register::A), 0b00000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);

        cpu.registers.set(&Register::A, 0b00000001);
        cpu.registers.f.zero = true;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = true;
        cpu.step();

        assert_eq!(cpu.registers.get(&Register::A), 0b00000010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_rla() {
        let mut cpu = Cpu::new();

        cpu.boot(vec![0x17, 0x17], vec![]);

        cpu.registers.set(&Register::A, 0b10000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.carry, true);

        cpu.registers.set(&Register::A, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000011);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_rrca() {
        let mut cpu = Cpu::new();

        cpu.boot(vec![0x0F, 0x0F], vec![]);

        cpu.registers.set(&Register::A, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10000000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);

        cpu.registers.set(&Register::A, 0b10000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b01000000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_rra() {
        let mut cpu = Cpu::new();

        cpu.boot(vec![0x1F, 0x1F], vec![]);

        assert_eq!(cpu.registers.f.carry, false);
        cpu.registers.set(&Register::A, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.carry, true);

        assert_eq!(cpu.registers.f.carry, true);
        cpu.registers.set(&Register::A, 0b00000010);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b10000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn execute_rlc() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![
            0xCB, 0x00, 0xCB, 0x01, 0xCB, 0x02, 0xCB, 0x03, 0xCB, 0x04, 0xCB, 0x05, 0xCB, 0x06,
            0xCB, 0x07,
        ], vec![]);

        // RLC B
        cpu.registers.set(&Register::B, 0b10000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::B), 0b00000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);

        // RLC C
        cpu.registers.set(&Register::C, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::C), 0b00000010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);

        // RLC D
        cpu.registers.set(&Register::D, 0b00000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::D), 0b00000000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.carry, false);

        // RLC E
        cpu.registers.set(&Register::E, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::E), 0b00000010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);

        // RLC H
        cpu.registers.set(&Register::H, 0b10000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::H), 0b00000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);

        // RLC L
        cpu.registers.set(&Register::L, 0b00000001);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::L), 0b00000010);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, false);

        // RLC (HL)
        cpu.memory.write(0x00FF, 0b10000000);
        cpu.registers.set_16(&Register::HL, 0x00FF);
        cpu.step();
        assert_eq!(cpu.memory.read(0x00FF), 0b00000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);

        // RLC A
        cpu.registers.set(&Register::A, 0b10000000);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0b00000001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.carry, true);
    }
}
