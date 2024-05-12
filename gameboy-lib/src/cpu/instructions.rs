use super::registers::Register;

#[derive(Debug)]
pub enum Instruction {
    Nop,                      // No operation
    Ld8(Register, Register),  // Load 8-bit value into register
    Ld16(Register, Register), // Load 16-bit value into register
    LdCa,                     // Load $FF00 + C into A
    LdAc,                     // Load A into $FF00 + C
    LdNa,                     // Load A into $FF00 + n
    LdAn,                     // Load $FF00 + n into A
    LdHi,                     // Load A into HL + 1
    LdHd,                     // Load A into HL - 1
}

impl Instruction {
    pub fn from_byte(byte: u8, prefix: bool) -> Option<Instruction> {
        return if prefix {
            Instruction::from_byte_with_prefix(byte)
        } else {
            Instruction::from_byte_without_prefix(byte)
        };
    }

    fn from_byte_with_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None,
        }
    }

    fn from_byte_without_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Nop),
            0x06 => Some(Instruction::Ld8(Register::B, Register::D8)),
            0x0E => Some(Instruction::Ld8(Register::C, Register::D8)),
            0x16 => Some(Instruction::Ld8(Register::D, Register::D8)),
            0x1E => Some(Instruction::Ld8(Register::E, Register::D8)),
            0x26 => Some(Instruction::Ld8(Register::H, Register::D8)),
            0x2E => Some(Instruction::Ld8(Register::L, Register::D8)),

            0x7F => Some(Instruction::Ld8(Register::A, Register::A)),
            0x78 => Some(Instruction::Ld8(Register::A, Register::B)),
            0x79 => Some(Instruction::Ld8(Register::A, Register::C)),
            0x7A => Some(Instruction::Ld8(Register::A, Register::D)),
            0x7B => Some(Instruction::Ld8(Register::A, Register::E)),
            0x7C => Some(Instruction::Ld8(Register::A, Register::H)),
            0x7D => Some(Instruction::Ld8(Register::A, Register::L)),
            0x0A => Some(Instruction::Ld8(Register::A, Register::BC)),
            0x1A => Some(Instruction::Ld8(Register::A, Register::DE)),
            0x7E => Some(Instruction::Ld8(Register::A, Register::HL)),
            0xFA => Some(Instruction::Ld8(Register::A, Register::D16)),
            0x3E => Some(Instruction::Ld8(Register::A, Register::D8)),

            0x40 => Some(Instruction::Ld8(Register::B, Register::B)),
            0x41 => Some(Instruction::Ld8(Register::B, Register::C)),
            0x42 => Some(Instruction::Ld8(Register::B, Register::D)),
            0x43 => Some(Instruction::Ld8(Register::B, Register::E)),
            0x44 => Some(Instruction::Ld8(Register::B, Register::H)),
            0x45 => Some(Instruction::Ld8(Register::B, Register::L)),
            0x46 => Some(Instruction::Ld8(Register::B, Register::HL)),

            0x48 => Some(Instruction::Ld8(Register::C, Register::B)),
            0x49 => Some(Instruction::Ld8(Register::C, Register::C)),
            0x4A => Some(Instruction::Ld8(Register::C, Register::D)),
            0x4B => Some(Instruction::Ld8(Register::C, Register::E)),
            0x4C => Some(Instruction::Ld8(Register::C, Register::H)),
            0x4D => Some(Instruction::Ld8(Register::C, Register::L)),
            0x4E => Some(Instruction::Ld8(Register::C, Register::HL)),

            0x50 => Some(Instruction::Ld8(Register::D, Register::B)),
            0x51 => Some(Instruction::Ld8(Register::D, Register::C)),
            0x52 => Some(Instruction::Ld8(Register::D, Register::D)),
            0x53 => Some(Instruction::Ld8(Register::D, Register::E)),
            0x54 => Some(Instruction::Ld8(Register::D, Register::H)),
            0x55 => Some(Instruction::Ld8(Register::D, Register::L)),
            0x56 => Some(Instruction::Ld8(Register::D, Register::HL)),

            0x58 => Some(Instruction::Ld8(Register::E, Register::B)),
            0x59 => Some(Instruction::Ld8(Register::E, Register::C)),
            0x5A => Some(Instruction::Ld8(Register::E, Register::D)),
            0x5B => Some(Instruction::Ld8(Register::E, Register::E)),
            0x5C => Some(Instruction::Ld8(Register::E, Register::H)),
            0x5D => Some(Instruction::Ld8(Register::E, Register::L)),
            0x5E => Some(Instruction::Ld8(Register::E, Register::HL)),

            0x60 => Some(Instruction::Ld8(Register::H, Register::B)),
            0x61 => Some(Instruction::Ld8(Register::H, Register::C)),
            0x62 => Some(Instruction::Ld8(Register::H, Register::D)),
            0x63 => Some(Instruction::Ld8(Register::H, Register::E)),
            0x64 => Some(Instruction::Ld8(Register::H, Register::H)),
            0x65 => Some(Instruction::Ld8(Register::H, Register::L)),
            0x66 => Some(Instruction::Ld8(Register::H, Register::HL)),

            0x68 => Some(Instruction::Ld8(Register::L, Register::B)),
            0x69 => Some(Instruction::Ld8(Register::L, Register::C)),
            0x6A => Some(Instruction::Ld8(Register::L, Register::D)),
            0x6B => Some(Instruction::Ld8(Register::L, Register::E)),
            0x6C => Some(Instruction::Ld8(Register::L, Register::H)),
            0x6D => Some(Instruction::Ld8(Register::L, Register::L)),
            0x6E => Some(Instruction::Ld8(Register::L, Register::HL)),

            0x70 => Some(Instruction::Ld8(Register::HL, Register::B)),
            0x71 => Some(Instruction::Ld8(Register::HL, Register::C)),
            0x72 => Some(Instruction::Ld8(Register::HL, Register::D)),
            0x73 => Some(Instruction::Ld8(Register::HL, Register::E)),
            0x74 => Some(Instruction::Ld8(Register::HL, Register::H)),
            0x75 => Some(Instruction::Ld8(Register::HL, Register::L)),
            0x36 => Some(Instruction::Ld8(Register::HL, Register::D8)),

            0x47 => Some(Instruction::Ld8(Register::B, Register::A)),
            0x4F => Some(Instruction::Ld8(Register::C, Register::A)),
            0x57 => Some(Instruction::Ld8(Register::D, Register::A)),
            0x5F => Some(Instruction::Ld8(Register::E, Register::A)),
            0x67 => Some(Instruction::Ld8(Register::H, Register::A)),
            0x6F => Some(Instruction::Ld8(Register::L, Register::A)),
            0x02 => Some(Instruction::Ld8(Register::BC, Register::A)),
            0x12 => Some(Instruction::Ld8(Register::DE, Register::A)),
            0x77 => Some(Instruction::Ld8(Register::HL, Register::A)),
            0xEA => Some(Instruction::Ld8(Register::D16, Register::A)),

            0xF2 => Some(Instruction::LdAc),
            0xE2 => Some(Instruction::LdCa),

            0x3A => Some(Instruction::LdHd),
            0x32 => Some(Instruction::LdHd),

            0x2A => Some(Instruction::LdHi),
            0x22 => Some(Instruction::LdHi),

            0xE0 => Some(Instruction::LdNa),
            0xF0 => Some(Instruction::LdAn),

            0x01 => Some(Instruction::Ld16(Register::BC, Register::D16)),
            0x11 => Some(Instruction::Ld16(Register::DE, Register::D16)),
            0x21 => Some(Instruction::Ld16(Register::HL, Register::D16)),
            0x31 => Some(Instruction::Ld16(Register::SP, Register::D16)),

            _ => None,
        }
    }
}
