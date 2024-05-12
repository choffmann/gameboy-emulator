use crate::{cpu::instructions::Instruction, memory::Memory};

use self::registers::Register;

pub mod instructions;
pub mod registers;

pub struct Cpu {
    pub registers: registers::Registers,
    pub pc: u16,
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: registers::Registers::new(),
            pc: 0,
            memory: Memory::new(),
        }
    }

    pub fn boot(&mut self, boot_rom: Vec<u8>) {
        self.pc = 0x100;
        self.memory.write_vec(0x100, boot_rom);
    }

    pub fn step(&mut self) {
        let opcode = self.memory.read(self.pc);
        let prefixed = opcode == 0xCB;
        let instruction = if prefixed {
            self.memory.read(self.pc + 1)
        } else {
            opcode
        };

        println!("[CPU] Next instruction 0x{:x}", instruction);

        let next_pc = match Instruction::from_byte(instruction, prefixed) {
            Some(instruction) => self.execute(instruction, prefixed),
            None => panic!("[CPU] Invalid instruction 0x{:x}", instruction),
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction, prefixed: bool) -> u16 {
        match instruction {
            Instruction::Nop => self.pc.wrapping_add(1),
            Instruction::Ld8(_, _) => self.load_8(instruction),
            _ => panic!("[CPU] Not implementet {:?}", instruction),
        }
    }

    fn load_8(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Ld8(to, from) => match (&to, &from) {
                (Register::BC | Register::DE | Register::HL | Register::AF, Register::D8) => {
                    let value = self.memory.read(self.pc + 1);
                    let address = self.registers.get_16(to);
                    self.memory.write(address, value);

                    self.pc.wrapping_add(2)
                }
                (Register::BC | Register::DE | Register::HL | Register::AF, from) => {
                    let value = self.registers.get(from);
                    let address = self.registers.get_16(to);
                    self.memory.write(address, value);

                    self.pc.wrapping_add(1)
                }
                (Register::D16, from) => {
                    let value = self.registers.get(from);
                    let address = self.memory.read_16(self.pc + 1);
                    self.memory.write(address, value);

                    self.pc.wrapping_add(3)
                }
                (to, Register::HL | Register::BC | Register::DE | Register::AF) => {
                    let address = self.registers.get_16(from);
                    let value = self.memory.read(address);
                    self.registers.set(to, value);

                    self.pc.wrapping_add(1)
                }
                (to, Register::D8) => {
                    let value = self.memory.read(self.pc + 1);
                    self.registers.set(to, value);

                    self.pc.wrapping_add(2)
                }
                (to, Register::D16) => {
                    let address = self.memory.read_16(self.pc + 1);
                    let value = self.memory.read(address);
                    self.registers.set(to, value);

                    self.pc.wrapping_add(3)
                }
                (to, from) => {
                    let value = self.registers.get(from);
                    self.registers.set(to, value);

                    self.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(cpu.pc, 0x100);
        assert_eq!(cpu.memory.read(0x100), 0x00);
        assert_eq!(cpu.memory.read(0x101), 0x01);
        assert_eq!(cpu.memory.read(0x102), 0x02);
        assert_eq!(cpu.memory.read(0x103), 0x03);
    }

    #[test]
    fn step() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![0x00, 0x00, 0x00, 0x00]);
        cpu.step();
        assert_eq!(cpu.pc, 0x101);
        cpu.step();
        assert_eq!(cpu.pc, 0x102);
        cpu.step();
        assert_eq!(cpu.pc, 0x103);
        cpu.step();
        assert_eq!(cpu.pc, 0x104);
    }

    #[test]
    fn execute_nop() {
        let mut cpu = Cpu::new();
        let pc = cpu.pc;
        let next_pc = cpu.execute(Instruction::Nop, false);
        assert_eq!(next_pc, pc + 1);
    }

    #[test]
    fn execute_ld8_immediate() {
        let mut cpu = Cpu::new();
        cpu.boot(vec![
            0x3E, 0x42, 0x06, 0x69, 0x0e, 0x42, 0x16, 0x69, 0x1e, 0x42, 0x26, 0x69, 0x2e, 0x42,
        ]);
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

        cpu.boot(vec![0x36, 0x42, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75]);
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

        cpu.boot(vec![0x46, 0x4E, 0x56, 0x5E, 0x66, 0x6E, 0x7E]);
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
        ]);
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
        ]);

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
}
