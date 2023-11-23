use super::registers::{Register16BitName, Register8BitName};

#[derive(Debug)]
pub enum Instruction {
    NOP,
    // Arithmetic Instructions
    ADD(Target8Bit),
    ADC(Target8Bit),
    SUB(Target8Bit),
    SBC(Target8Bit),
    INC(Target8Bit),
    DEC(Target8Bit),
    AND(Target8Bit),
    OR(Target8Bit),
    XOR(Target8Bit),
    CP(Target8Bit),
    DEC16(Target16Bit),
    ADD16(Target16Bit),
    INC16(Target16Bit),

    // Jump Instructions
    JP(JumpCondition),
    JR(JumpCondition),
    JPI,

    // Load Instructions
    LD8(Target8Bit, Target8Bit),
    LD16(Target16Bit, Source16Bit),
    LDD,
    LDC,
    LDHA,

    // CB Flag
    BIT(u8, Target8Bit),
    RES(u8, Target8Bit),
    SET(u8, Target8Bit),
}

#[derive(Debug, PartialEq)]
pub enum Target8Bit {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Debug)]
pub enum Target16Bit {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Source16Bit {
    BC,
    DE,
    HL,
    SP,
    D16,
}

#[derive(Debug)]
pub enum JumpCondition {
    NotZero,
    NotCarry,
    Zero,
    Carry,
    Always,
}

impl From<&Target8Bit> for &Register8BitName {
    fn from(value: &Target8Bit) -> Self {
        match value {
            Target8Bit::A => &Register8BitName::A,
            Target8Bit::B => &Register8BitName::B,
            Target8Bit::C => &Register8BitName::C,
            Target8Bit::D => &Register8BitName::D,
            Target8Bit::E => &Register8BitName::E,
            Target8Bit::H => &Register8BitName::H,
            Target8Bit::L => &Register8BitName::L,
            Target8Bit::D8 => panic!("Impossible: {:?}", value),
            Target8Bit::HLI => panic!("Impossible: {:?}", value),
        }
    }
}

impl From<Target8Bit> for &Register8BitName {
    fn from(value: Target8Bit) -> Self {
        match value {
            Target8Bit::A => &Register8BitName::A,
            Target8Bit::B => &Register8BitName::B,
            Target8Bit::C => &Register8BitName::C,
            Target8Bit::D => &Register8BitName::D,
            Target8Bit::E => &Register8BitName::E,
            Target8Bit::H => &Register8BitName::H,
            Target8Bit::L => &Register8BitName::L,
            Target8Bit::D8 => panic!("Impossible: {:?}", value),
            Target8Bit::HLI => panic!("Impossible: {:?}", value),
        }
    }
}

impl From<&Target16Bit> for &Register16BitName {
    fn from(value: &Target16Bit) -> Self {
        match value {
            Target16Bit::BC => &Register16BitName::BC,
            Target16Bit::DE => &Register16BitName::DE,
            Target16Bit::HL => &Register16BitName::HL,
            Target16Bit::SP => &Register16BitName::SP,
        }
    }
}

impl From<Target16Bit> for &Register16BitName {
    fn from(value: Target16Bit) -> Self {
        match value {
            Target16Bit::BC => &Register16BitName::BC,
            Target16Bit::DE => &Register16BitName::DE,
            Target16Bit::HL => &Register16BitName::HL,
            Target16Bit::SP => &Register16BitName::SP,
        }
    }
}

impl From<Source16Bit> for &Register16BitName {
    fn from(value: Source16Bit) -> Self {
        match value {
            Source16Bit::BC => &Register16BitName::BC,
            Source16Bit::DE => &Register16BitName::DE,
            Source16Bit::HL => &Register16BitName::HL,
            Source16Bit::SP => &Register16BitName::SP,
            Source16Bit::D16 => panic!("Impossible: {:?}", value),
        }
    }
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
            0x40 => Some(Instruction::BIT(0, Target8Bit::B)),
            0x41 => Some(Instruction::BIT(0, Target8Bit::C)),
            0x42 => Some(Instruction::BIT(0, Target8Bit::D)),
            0x43 => Some(Instruction::BIT(0, Target8Bit::E)),
            0x44 => Some(Instruction::BIT(0, Target8Bit::H)),
            0x45 => Some(Instruction::BIT(0, Target8Bit::L)),
            0x46 => Some(Instruction::BIT(0, Target8Bit::HLI)),
            0x47 => Some(Instruction::BIT(0, Target8Bit::A)),

