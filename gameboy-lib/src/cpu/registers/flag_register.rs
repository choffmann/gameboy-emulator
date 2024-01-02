use serde_derive::Serialize;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct FlagRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }
    }
}

impl From<FlagRegister> for u8 {
    fn from(value: FlagRegister) -> Self {
        (if value.zero { 1 } else { 0 }) << 7
            | (if value.subtract { 1 } else { 0 }) << 6
            | (if value.half_carry { 1 } else { 0 }) << 5
            | (if value.carry { 1 } else { 0 } << 4)
    }
}

impl From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        let zero = ((value >> 7) & 0b1) != 0;
        let subtract = ((value >> 6) & 0b1) != 0;
        let half_carry = ((value >> 5) & 0b1) != 0;
        let carry = ((value >> 4) & 0b1) != 0;

        return FlagRegister {
            zero,
            subtract,
            half_carry,
            carry,
        };
    }
}

