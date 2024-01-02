use serde_derive::Serialize;
use super::registers::flag_register::FlagRegister;
use super::registers::register::Register;
use super::registers::register_8bit::Register8Bit;
use super::registers::stack_pointer::StackPointer;

pub(crate) mod flag_register;
mod register;
mod register_8bit;
mod stack_pointer;

#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Registers {
    a: Register8Bit,
    b: Register8Bit,
    c: Register8Bit,
    d: Register8Bit,
    e: Register8Bit,
    pub f: FlagRegister,
    h: Register8Bit,
    l: Register8Bit,
    sp: StackPointer,
}

pub enum Register8BitName {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Register16BitName {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: Register8Bit::new("A"),
            b: Register8Bit::new("B"),
            c: Register8Bit::new("C"),
            d: Register8Bit::new("D"),
            e: Register8Bit::new("E"),
            f: FlagRegister::new(),
            h: Register8Bit::new("H"),
            l: Register8Bit::new("L"),
            sp: StackPointer::new("SP"),
        }
    }

    pub fn get_8bit(&self, register: &Register8BitName) -> u8 {
        match register {
            Register8BitName::A => self.a.get(),
            Register8BitName::B => self.b.get(),
            Register8BitName::C => self.c.get(),
            Register8BitName::D => self.d.get(),
            Register8BitName::E => self.e.get(),
            Register8BitName::F => self.f.into(),
            Register8BitName::H => self.h.get(),
            Register8BitName::L => self.l.get(),
        }
    }

    pub fn get_16bit(&self, registers: &Register16BitName) -> u16 {
        match registers {
            Register16BitName::AF => self.get_af(),
            Register16BitName::BC => self.get_bc(),
            Register16BitName::DE => self.get_de(),
            Register16BitName::HL => self.get_hl(),
            Register16BitName::SP => self.sp.get(),
        }
    }

    pub fn set_8bit(&mut self, register: &Register8BitName, value: u8) {
        match register {
            Register8BitName::A => self.a.set(value),
            Register8BitName::B => self.b.set(value),
            Register8BitName::C => self.c.set(value),
            Register8BitName::D => self.d.set(value),
            Register8BitName::E => self.e.set(value),
            Register8BitName::F => self.f = value.into(),
            Register8BitName::H => self.h.set(value),
            Register8BitName::L => self.l.set(value),
        }
    }

    pub fn set_16bit(&mut self, register: &Register16BitName, value: u16) {
        match register {
            Register16BitName::AF => self.set_af(value),
            Register16BitName::BC => self.set_bc(value),
            Register16BitName::DE => self.set_de(value),
            Register16BitName::HL => self.set_hl(value),
            Register16BitName::SP => self.sp.set(value),
        }
    }

    pub fn write_a(&mut self, value: u8) {
        self.a.set(value);
    }

    pub fn read_a(&self) -> u8 {
        return self.a.get();
    }

    fn get_af(&self) -> u16 {
        let value = (self.a.get() as u16) << 8 | u8::from(self.f) as u16;
        println!("[REG AF] Reading value: 0x{:x}", value);
        return value;
    }

    fn set_af(&mut self, value: u16) {
        println!("[REG AF] Writing value: 0x{:x}", value);
        self.a.set(((value & 0xff00) >> 8) as u8);
        self.f = FlagRegister::from((value & 0xff) as u8);
    }

    fn get_bc(&self) -> u16 {
        let value = (self.b.get() as u16) << 8 | self.c.get() as u16;
        println!("[REG BC] Reading value: 0x{:x}", value);
        return value;
    }

    fn set_bc(&mut self, value: u16) {
        println!("[REG BC] Writing value: 0x{:x}", value);
        self.b.set(((value & 0xff00) >> 8) as u8);
        self.c.set((value & 0xff) as u8);
    }

    fn get_de(&self) -> u16 {
        let value = (self.d.get() as u16) << 8 | self.e.get() as u16;
        println!("[REG DE] Reading value: 0x{:x}", value);
        return value;
    }

    fn set_de(&mut self, value: u16) {
        println!("[REG DE] Writing value: 0x{:x}", value);
        self.d.set(((value & 0xff00) >> 8) as u8);
        self.e.set((value & 0xff) as u8);
    }

    fn get_hl(&self) -> u16 {
        let value = (self.h.get() as u16) << 8 | self.l.get() as u16;
        println!("[REG HL] Reading value: 0x{:x}", value);
        return value;
    }

    fn set_hl(&mut self, value: u16) {
        println!("[REG HL] Writing value: 0x{:x}", value);
        self.h.set(((value & 0xff00) >> 8) as u8);
        self.l.set((value & 0xff) as u8);
    }
}