            0x48 => Some(Instruction::BIT(1, Target8Bit::B)),
            0x49 => Some(Instruction::BIT(1, Target8Bit::C)),
            0x4a => Some(Instruction::BIT(1, Target8Bit::D)),
            0x4b => Some(Instruction::BIT(1, Target8Bit::E)),
            0x4c => Some(Instruction::BIT(1, Target8Bit::H)),
            0x4d => Some(Instruction::BIT(1, Target8Bit::L)),
            0x4e => Some(Instruction::BIT(1, Target8Bit::HLI)),
            0x4f => Some(Instruction::BIT(1, Target8Bit::A)),

            0x50 => Some(Instruction::BIT(2, Target8Bit::B)),
            0x51 => Some(Instruction::BIT(2, Target8Bit::C)),
            0x52 => Some(Instruction::BIT(2, Target8Bit::D)),
            0x53 => Some(Instruction::BIT(2, Target8Bit::E)),
            0x54 => Some(Instruction::BIT(2, Target8Bit::H)),
            0x55 => Some(Instruction::BIT(2, Target8Bit::L)),
            0x56 => Some(Instruction::BIT(2, Target8Bit::HLI)),
            0x57 => Some(Instruction::BIT(2, Target8Bit::A)),

            0x58 => Some(Instruction::BIT(3, Target8Bit::B)),
            0x59 => Some(Instruction::BIT(3, Target8Bit::C)),
            0x5a => Some(Instruction::BIT(3, Target8Bit::D)),
            0x5b => Some(Instruction::BIT(3, Target8Bit::E)),
            0x5c => Some(Instruction::BIT(3, Target8Bit::H)),
            0x5d => Some(Instruction::BIT(3, Target8Bit::L)),
            0x5e => Some(Instruction::BIT(3, Target8Bit::HLI)),
            0x5f => Some(Instruction::BIT(3, Target8Bit::A)),

            0x60 => Some(Instruction::BIT(4, Target8Bit::B)),
            0x61 => Some(Instruction::BIT(4, Target8Bit::C)),
            0x62 => Some(Instruction::BIT(4, Target8Bit::D)),
            0x63 => Some(Instruction::BIT(4, Target8Bit::E)),
            0x64 => Some(Instruction::BIT(4, Target8Bit::H)),
            0x65 => Some(Instruction::BIT(4, Target8Bit::L)),
            0x66 => Some(Instruction::BIT(4, Target8Bit::HLI)),
            0x67 => Some(Instruction::BIT(4, Target8Bit::A)),

            0x68 => Some(Instruction::BIT(5, Target8Bit::B)),
            0x69 => Some(Instruction::BIT(5, Target8Bit::C)),
            0x6a => Some(Instruction::BIT(5, Target8Bit::D)),
            0x6b => Some(Instruction::BIT(5, Target8Bit::E)),
            0x6c => Some(Instruction::BIT(5, Target8Bit::H)),
            0x6d => Some(Instruction::BIT(5, Target8Bit::L)),
            0x6e => Some(Instruction::BIT(5, Target8Bit::HLI)),
            0x6f => Some(Instruction::BIT(5, Target8Bit::A)),

