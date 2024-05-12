use super::registers::Register;

pub enum Instruction {
    Load(Register),
    LoadRegister(Register, Register),
    Add(u8),
    AddCarried(u8),
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
            0x3E => Some(Instruction::Load(Register::A)),
            0x06 => Some(Instruction::Load(Register::B)),
            0x0E => Some(Instruction::Load(Register::C)),
            0x16 => Some(Instruction::Load(Register::D)),
            0x1E => Some(Instruction::Load(Register::E)),
            0x26 => Some(Instruction::Load(Register::H)),
            0x2E => Some(Instruction::Load(Register::L)),
            0x7F => Some(Instruction::LoadRegister(Register::A, Register::A)),
            0x78 => Some(Instruction::LoadRegister(Register::A, Register::B)),
            0x79 => Some(Instruction::LoadRegister(Register::A, Register::C)),
            0x7A => Some(Instruction::LoadRegister(Register::A, Register::D)),
            0x7B => Some(Instruction::LoadRegister(Register::A, Register::E)),
            0x7C => Some(Instruction::LoadRegister(Register::A, Register::H)),
            0x7D => Some(Instruction::LoadRegister(Register::A, Register::L)),
            0x47 => Some(Instruction::LoadRegister(Register::B, Register::A)),
            0x40 => Some(Instruction::LoadRegister(Register::B, Register::B)),
            0x41 => Some(Instruction::LoadRegister(Register::B, Register::C)),
            0x42 => Some(Instruction::LoadRegister(Register::B, Register::D)),
            0x43 => Some(Instruction::LoadRegister(Register::B, Register::E)),
            0x44 => Some(Instruction::LoadRegister(Register::B, Register::H)),
            0x45 => Some(Instruction::LoadRegister(Register::B, Register::L)),
            0x4F => Some(Instruction::LoadRegister(Register::C, Register::A)),
            0x48 => Some(Instruction::LoadRegister(Register::C, Register::B)),
            0x49 => Some(Instruction::LoadRegister(Register::C, Register::C)),
            _ => None,
        }
    }
}
