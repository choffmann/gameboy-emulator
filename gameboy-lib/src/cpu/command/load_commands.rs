use crate::cpu::instructions::LoadInstruction;
use crate::cpu::registers::Register;
use crate::cpu::Cpu;

use super::Command;

pub struct LoadCommand<'a> {
    instruction: &'a LoadInstruction,
    cpu: &'a mut Cpu,
}

impl<'a> LoadCommand<'a> {
    pub fn new(instruction: &'a LoadInstruction, cpu: &'a mut Cpu) -> LoadCommand<'a> {
        LoadCommand { instruction, cpu }
    }

    fn push(&mut self, register: &Register) -> u16 {
        let value = self.cpu.registers.get_16(&register);
        let next_sp = self.cpu.registers.sp.get().wrapping_sub(2);
        self.cpu.registers.sp.set(next_sp);
        self.cpu.memory.write_16(self.cpu.registers.sp.get(), value);

        self.cpu.pc.wrapping_add(1)
    }

    fn pop(&mut self, register: &Register) -> u16 {
        let value = self.cpu.memory.read_16(self.cpu.registers.sp.get());
        self.cpu.registers.set_16(&register, value);
        self.cpu
            .registers
            .sp
            .set(self.cpu.registers.sp.get().wrapping_add(2));

        self.cpu.pc.wrapping_add(1)
    }

    fn load_8(&mut self, instruction: &LoadInstruction) -> u16 {
        match instruction {
            LoadInstruction::Ld8(to, from) => match (&to, &from) {
                (Register::BC | Register::DE | Register::HL | Register::AF, Register::D8) => {
                    let value = self.cpu.memory.read(self.cpu.pc + 1);
                    let address = self.cpu.registers.get_16(&to);
                    self.cpu.memory.write(address, value);

                    self.cpu.pc.wrapping_add(2)
                }
                (Register::BC | Register::DE | Register::HL | Register::AF, from) => {
                    let value = self.cpu.registers.get(from);
                    let address = self.cpu.registers.get_16(&to);
                    self.cpu.memory.write(address, value);

                    self.cpu.pc.wrapping_add(1)
                }
                (Register::D16, from) => {
                    let value = self.cpu.registers.get(from);
                    let address = self.cpu.memory.read_16(self.cpu.pc + 1);
                    self.cpu.memory.write(address, value);

                    self.cpu.pc.wrapping_add(3)
                }
                (to, Register::HL | Register::BC | Register::DE | Register::AF) => {
                    let address = self.cpu.registers.get_16(&from);
                    let value = self.cpu.memory.read(address);
                    self.cpu.registers.set(to, value);

                    self.cpu.pc.wrapping_add(1)
                }
                (to, Register::D8) => {
                    let value = self.cpu.memory.read(self.cpu.pc + 1);
                    self.cpu.registers.set(to, value);

                    self.cpu.pc.wrapping_add(2)
                }
                (to, Register::D16) => {
                    let address = self.cpu.memory.read_16(self.cpu.pc + 1);
                    let value = self.cpu.memory.read(address);
                    self.cpu.registers.set(to, value);

                    self.cpu.pc.wrapping_add(3)
                }
                (to, from) => {
                    let value = self.cpu.registers.get(from);
                    self.cpu.registers.set(to, value);

                    self.cpu.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }

    fn load_16(&mut self, instruction: &LoadInstruction) -> u16 {
        match instruction {
            LoadInstruction::Ld16(to, from) => match (&to, &from) {
                (Register::SP, Register::HL) => {
                    let value = self.cpu.registers.get_16(&Register::HL);
                    self.cpu.registers.sp.set(value);

                    self.cpu.pc.wrapping_add(1)
                }
                (Register::SP, Register::D8) => {
                    let n = self.cpu.memory.read(self.cpu.pc + 1) as u16;
                    let (address, did_overflow) = self.cpu.registers.sp.get().overflowing_add(n);
                    println!(
                        "[CPU] SP: 0x{:x} + 0x{:x} = 0x{:x}",
                        self.cpu.registers.sp.get(),
                        n,
                        address
                    );
                    self.cpu.registers.set_16(&Register::HL, address);

                    self.cpu.registers.f.zero = false;
                    self.cpu.registers.f.subtract = false;
                    self.cpu.registers.f.half_carry =
                        (((self.cpu.registers.sp.get() & 0xFFF) + (n & 0xFFF)) & 0x1000) == 0x1000;
                    self.cpu.registers.f.carry = did_overflow;

                    self.cpu.pc.wrapping_add(3)
                }
                (Register::D16, Register::SP) => {
                    let address = self.cpu.memory.read_16(self.cpu.pc + 1);
                    self.cpu.registers.sp.set(address);

                    self.cpu.pc.wrapping_add(3)
                }
                (Register::BC | Register::DE | Register::HL | Register::SP, Register::D16) => {
                    let value = self.cpu.memory.read_16(self.cpu.pc + 1);
                    self.cpu.registers.set_16(&to, value);

                    self.cpu.pc.wrapping_add(3)
                }
                (to, from) => {
                    let value = self.cpu.registers.get_16(from);
                    self.cpu.registers.set_16(to, value);

                    self.cpu.pc.wrapping_add(1)
                }
            },
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }

    fn load_special(&mut self, instruction: &LoadInstruction) -> u16 {
        match instruction {
            LoadInstruction::LdCa => {
                let address = 0xFF00 + self.cpu.registers.get(&Register::C) as u16;
                let value = self.cpu.registers.get(&Register::A);
                self.cpu.memory.write(address, value);

                self.cpu.pc.wrapping_add(1)
            }
            LoadInstruction::LdAc => {
                let address = 0xFF00 + self.cpu.registers.get(&Register::C) as u16;
                let value = self.cpu.memory.read(address);
                self.cpu.registers.set(&Register::A, value);

                self.cpu.pc.wrapping_add(1)
            }
            LoadInstruction::LdNa => {
                let address = 0xFF00 + self.cpu.memory.read(self.cpu.pc + 1) as u16;
                let value = self.cpu.registers.get(&Register::A);
                self.cpu.memory.write(address, value);

                self.cpu.pc.wrapping_add(2)
            }
            LoadInstruction::LdAn => {
                let address = 0xFF00 + self.cpu.memory.read(self.cpu.pc + 1) as u16;
                let value = self.cpu.memory.read(address);
                self.cpu.registers.set(&Register::A, value);

                self.cpu.pc.wrapping_add(2)
            }
            LoadInstruction::LdHi => {
                let address = self.cpu.registers.get_16(&Register::HL);
                let value = self.cpu.registers.get(&Register::A);
                self.cpu.memory.write(address, value);

                let value = self.cpu.registers.get_16(&Register::HL).wrapping_add(1);
                self.cpu.registers.set_16(&Register::HL, value);

                self.cpu.pc.wrapping_add(1)
            }
            LoadInstruction::LdHd => {
                let address = self.cpu.registers.get_16(&Register::HL);
                let value = self.cpu.registers.get(&Register::A);
                self.cpu.memory.write(address, value);
                
                let value = self.cpu.registers.get_16(&Register::HL).wrapping_sub(1);
                self.cpu.registers.set_16(&Register::HL, value);

                self.cpu.pc.wrapping_add(1)
            }
            _ => panic!("[CPU] Invalid instruction {:?}", instruction),
        }
    }
}

impl Command for LoadCommand<'_> {
    fn execute(&mut self) -> u16 {
        let instruction = &self.instruction;
        match instruction {
            LoadInstruction::Push(register) => self.push(register),
            LoadInstruction::Pop(ref register) => self.pop(register),
            LoadInstruction::Ld8(_, _) => self.load_8(&self.instruction),
            LoadInstruction::Ld16(_, _) => self.load_16(&self.instruction),
            LoadInstruction::LdCa
            | LoadInstruction::LdAc
            | LoadInstruction::LdNa
            | LoadInstruction::LdAn
            | LoadInstruction::LdHi
            | LoadInstruction::LdHd => self.load_special(&self.instruction),
        }
    }
}
