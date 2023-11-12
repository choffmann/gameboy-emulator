use std::convert::From;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagRegister {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl From<FlagRegister> for u8 {
    fn from(value: FlagRegister) -> Self {
        (if value.zero { 1 } else { 0 }) << 7
            | (if value.substract { 1 } else { 0 }) << 6
            | (if value.half_carry { 1 } else { 0 }) << 5
            | (if value.carry { 1 } else { 0 } << 4)
    }
}

impl From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        let zero = ((value >> 7) & 0b1) != 0;
        let substract = ((value >> 6) & 0b1) != 0;
        let half_carry = ((value >> 5) & 0b1) != 0;
        let carry = ((value >> 4) & 0b1) != 0;

        return FlagRegister {
            zero,
            substract,
            half_carry,
            carry,
        };
    }
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        self.f = FlagRegister::from((value & 0xff) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}