            0x70 => Some(Instruction::BIT(6, Target8Bit::B)),
            0x71 => Some(Instruction::BIT(6, Target8Bit::C)),
            0x72 => Some(Instruction::BIT(6, Target8Bit::D)),
            0x73 => Some(Instruction::BIT(6, Target8Bit::E)),
            0x74 => Some(Instruction::BIT(6, Target8Bit::H)),
            0x75 => Some(Instruction::BIT(6, Target8Bit::L)),
            0x76 => Some(Instruction::BIT(6, Target8Bit::HLI)),
            0x77 => Some(Instruction::BIT(6, Target8Bit::A)),

            0x78 => Some(Instruction::BIT(7, Target8Bit::B)),
            0x79 => Some(Instruction::BIT(7, Target8Bit::C)),
            0x7a => Some(Instruction::BIT(7, Target8Bit::D)),
            0x7b => Some(Instruction::BIT(7, Target8Bit::E)),
            0x7c => Some(Instruction::BIT(7, Target8Bit::H)),
            0x7d => Some(Instruction::BIT(7, Target8Bit::L)),
            0x7e => Some(Instruction::BIT(7, Target8Bit::HLI)),
            0x7f => Some(Instruction::BIT(7, Target8Bit::A)),

            0x80 => Some(Instruction::RES(0, Target8Bit::B)),
            0x81 => Some(Instruction::RES(0, Target8Bit::C)),
            0x82 => Some(Instruction::RES(0, Target8Bit::D)),
            0x83 => Some(Instruction::RES(0, Target8Bit::E)),
            0x84 => Some(Instruction::RES(0, Target8Bit::H)),
            0x85 => Some(Instruction::RES(0, Target8Bit::L)),
            0x86 => Some(Instruction::RES(0, Target8Bit::HLI)),
            0x87 => Some(Instruction::RES(0, Target8Bit::A)),

            0x88 => Some(Instruction::RES(1, Target8Bit::B)),
            0x89 => Some(Instruction::RES(1, Target8Bit::C)),
            0x8a => Some(Instruction::RES(1, Target8Bit::D)),
            0x8b => Some(Instruction::RES(1, Target8Bit::E)),
            0x8c => Some(Instruction::RES(1, Target8Bit::H)),
            0x8d => Some(Instruction::RES(1, Target8Bit::L)),
            0x8e => Some(Instruction::RES(1, Target8Bit::HLI)),
            0x8f => Some(Instruction::RES(1, Target8Bit::A)),

            0x90 => Some(Instruction::RES(2, Target8Bit::B)),
            0x91 => Some(Instruction::RES(2, Target8Bit::C)),
            0x92 => Some(Instruction::RES(2, Target8Bit::D)),
            0x93 => Some(Instruction::RES(2, Target8Bit::E)),
            0x94 => Some(Instruction::RES(2, Target8Bit::H)),
            0x95 => Some(Instruction::RES(2, Target8Bit::L)),
            0x96 => Some(Instruction::RES(2, Target8Bit::HLI)),
            0x97 => Some(Instruction::RES(2, Target8Bit::A)),

            0x98 => Some(Instruction::RES(3, Target8Bit::B)),
            0x99 => Some(Instruction::RES(3, Target8Bit::C)),
            0x9a => Some(Instruction::RES(3, Target8Bit::D)),
            0x9b => Some(Instruction::RES(3, Target8Bit::E)),
            0x9c => Some(Instruction::RES(3, Target8Bit::H)),
            0x9d => Some(Instruction::RES(3, Target8Bit::L)),
            0x9e => Some(Instruction::RES(3, Target8Bit::HLI)),
            0x9f => Some(Instruction::RES(3, Target8Bit::A)),

            0xa0 => Some(Instruction::RES(4, Target8Bit::B)),
            0xa1 => Some(Instruction::RES(4, Target8Bit::C)),
            0xa2 => Some(Instruction::RES(4, Target8Bit::D)),
            0xa3 => Some(Instruction::RES(4, Target8Bit::E)),
            0xa4 => Some(Instruction::RES(4, Target8Bit::H)),
            0xa5 => Some(Instruction::RES(4, Target8Bit::L)),
            0xa6 => Some(Instruction::RES(4, Target8Bit::HLI)),
            0xa7 => Some(Instruction::RES(4, Target8Bit::A)),

