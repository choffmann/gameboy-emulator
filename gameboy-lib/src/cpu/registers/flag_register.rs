pub struct FlagRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
    pub rest: u8, // The rest of the flag register 4 bits
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
            rest: 0,
        }
    }

    pub fn get(&self) -> u8 {
        (if self.zero { 1 } else { 0 }) << 7
            | (if self.subtract { 1 } else { 0 }) << 6
            | (if self.half_carry { 1 } else { 0 }) << 5
            | (if self.carry { 1 } else { 0 } << 4)
            | self.rest
    }

    pub fn set(&mut self, value: u8) {
        self.zero = ((value >> 7) & 0b1) != 0;
        self.subtract = ((value >> 6) & 0b1) != 0;
        self.half_carry = ((value >> 5) & 0b1) != 0;
        self.carry = ((value >> 4) & 0b1) != 0;
        self.rest = value & 0b0000_1111;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut flag_register = FlagRegister::new();
        flag_register.zero = true;
        flag_register.subtract = true;
        flag_register.half_carry = true;
        flag_register.carry = true;
        assert_eq!(flag_register.get(), 0b1111_0000);
    }

    #[test]
    fn test_new() {
        let flag_register = FlagRegister::new();
        assert_eq!(flag_register.zero, false);
        assert_eq!(flag_register.subtract, false);
        assert_eq!(flag_register.half_carry, false);
        assert_eq!(flag_register.carry, false);
    }

    #[test]
    fn test_set_flags() {
        let mut flag_register = FlagRegister::new();
        flag_register.set(0b1010_0000);
        assert_eq!(flag_register.zero, true);
        assert_eq!(flag_register.subtract, false);
        assert_eq!(flag_register.half_carry, true);
        assert_eq!(flag_register.carry, false);
    }

    #[test]
    fn test_set() {
        let mut flag_register = FlagRegister::new();
        flag_register.set(0b1111_0000);
        assert_eq!(flag_register.zero, true);
        assert_eq!(flag_register.subtract, true);
        assert_eq!(flag_register.half_carry, true);
        assert_eq!(flag_register.carry, true);
    }
}
