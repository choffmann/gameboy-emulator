use self::{flag_register::FlagRegister, stack_pointer::StackPointer};

pub(crate) mod flag_register;
pub(crate) mod stack_pointer;

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    D8,
    D16,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
    pub sp: StackPointer,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagRegister::new(),
            h: 0,
            l: 0,
            sp: StackPointer::new(),
        }
    }

    pub fn get(&self, register: &Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f.get(),
            Register::H => self.h,
            Register::L => self.l,
            _ => panic!("Invalid register"),
        }
    }

    pub fn get_16(&self, register: &Register) -> u16 {
        match register {
            Register::AF => self.get_af(),
            Register::BC => self.get_bc(),
            Register::DE => self.get_de(),
            Register::HL => self.get_hl(),
            Register::SP => self.sp.get(),
            _ => panic!("Invalid register"),
        }
    }

    pub fn set(&mut self, register: &Register, value: u8) {
        println!("[REG] Setting 8-bit register {:?} to 0x{:X}", register, value);
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f.set(value),
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::BC => self.set_bc((self.b as u16) << 8 | value as u16),
            Register::DE => self.set_de((self.d as u16) << 8 | value as u16),
            Register::HL => self.set_hl((self.h as u16) << 8 | value as u16),
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_16(&mut self, register: &Register, value: u16) {
        println!("[REG] Setting 16-bit register {:?} to 0x{:X}", register, value);
        match register {
            Register::AF => self.set_af(value),
            Register::BC => self.set_bc(value),
            Register::DE => self.set_de(value),
            Register::HL => self.set_hl(value),
            Register::SP => self.sp.set(value),
            _ => panic!("Invalid register"),
        }
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.get() as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f.set(value as u8);
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut registers = Registers::new();
        registers.a = 1;
        registers.b = 2;
        registers.c = 3;
        registers.d = 4;
        registers.e = 5;
        registers.f.set(6);
        registers.h = 7;
        registers.l = 8;

        assert_eq!(registers.get(&Register::A), 1);
        assert_eq!(registers.get(&Register::B), 2);
        assert_eq!(registers.get(&Register::C), 3);
        assert_eq!(registers.get(&Register::D), 4);
        assert_eq!(registers.get(&Register::E), 5);
        assert_eq!(registers.get(&Register::F), 6);
        assert_eq!(registers.get(&Register::H), 7);
        assert_eq!(registers.get(&Register::L), 8);
    }

    #[test]
    fn get_16() {
        let mut registers = Registers::new();
        registers.set_af(0x0102);
        registers.set_bc(0x0304);
        registers.set_de(0x0506);
        registers.set_hl(0x0708);
        registers.sp.set(0x090A);

        assert_eq!(registers.get_16(&Register::AF), 0x0102);
        assert_eq!(registers.get_16(&Register::BC), 0x0304);
        assert_eq!(registers.get_16(&Register::DE), 0x0506);
        assert_eq!(registers.get_16(&Register::HL), 0x0708);
        assert_eq!(registers.get_16(&Register::SP), 0x090A);
    }

    #[test]
    fn set() {
        let mut registers = Registers::new();
        registers.set(&Register::A, 1);
        registers.set(&Register::B, 2);
        registers.set(&Register::C, 3);
        registers.set(&Register::D, 4);
        registers.set(&Register::E, 5);
        registers.set(&Register::F, 6);
        registers.set(&Register::H, 7);
        registers.set(&Register::L, 8);

        assert_eq!(registers.a, 1);
        assert_eq!(registers.b, 2);
        assert_eq!(registers.c, 3);
        assert_eq!(registers.d, 4);
        assert_eq!(registers.e, 5);
        assert_eq!(registers.f.get(), 6);
        assert_eq!(registers.h, 7);
        assert_eq!(registers.l, 8);
    }

    #[test]
    fn set_16() {
        let mut registers = Registers::new();
        registers.set_16(&Register::AF, 0x0102);
        registers.set_16(&Register::BC, 0x0304);
        registers.set_16(&Register::DE, 0x0506);
        registers.set_16(&Register::HL, 0x0708);
        registers.set_16(&Register::SP, 0x090A);

        assert_eq!(registers.get_af(), 0x0102);
        assert_eq!(registers.get_bc(), 0x0304);
        assert_eq!(registers.get_de(), 0x0506);
        assert_eq!(registers.get_hl(), 0x0708);
        assert_eq!(registers.sp.get(), 0x090A);
    }
}