            0xa8 => Some(Instruction::RES(5, Target8Bit::B)),
            0xa9 => Some(Instruction::RES(5, Target8Bit::C)),
            0xaa => Some(Instruction::RES(5, Target8Bit::D)),
            0xab => Some(Instruction::RES(5, Target8Bit::E)),
            0xac => Some(Instruction::RES(5, Target8Bit::H)),
            0xad => Some(Instruction::RES(5, Target8Bit::L)),
            0xae => Some(Instruction::RES(5, Target8Bit::HLI)),
            0xaf => Some(Instruction::RES(5, Target8Bit::A)),

            0xb0 => Some(Instruction::RES(6, Target8Bit::B)),
            0xb1 => Some(Instruction::RES(6, Target8Bit::C)),
            0xb2 => Some(Instruction::RES(6, Target8Bit::D)),
            0xb3 => Some(Instruction::RES(6, Target8Bit::E)),
            0xb4 => Some(Instruction::RES(6, Target8Bit::H)),
            0xb5 => Some(Instruction::RES(6, Target8Bit::L)),
            0xb6 => Some(Instruction::RES(6, Target8Bit::HLI)),
            0xb7 => Some(Instruction::RES(6, Target8Bit::A)),

            0xb8 => Some(Instruction::RES(7, Target8Bit::B)),
            0xb9 => Some(Instruction::RES(7, Target8Bit::C)),
            0xba => Some(Instruction::RES(7, Target8Bit::D)),
            0xbb => Some(Instruction::RES(7, Target8Bit::E)),
            0xbc => Some(Instruction::RES(7, Target8Bit::H)),
            0xbd => Some(Instruction::RES(7, Target8Bit::L)),
            0xbe => Some(Instruction::RES(7, Target8Bit::HLI)),
            0xbf => Some(Instruction::RES(7, Target8Bit::A)),

            0xc0 => Some(Instruction::SET(0, Target8Bit::B)),
            0xc1 => Some(Instruction::SET(0, Target8Bit::C)),
            0xc2 => Some(Instruction::SET(0, Target8Bit::D)),
            0xc3 => Some(Instruction::SET(0, Target8Bit::E)),
            0xc4 => Some(Instruction::SET(0, Target8Bit::H)),
            0xc5 => Some(Instruction::SET(0, Target8Bit::L)),
            0xc6 => Some(Instruction::SET(0, Target8Bit::HLI)),
            0xc7 => Some(Instruction::SET(0, Target8Bit::A)),

            0xc8 => Some(Instruction::SET(1, Target8Bit::B)),
            0xc9 => Some(Instruction::SET(1, Target8Bit::C)),
            0xca => Some(Instruction::SET(1, Target8Bit::D)),
            0xcb => Some(Instruction::SET(1, Target8Bit::E)),
            0xcc => Some(Instruction::SET(1, Target8Bit::H)),
            0xcd => Some(Instruction::SET(1, Target8Bit::L)),
            0xce => Some(Instruction::SET(1, Target8Bit::HLI)),
            0xcf => Some(Instruction::SET(1, Target8Bit::A)),

            0xd0 => Some(Instruction::SET(2, Target8Bit::B)),
            0xd1 => Some(Instruction::SET(2, Target8Bit::C)),
            0xd2 => Some(Instruction::SET(2, Target8Bit::D)),
            0xd3 => Some(Instruction::SET(2, Target8Bit::E)),
            0xd4 => Some(Instruction::SET(2, Target8Bit::H)),
            0xd5 => Some(Instruction::SET(2, Target8Bit::L)),
            0xd6 => Some(Instruction::SET(2, Target8Bit::HLI)),
            0xd7 => Some(Instruction::SET(2, Target8Bit::A)),

