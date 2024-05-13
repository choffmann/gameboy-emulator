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
            Instruction::Ld16(_, _) => self.load_16(instruction),
            Instruction::LdCa
            | Instruction::LdAc
            | Instruction::LdNa
            | Instruction::LdAn
            | Instruction::LdHi
            | Instruction::LdHd => self.load_special(instruction),
            Instruction::Push(register) => self.push(register),
            Instruction::Pop(register) => self.pop(register),
            Instruction::Add(_) | Instruction::Adc(_) => self.add(instruction),
            Instruction::Sub(_) | Instruction::Sbc(_) => self.sub(instruction),
            _ => panic!("[CPU] Not implementet {:?}", instruction),
        }
    }

    fn exec_add(&mut self, a: u8, b: u8, carry: bool) {
        let carry_value = if carry && self.registers.f.carry { 1 } else { 0 };
        let (add, frist_did_overflow) = a.overflowing_add(b);
        let (new_value, result_did_overflow) = add.overflowing_add(carry_value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        self.registers.f.carry = frist_did_overflow || result_did_overflow;
        self.registers.set(&Register::A, new_value);
    }

    fn add(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Add(from) => match &from {
                Register::D8 => {
                    let value = self.memory.read(self.pc + 1);
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, false);
                    self.pc.wrapping_add(2)
                }
                Register::HL => {
                    let value = self.memory.read(self.registers.get_16(&Register::HL));
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, false);
                    self.pc.wrapping_add(1)
                }
                _ => {
                    let value = self.registers.get(&from);
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, false);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::Adc(from) => match &from {
                Register::D8 => {
                    let value = self.memory.read(self.pc + 1);
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, true);
                    self.pc.wrapping_add(2)
                }
                Register::HL => {
                    let value = self.memory.read(self.registers.get_16(&Register::HL));
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, true);
                    self.pc.wrapping_add(1)
                }
                _ => {
                    let value = self.registers.get(&from);
                    let a = self.registers.get(&Register::A);
                    self.exec_add(a, value, true);
                    self.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }

    fn exec_sub(&mut self, a: u8, b: u8, carry: bool) {
        let carry_value = if carry && self.registers.f.carry { 1 } else { 0 };
        let (sub, first_did_underflow) = a.overflowing_sub(b);
        let (new_value, result_did_underflow) = sub.overflowing_sub(carry_value);

        self.registers.set(&Register::A, new_value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        println!(
            "[CPU] SUB: 0x{:x} - 0x{:x} = 0x{:x}",
            a, b, new_value
        );
        println!(
            "[CPU] SUB: Half carry: {}",
            (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
            
        );
        self.registers.f.carry = first_did_underflow || result_did_underflow;
    }

    fn sub(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Sub(from) => match &from {
                Register::D8 => {
                    let value = self.memory.read(self.pc + 1);
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, false);
                    self.pc.wrapping_add(2)
                }
                Register::HL => {
                    let value = self.memory.read(self.registers.get_16(&Register::HL));
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, false);
                    self.pc.wrapping_add(1)
                }
                _ => {
                    let value = self.registers.get(&from);
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, false);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::Sbc(from) => match &from {
                Register::D8 => {
                    let value = self.memory.read(self.pc + 1);
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, true);
                    self.pc.wrapping_add(2)
                }
                Register::HL => {
                    let value = self.memory.read(self.registers.get_16(&Register::HL));
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, true);
                    self.pc.wrapping_add(1)
                }
                _ => {
                    let value = self.registers.get(&from);
                    let a = self.registers.get(&Register::A);
                    self.exec_sub(a, value, true);
                    self.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }

    fn push(&mut self, register: Register) -> u16 {
        let value = self.registers.get_16(&register);
        self.registers
            .sp
            .set(self.registers.sp.get().wrapping_sub(2));
        self.memory.write_16(self.registers.sp.get(), value);

        self.pc.wrapping_add(1)
    }

    fn pop(&mut self, register: Register) -> u16 {
        let value = self.memory.read_16(self.registers.sp.get());
        self.registers.set_16(&register, value);
        self.registers
            .sp
            .set(self.registers.sp.get().wrapping_add(2));

        self.pc.wrapping_add(1)
    }

    fn load_8(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Ld8(to, from) => match (&to, &from) {
                (Register::BC | Register::DE | Register::HL | Register::AF, Register::D8) => {
                    let value = self.memory.read(self.pc + 1);
                    let address = self.registers.get_16(&to);
                    self.memory.write(address, value);

                    self.pc.wrapping_add(2)
                }
                (Register::BC | Register::DE | Register::HL | Register::AF, from) => {
                    let value = self.registers.get(from);
                    let address = self.registers.get_16(&to);
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
                    let address = self.registers.get_16(&from);
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

    fn load_16(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Ld16(to, from) => match (&to, &from) {
                (Register::SP, Register::HL) => {
                    let value = self.registers.get_16(&Register::HL);
                    self.registers.sp.set(value);

                    self.pc.wrapping_add(1)
                }
                (Register::SP, Register::D8) => {
                    let n = self.memory.read(self.pc + 1) as u16;
                    let address = self.registers.sp.get().wrapping_add(n);
                    println!(
                        "[CPU] SP: 0x{:x} + 0x{:x} = 0x{:x}",
                        self.registers.sp.get(),
                        n,
                        address
                    );
                    self.registers.set_16(&Register::HL, address);

                    self.registers.f.zero = false;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = (self.registers.sp.get() & 0xF) + (n & 0xF) > 0xF;
                    self.registers.f.carry = (self.registers.sp.get() & 0xFF) + (n & 0xFF) > 0xFF;

                    self.pc.wrapping_add(3)
                }
                (Register::D16, Register::SP) => {
                    let address = self.memory.read_16(self.pc + 1);
                    self.registers.sp.set(address);

                    self.pc.wrapping_add(3)
                }
                (Register::BC | Register::DE | Register::HL | Register::SP, Register::D16) => {
                    let value = self.memory.read_16(self.pc + 1);
                    self.registers.set_16(&to, value);

                    self.pc.wrapping_add(3)
                }
                (to, from) => {
                    let value = self.registers.get_16(from);
                    self.registers.set_16(to, value);

                    self.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }

    fn load_special(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::LdCa => {
                let address = 0xFF00 + self.registers.get(&Register::C) as u16;
                let value = self.registers.get(&Register::A);
                self.memory.write(address, value);

                self.pc.wrapping_add(1)
            }
            Instruction::LdAc => {
                let address = 0xFF00 + self.registers.get(&Register::C) as u16;
                let value = self.memory.read(address);
                self.registers.set(&Register::A, value);

                self.pc.wrapping_add(1)
            }
            Instruction::LdNa => {
                let address = 0xFF00 + self.memory.read(self.pc + 1) as u16;
                let value = self.registers.get(&Register::A);
                self.memory.write(address, value);

                self.pc.wrapping_add(2)
            }
            Instruction::LdAn => {
                let address = 0xFF00 + self.memory.read(self.pc + 1) as u16;
                let value = self.memory.read(address);
                self.registers.set(&Register::A, value);

                self.pc.wrapping_add(2)
            }
            Instruction::LdHi => {
                let address = self.registers.get_16(&Register::HL) + 1;
                let value = self.registers.get(&Register::A);
                self.memory.write(address, value);

                self.pc.wrapping_add(1)
            }
            Instruction::LdHd => {
                let address = self.registers.get_16(&Register::HL) - 1;
                let value = self.registers.get(&Register::A);
                self.memory.write(address, value);

                self.pc.wrapping_add(1)
            }
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

    #[test]
    fn execute_ldac() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.registers.set(&Register::C, 0x42);
        cpu.memory.write(0xFF42, 0x69);

        cpu.boot(vec![0xF2]);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
    }

    #[test]
    fn execute_ldca() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set(&Register::C, 0x42);

        cpu.boot(vec![0xE2]);
        cpu.step();
        assert_eq!(cpu.memory.read(0xFF42), 0x69);
    }

    #[test]
    fn execute_ldna() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.memory.write(0xFF42, 0x00);

        cpu.boot(vec![0xE0, 0x42]);
        cpu.step();
        assert_eq!(cpu.memory.read(0xFF42), 0x69);
    }

    #[test]
    fn execute_ldan() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x00);
        cpu.memory.write(0xFF42, 0x69);

        cpu.boot(vec![0xF0, 0x42]);
        cpu.step();
        assert_eq!(cpu.registers.get(&Register::A), 0x69);
    }

    #[test]
    fn execute_ldhi() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set_16(&Register::HL, 0x1234);
        cpu.memory.write(0x1235, 0x00);

        cpu.boot(vec![0x22]);
        cpu.step();
        assert_eq!(cpu.memory.read(0x1235), 0x69);
    }

    #[test]
    fn execute_ldhd() {
        let mut cpu = Cpu::new();
        cpu.registers.set(&Register::A, 0x69);
        cpu.registers.set_16(&Register::HL, 0x1234);
        cpu.memory.write(0x1233, 0x00);

        cpu.boot(vec![0x32]);
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
        ]);
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

        cpu.boot(vec![0xF9, 0xF8, 0xFF]);
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
        cpu.registers.sp.set(0x0F);
        cpu.boot(vec![0xF8, 0x01]);
        cpu.step();
        assert_eq!(cpu.registers.get_16(&Register::HL), 0x10);
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

        cpu.boot(vec![0xC5, 0xD5, 0xE5, 0xF5, 0xF1, 0xC1, 0xD1, 0xE1]);
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
        ]);
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

        cpu.boot(vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0xC6, 0x42]);
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

        cpu.boot(vec![0x8F, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0xCE, 0x42]);

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
        ]);
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

        cpu.boot(vec![0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0xD6, 0x42]);
        cpu.step();

        // Sub A 0xFF from B
        assert_eq!(cpu.registers.get(&Register::A), 0x01);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
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
        assert_eq!(cpu.registers.f.half_carry, false);
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
        assert_eq!(cpu.registers.f.half_carry, false);
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

        cpu.boot(vec![0x9F, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0xDE, 0x42]);

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
}
