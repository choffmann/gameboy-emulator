use crate::memory::Memory;

use self::registers::Register;

pub mod registers;
pub mod instructions;

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

        if instruction != 0 {
            println!("[CPU] Next instruction 0x{:x}", instruction);
        }
    }

    fn load(&mut self, register: Register, value: u8) {
        self.registers.set(register, value);
    }

    fn load_register(&mut self, from: Register, to: Register) {
        let value = self.registers.get(from);
        self.load(to, value);
    }

    fn add(&mut self, value: u8) {
        let a = self.registers.a;
        let result = a.wrapping_add(value);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (a & 0x0F) + (value & 0x0F) > 0x0F;
        self.registers.f.carry = (a as u16) + (value as u16) > 0xFF;
        self.registers.a = result;
    }

    fn add_carried(&mut self, value: u8) {
        let a = self.registers.a;
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let result = a.wrapping_add(value).wrapping_add(carry);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (a & 0x0F) + (value & 0x0F) + carry > 0x0F;
        self.registers.f.carry = (a as u16) + (value as u16) + (carry as u16) > 0xFF;
        self.registers.a = result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_a() {
        let mut cpu = Cpu::new();
        cpu.load(Register::A, 0x01);
        assert_eq!(cpu.registers.a, 0x01);
    }

    #[test]
    fn test_load_b() {
        let mut cpu = Cpu::new();
        cpu.load(Register::B, 0x01);
        assert_eq!(cpu.registers.b, 0x01);
    }

    #[test]
    fn test_load_c() {
        let mut cpu = Cpu::new();
        cpu.load(Register::C, 0x01);
        assert_eq!(cpu.registers.c, 0x01);
    }

    #[test]
    fn test_load_d() {
        let mut cpu = Cpu::new();
        cpu.load(Register::D, 0x01);
        assert_eq!(cpu.registers.d, 0x01);
    }

    #[test]
    fn test_load_e() {
        let mut cpu = Cpu::new();
        cpu.load(Register::E, 0x01);
        assert_eq!(cpu.registers.e, 0x01);
    }

    #[test]
    fn test_load_h() {
        let mut cpu = Cpu::new();
        cpu.load(Register::H, 0x01);
        assert_eq!(cpu.registers.h, 0x01);
    }

    #[test]
    fn test_load_l() {
        let mut cpu = Cpu::new();
        cpu.load(Register::L, 0x01);
        assert_eq!(cpu.registers.l, 0x01);
    }

    #[test]
    fn test_add() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x10;
        cpu.add(0x01);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_add_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.add(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_add_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0F;
        cpu.add(0x01);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.add(0x01);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_add_carried() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x10;
        cpu.registers.f.carry = true;
        cpu.add_carried(0x01);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_add_carried_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.f.carry = true;
        cpu.add_carried(0x01);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_add_carried_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0F;
        cpu.registers.f.carry = true;
        cpu.add_carried(0x01);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn test_add_carried_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.f.carry = true;
        cpu.add_carried(0x01);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn test_add_carried_carry_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.f.carry = true;
        cpu.add_carried(0xFF);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, true);
    }
}