            0xd8 => Some(Instruction::SET(3, Target8Bit::B)),
            0xd9 => Some(Instruction::SET(3, Target8Bit::C)),
            0xda => Some(Instruction::SET(3, Target8Bit::D)),
            0xdb => Some(Instruction::SET(3, Target8Bit::E)),
            0xdc => Some(Instruction::SET(3, Target8Bit::H)),
            0xdd => Some(Instruction::SET(3, Target8Bit::L)),
            0xde => Some(Instruction::SET(3, Target8Bit::HLI)),
            0xdf => Some(Instruction::SET(3, Target8Bit::A)),

            0xe0 => Some(Instruction::SET(4, Target8Bit::B)),
            0xe1 => Some(Instruction::SET(4, Target8Bit::C)),
            0xe2 => Some(Instruction::SET(4, Target8Bit::D)),
            0xe3 => Some(Instruction::SET(4, Target8Bit::E)),
            0xe4 => Some(Instruction::SET(4, Target8Bit::H)),
            0xe5 => Some(Instruction::SET(4, Target8Bit::L)),
            0xe6 => Some(Instruction::SET(4, Target8Bit::HLI)),
            0xe7 => Some(Instruction::SET(4, Target8Bit::A)),

            0xe8 => Some(Instruction::SET(5, Target8Bit::B)),
            0xe9 => Some(Instruction::SET(5, Target8Bit::C)),
            0xea => Some(Instruction::SET(5, Target8Bit::D)),
            0xeb => Some(Instruction::SET(5, Target8Bit::E)),
            0xec => Some(Instruction::SET(5, Target8Bit::H)),
            0xed => Some(Instruction::SET(5, Target8Bit::L)),
            0xee => Some(Instruction::SET(5, Target8Bit::HLI)),
            0xef => Some(Instruction::SET(5, Target8Bit::A)),

            0xf0 => Some(Instruction::SET(6, Target8Bit::B)),
            0xf1 => Some(Instruction::SET(6, Target8Bit::C)),
            0xf2 => Some(Instruction::SET(6, Target8Bit::D)),
            0xf3 => Some(Instruction::SET(6, Target8Bit::E)),
            0xf4 => Some(Instruction::SET(6, Target8Bit::H)),
            0xf5 => Some(Instruction::SET(6, Target8Bit::L)),
            0xf6 => Some(Instruction::SET(6, Target8Bit::HLI)),
            0xf7 => Some(Instruction::SET(6, Target8Bit::A)),

            0xf8 => Some(Instruction::SET(7, Target8Bit::B)),
            0xf9 => Some(Instruction::SET(7, Target8Bit::C)),
            0xfa => Some(Instruction::SET(7, Target8Bit::D)),
            0xfb => Some(Instruction::SET(7, Target8Bit::E)),
            0xfc => Some(Instruction::SET(7, Target8Bit::H)),
            0xfd => Some(Instruction::SET(7, Target8Bit::L)),
            0xfe => Some(Instruction::SET(7, Target8Bit::HLI)),
            0xff => Some(Instruction::SET(7, Target8Bit::A)),

            _ => {
                println!("[INS] Missing byte Instruction 0x{:x} with prefix", byte);
                None
            }
        }
    }

    fn from_byte_without_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),

            0x3c => Some(Instruction::INC(Target8Bit::A)),
            0x04 => Some(Instruction::INC(Target8Bit::B)),
            0x0c => Some(Instruction::INC(Target8Bit::C)),
            0x14 => Some(Instruction::INC(Target8Bit::D)),
            0x1c => Some(Instruction::INC(Target8Bit::E)),
            0x24 => Some(Instruction::INC(Target8Bit::H)),
            0x2c => Some(Instruction::INC(Target8Bit::L)),
            0x34 => Some(Instruction::INC(Target8Bit::HLI)),

            0x87 => Some(Instruction::ADD(Target8Bit::A)),
            0x80 => Some(Instruction::ADD(Target8Bit::B)),
            0x81 => Some(Instruction::ADD(Target8Bit::C)),
            0x82 => Some(Instruction::ADD(Target8Bit::D)),
            0x83 => Some(Instruction::ADD(Target8Bit::E)),
            0x84 => Some(Instruction::ADD(Target8Bit::H)),
            0x85 => Some(Instruction::ADD(Target8Bit::L)),
            0x86 => Some(Instruction::ADD(Target8Bit::HLI)),
            0xc6 => Some(Instruction::ADD(Target8Bit::D8)),

            0x8f => Some(Instruction::ADC(Target8Bit::A)),
            0x88 => Some(Instruction::ADC(Target8Bit::B)),
            0x89 => Some(Instruction::ADC(Target8Bit::C)),
            0x8A => Some(Instruction::ADC(Target8Bit::D)),
            0x8B => Some(Instruction::ADC(Target8Bit::E)),
            0x8C => Some(Instruction::ADC(Target8Bit::H)),
            0x8D => Some(Instruction::ADC(Target8Bit::L)),

            0x97 => Some(Instruction::SUB(Target8Bit::A)),
            0x90 => Some(Instruction::SUB(Target8Bit::B)),
            0x91 => Some(Instruction::SUB(Target8Bit::C)),
            0x92 => Some(Instruction::SUB(Target8Bit::D)),
            0x93 => Some(Instruction::SUB(Target8Bit::E)),
            0x94 => Some(Instruction::SUB(Target8Bit::H)),
            0x95 => Some(Instruction::SUB(Target8Bit::L)),

            0x9f => Some(Instruction::SBC(Target8Bit::A)),
            0x98 => Some(Instruction::SBC(Target8Bit::B)),
            0x99 => Some(Instruction::SBC(Target8Bit::C)),
            0x9a => Some(Instruction::SBC(Target8Bit::D)),
            0x9b => Some(Instruction::SBC(Target8Bit::E)),
            0x9c => Some(Instruction::SBC(Target8Bit::H)),
            0x9d => Some(Instruction::SBC(Target8Bit::L)),

            0xa7 => Some(Instruction::AND(Target8Bit::A)),
            0xa0 => Some(Instruction::AND(Target8Bit::B)),
            0xa1 => Some(Instruction::AND(Target8Bit::C)),
            0xa2 => Some(Instruction::AND(Target8Bit::D)),
            0xa3 => Some(Instruction::AND(Target8Bit::E)),
            0xa4 => Some(Instruction::AND(Target8Bit::H)),
            0xa6 => Some(Instruction::AND(Target8Bit::HLI)),
            0xe6 => Some(Instruction::AND(Target8Bit::D8)),

            0xb7 => Some(Instruction::OR(Target8Bit::A)),
            0xb0 => Some(Instruction::OR(Target8Bit::B)),
            0xb1 => Some(Instruction::OR(Target8Bit::C)),
            0xb2 => Some(Instruction::OR(Target8Bit::D)),
            0xb3 => Some(Instruction::OR(Target8Bit::E)),
            0xb4 => Some(Instruction::OR(Target8Bit::H)),
            0xb5 => Some(Instruction::OR(Target8Bit::L)),
            0xb6 => Some(Instruction::OR(Target8Bit::HLI)),
            0xf6 => Some(Instruction::OR(Target8Bit::D8)),

            0xaf => Some(Instruction::XOR(Target8Bit::A)),
            0xa8 => Some(Instruction::XOR(Target8Bit::B)),
            0xa9 => Some(Instruction::XOR(Target8Bit::C)),
            0xaa => Some(Instruction::XOR(Target8Bit::D)),
            0xab => Some(Instruction::XOR(Target8Bit::E)),
            0xac => Some(Instruction::XOR(Target8Bit::H)),
            0xad => Some(Instruction::XOR(Target8Bit::L)),
            0xae => Some(Instruction::XOR(Target8Bit::HLI)),
            0xee => Some(Instruction::XOR(Target8Bit::D8)),

            0xbf => Some(Instruction::CP(Target8Bit::A)),
            0xb8 => Some(Instruction::CP(Target8Bit::B)),
            0xb9 => Some(Instruction::CP(Target8Bit::C)),
            0xba => Some(Instruction::CP(Target8Bit::D)),
            0xbb => Some(Instruction::CP(Target8Bit::E)),
            0xbc => Some(Instruction::CP(Target8Bit::H)),
            0xbd => Some(Instruction::CP(Target8Bit::L)),
            0xbe => Some(Instruction::CP(Target8Bit::HLI)),
            0xfe => Some(Instruction::CP(Target8Bit::D8)),

            0x3d => Some(Instruction::DEC(Target8Bit::A)),
            0x05 => Some(Instruction::DEC(Target8Bit::B)),
            0x0d => Some(Instruction::DEC(Target8Bit::C)),
            0x15 => Some(Instruction::DEC(Target8Bit::D)),
            0x1d => Some(Instruction::DEC(Target8Bit::E)),
            0x25 => Some(Instruction::DEC(Target8Bit::H)),
            0x2d => Some(Instruction::DEC(Target8Bit::L)),
            0x35 => Some(Instruction::DEC(Target8Bit::HLI)),

            0xc2 => Some(Instruction::JP(JumpCondition::NotZero)),
            0xca => Some(Instruction::JP(JumpCondition::Zero)),
            0xd2 => Some(Instruction::JP(JumpCondition::NotCarry)),
            0xda => Some(Instruction::JP(JumpCondition::Carry)),

            0x20 => Some(Instruction::JR(JumpCondition::NotZero)),
            0x28 => Some(Instruction::JR(JumpCondition::Zero)),
            0x30 => Some(Instruction::JR(JumpCondition::NotCarry)),
            0x38 => Some(Instruction::JR(JumpCondition::Carry)),
            0x18 => Some(Instruction::JR(JumpCondition::Always)),

            0xe9 => Some(Instruction::JPI),

            0x06 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::D8)),
            0x0e => Some(Instruction::LD8(Target8Bit::C, Target8Bit::D8)),
            0x16 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::D8)),
            0x1e => Some(Instruction::LD8(Target8Bit::E, Target8Bit::D8)),
            0x26 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::D8)),
            0x2e => Some(Instruction::LD8(Target8Bit::L, Target8Bit::D8)),

            0x7f => Some(Instruction::LD8(Target8Bit::A, Target8Bit::A)),
            0x78 => Some(Instruction::LD8(Target8Bit::A, Target8Bit::B)),
            0x79 => Some(Instruction::LD8(Target8Bit::A, Target8Bit::C)),
            0x7a => Some(Instruction::LD8(Target8Bit::A, Target8Bit::D)),
            0x7b => Some(Instruction::LD8(Target8Bit::A, Target8Bit::E)),
            0x7c => Some(Instruction::LD8(Target8Bit::A, Target8Bit::H)),
            0x7d => Some(Instruction::LD8(Target8Bit::A, Target8Bit::L)),
            0x7e => Some(Instruction::LD8(Target8Bit::A, Target8Bit::HLI)),
            0x3e => Some(Instruction::LD8(Target8Bit::A, Target8Bit::D8)),

            0x40 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::B)),
            0x41 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::C)),
            0x42 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::D)),
            0x43 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::E)),
            0x44 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::H)),
            0x45 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::L)),
            0x46 => Some(Instruction::LD8(Target8Bit::B, Target8Bit::HLI)),

            0x48 => Some(Instruction::LD8(Target8Bit::C, Target8Bit::B)),
            0x49 => Some(Instruction::LD8(Target8Bit::C, Target8Bit::C)),
            0x4a => Some(Instruction::LD8(Target8Bit::C, Target8Bit::D)),
            0x4b => Some(Instruction::LD8(Target8Bit::C, Target8Bit::E)),
            0x4c => Some(Instruction::LD8(Target8Bit::C, Target8Bit::H)),
            0x4d => Some(Instruction::LD8(Target8Bit::C, Target8Bit::L)),
            0x4e => Some(Instruction::LD8(Target8Bit::C, Target8Bit::HLI)),

            0x50 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::B)),
            0x51 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::C)),
            0x52 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::D)),
            0x53 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::E)),
            0x54 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::H)),
            0x55 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::L)),
            0x56 => Some(Instruction::LD8(Target8Bit::D, Target8Bit::HLI)),

            0x58 => Some(Instruction::LD8(Target8Bit::E, Target8Bit::B)),
            0x59 => Some(Instruction::LD8(Target8Bit::E, Target8Bit::C)),
            0x5a => Some(Instruction::LD8(Target8Bit::E, Target8Bit::D)),
            0x5b => Some(Instruction::LD8(Target8Bit::E, Target8Bit::E)),
            0x5c => Some(Instruction::LD8(Target8Bit::E, Target8Bit::H)),
            0x5d => Some(Instruction::LD8(Target8Bit::E, Target8Bit::L)),
            0x5e => Some(Instruction::LD8(Target8Bit::E, Target8Bit::HLI)),

            0x60 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::B)),
            0x61 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::C)),
            0x62 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::D)),
            0x63 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::E)),
            0x64 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::H)),
            0x65 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::L)),
            0x66 => Some(Instruction::LD8(Target8Bit::H, Target8Bit::HLI)),

            0x68 => Some(Instruction::LD8(Target8Bit::L, Target8Bit::B)),
            0x69 => Some(Instruction::LD8(Target8Bit::L, Target8Bit::C)),
            0x6a => Some(Instruction::LD8(Target8Bit::L, Target8Bit::D)),
            0x6b => Some(Instruction::LD8(Target8Bit::L, Target8Bit::E)),
            0x6c => Some(Instruction::LD8(Target8Bit::L, Target8Bit::H)),
            0x6d => Some(Instruction::LD8(Target8Bit::L, Target8Bit::L)),
            0x6e => Some(Instruction::LD8(Target8Bit::L, Target8Bit::HLI)),

            0x70 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::B)),
            0x71 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::C)),
            0x72 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::D)),
            0x73 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::E)),
            0x74 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::H)),
            0x75 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::L)),
            0x36 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::D8)),
            0x77 => Some(Instruction::LD8(Target8Bit::HLI, Target8Bit::A)),

            0x01 => Some(Instruction::LD16(Target16Bit::BC, Source16Bit::D16)),
            0x11 => Some(Instruction::LD16(Target16Bit::DE, Source16Bit::D16)),
            0x21 => Some(Instruction::LD16(Target16Bit::HL, Source16Bit::D16)),
            0x31 => Some(Instruction::LD16(Target16Bit::SP, Source16Bit::D16)),

            0x0b => Some(Instruction::DEC16(Target16Bit::BC)),
            0x1b => Some(Instruction::DEC16(Target16Bit::DE)),
            0x2b => Some(Instruction::DEC16(Target16Bit::HL)),
            0x3b => Some(Instruction::DEC16(Target16Bit::SP)),

            0x09 => Some(Instruction::ADD16(Target16Bit::BC)),
            0x19 => Some(Instruction::ADD16(Target16Bit::DE)),
            0x29 => Some(Instruction::ADD16(Target16Bit::HL)),
            0x39 => Some(Instruction::ADD16(Target16Bit::SP)),

            0x03 => Some(Instruction::INC16(Target16Bit::BC)),
            0x13 => Some(Instruction::INC16(Target16Bit::DE)),
            0x23 => Some(Instruction::INC16(Target16Bit::HL)),
            0x33 => Some(Instruction::INC16(Target16Bit::SP)),

            0x32 => Some(Instruction::LDD),
            0xe2 => Some(Instruction::LDC),
            0xe0 => Some(Instruction::LDHA),

            _ => {
                eprintln!("[INS] Missing byte Instruction 0x{:x}", byte);
                None
            }
        }
    }
}
