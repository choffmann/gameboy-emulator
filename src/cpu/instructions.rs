#[derive(Debug)]
pub enum Instruction {
    // Arithmetic Instructions
    ADD(Register8Bit),
    ADC(Register8Bit),
    SUB(Register8Bit),
    SBC(Register8Bit),
    INC(Register8Bit),
    DEC(Register8Bit),
    AND(Register8Bit),
    OR(Register8Bit),
    XOR(Register8Bit),
    CP(Register8Bit),
    NOP,

    // Jump Instructions
    JP(JumpCondition),
    JR(JumpCondition),
    JPI,

    // Load Instructions
    LD(Register8Bit, Register8Bit),
}

#[derive(Debug)]
pub enum Register8Bit {
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

impl ToString for Register8Bit {
    fn to_string(&self) -> String {
        return String::from(match self {
            Register8Bit::A => "Register_A",
            Register8Bit::B => "Register_B",
            Register8Bit::C => "Register_C",
            Register8Bit::D => "Register_D",
            Register8Bit::E => "Register_E",
            Register8Bit::H => "Register_H",
            Register8Bit::L => "Register_L",
            Register8Bit::D8 => "Register_D8",
            Register8Bit::HLI => "Register_HLI",
        });
    }
}

#[derive(Debug)]
pub enum Register16Bit {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
pub enum JumpCondition {
    NotZero,
    NotCarry,
    Zero,
    Carry,
    Always,
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
            _ => {
                println!("[INS] Missing byte Instruction 0x{:x}", byte);
                None
           }
        }
    }

    fn from_byte_without_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),

            0x3c => Some(Instruction::INC(Register8Bit::A)),
            0x04 => Some(Instruction::INC(Register8Bit::B)),
            0x0c => Some(Instruction::INC(Register8Bit::C)),
            0x14 => Some(Instruction::INC(Register8Bit::D)),
            0x1c => Some(Instruction::INC(Register8Bit::E)),
            0x24 => Some(Instruction::INC(Register8Bit::H)),
            0x2c => Some(Instruction::INC(Register8Bit::L)),
            0x34 => Some(Instruction::INC(Register8Bit::HLI)),

            0x87 => Some(Instruction::ADD(Register8Bit::A)),
            0x80 => Some(Instruction::ADD(Register8Bit::B)),
            0x81 => Some(Instruction::ADD(Register8Bit::C)),
            0x82 => Some(Instruction::ADD(Register8Bit::D)),
            0x83 => Some(Instruction::ADD(Register8Bit::E)),
            0x84 => Some(Instruction::ADD(Register8Bit::H)),
            0x85 => Some(Instruction::ADD(Register8Bit::L)),
            0x86 => Some(Instruction::ADD(Register8Bit::HLI)),
            0xc6 => Some(Instruction::ADD(Register8Bit::D8)),

            0x8f => Some(Instruction::ADC(Register8Bit::A)),
            0x88 => Some(Instruction::ADC(Register8Bit::B)),
            0x89 => Some(Instruction::ADC(Register8Bit::C)),
            0x8A => Some(Instruction::ADC(Register8Bit::D)),
            0x8B => Some(Instruction::ADC(Register8Bit::E)),
            0x8C => Some(Instruction::ADC(Register8Bit::H)),
            0x8D => Some(Instruction::ADC(Register8Bit::L)),

            0x97 => Some(Instruction::SUB(Register8Bit::A)),
            0x90 => Some(Instruction::SUB(Register8Bit::B)),
            0x91 => Some(Instruction::SUB(Register8Bit::C)),
            0x92 => Some(Instruction::SUB(Register8Bit::D)),
            0x93 => Some(Instruction::SUB(Register8Bit::E)),
            0x94 => Some(Instruction::SUB(Register8Bit::H)),
            0x95 => Some(Instruction::SUB(Register8Bit::L)),

            0x9f => Some(Instruction::SBC(Register8Bit::A)),
            0x98 => Some(Instruction::SBC(Register8Bit::B)),
            0x99 => Some(Instruction::SBC(Register8Bit::C)),
            0x9a => Some(Instruction::SBC(Register8Bit::D)),
            0x9b => Some(Instruction::SBC(Register8Bit::E)),
            0x9c => Some(Instruction::SBC(Register8Bit::H)),
            0x9d => Some(Instruction::SBC(Register8Bit::L)),

            0xa7 => Some(Instruction::AND(Register8Bit::A)),
            0xa0 => Some(Instruction::AND(Register8Bit::B)),
            0xa1 => Some(Instruction::AND(Register8Bit::C)),
            0xa2 => Some(Instruction::AND(Register8Bit::D)),
            0xa3 => Some(Instruction::AND(Register8Bit::E)),
            0xa4 => Some(Instruction::AND(Register8Bit::H)),
            0xa6 => Some(Instruction::AND(Register8Bit::HLI)),
            0xe6 => Some(Instruction::AND(Register8Bit::D8)),

            0xb7 => Some(Instruction::OR(Register8Bit::A)),
            0xb0 => Some(Instruction::OR(Register8Bit::B)),
            0xb1 => Some(Instruction::OR(Register8Bit::C)),
            0xb2 => Some(Instruction::OR(Register8Bit::D)),
            0xb3 => Some(Instruction::OR(Register8Bit::E)),
            0xb4 => Some(Instruction::OR(Register8Bit::H)),
            0xb5 => Some(Instruction::OR(Register8Bit::L)),
            0xb6 => Some(Instruction::OR(Register8Bit::HLI)),
            0xf6 => Some(Instruction::OR(Register8Bit::D8)),

            0xaf => Some(Instruction::XOR(Register8Bit::A)),
            0xa8 => Some(Instruction::XOR(Register8Bit::B)),
            0xa9 => Some(Instruction::XOR(Register8Bit::C)),
            0xaa => Some(Instruction::XOR(Register8Bit::D)),
            0xab => Some(Instruction::XOR(Register8Bit::E)),
            0xac => Some(Instruction::XOR(Register8Bit::H)),
            0xad => Some(Instruction::XOR(Register8Bit::L)),
            0xae => Some(Instruction::XOR(Register8Bit::HLI)),
            0xee => Some(Instruction::XOR(Register8Bit::D8)),

            0xbf => Some(Instruction::CP(Register8Bit::A)),
            0xb8 => Some(Instruction::CP(Register8Bit::B)),
            0xb9 => Some(Instruction::CP(Register8Bit::C)),
            0xba => Some(Instruction::CP(Register8Bit::D)),
            0xbb => Some(Instruction::CP(Register8Bit::E)),
            0xbc => Some(Instruction::CP(Register8Bit::H)),
            0xbd => Some(Instruction::CP(Register8Bit::L)),
            0xbe => Some(Instruction::CP(Register8Bit::HLI)),
            0xfe => Some(Instruction::CP(Register8Bit::D8)),

            0x3d => Some(Instruction::DEC(Register8Bit::A)),
            0x05 => Some(Instruction::DEC(Register8Bit::B)),
            0x0d => Some(Instruction::DEC(Register8Bit::C)),
            0x15 => Some(Instruction::DEC(Register8Bit::D)),
            0x1d => Some(Instruction::DEC(Register8Bit::E)),
            0x25 => Some(Instruction::DEC(Register8Bit::H)),
            0x2d => Some(Instruction::DEC(Register8Bit::L)),
            0x35 => Some(Instruction::DEC(Register8Bit::HLI)),

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

            0x06 => Some(Instruction::LD(Register8Bit::B, Register8Bit::D8)),
            0x0e => Some(Instruction::LD(Register8Bit::C, Register8Bit::D8)),
            0x16 => Some(Instruction::LD(Register8Bit::D, Register8Bit::D8)),
            0x1e => Some(Instruction::LD(Register8Bit::E, Register8Bit::D8)),
            0x26 => Some(Instruction::LD(Register8Bit::H, Register8Bit::D8)),
            0x2e => Some(Instruction::LD(Register8Bit::L, Register8Bit::D8)),

            0x7f => Some(Instruction::LD(Register8Bit::A, Register8Bit::A)),
            0x78 => Some(Instruction::LD(Register8Bit::A, Register8Bit::B)),
            0x79 => Some(Instruction::LD(Register8Bit::A, Register8Bit::C)),
            0x7a => Some(Instruction::LD(Register8Bit::A, Register8Bit::D)),
            0x7b => Some(Instruction::LD(Register8Bit::A, Register8Bit::E)),
            0x7c => Some(Instruction::LD(Register8Bit::A, Register8Bit::H)),
            0x7d => Some(Instruction::LD(Register8Bit::A, Register8Bit::L)),
            0x7e => Some(Instruction::LD(Register8Bit::A, Register8Bit::HLI)),

            0x40 => Some(Instruction::LD(Register8Bit::B, Register8Bit::B)),
            0x41 => Some(Instruction::LD(Register8Bit::B, Register8Bit::C)),
            0x42 => Some(Instruction::LD(Register8Bit::B, Register8Bit::D)),
            0x43 => Some(Instruction::LD(Register8Bit::B, Register8Bit::E)),
            0x44 => Some(Instruction::LD(Register8Bit::B, Register8Bit::H)),
            0x45 => Some(Instruction::LD(Register8Bit::B, Register8Bit::L)),
            0x46 => Some(Instruction::LD(Register8Bit::B, Register8Bit::HLI)),

            0x48 => Some(Instruction::LD(Register8Bit::C, Register8Bit::B)),
            0x49 => Some(Instruction::LD(Register8Bit::C, Register8Bit::C)),
            0x4a => Some(Instruction::LD(Register8Bit::C, Register8Bit::D)),
            0x4b => Some(Instruction::LD(Register8Bit::C, Register8Bit::E)),
            0x4c => Some(Instruction::LD(Register8Bit::C, Register8Bit::H)),
            0x4d => Some(Instruction::LD(Register8Bit::C, Register8Bit::L)),
            0x4e => Some(Instruction::LD(Register8Bit::C, Register8Bit::HLI)),

            0x50 => Some(Instruction::LD(Register8Bit::D, Register8Bit::B)),
            0x51 => Some(Instruction::LD(Register8Bit::D, Register8Bit::C)),
            0x52 => Some(Instruction::LD(Register8Bit::D, Register8Bit::D)),
            0x53 => Some(Instruction::LD(Register8Bit::D, Register8Bit::E)),
            0x54 => Some(Instruction::LD(Register8Bit::D, Register8Bit::H)),
            0x55 => Some(Instruction::LD(Register8Bit::D, Register8Bit::L)),
            0x56 => Some(Instruction::LD(Register8Bit::D, Register8Bit::HLI)),

            0x58 => Some(Instruction::LD(Register8Bit::E, Register8Bit::B)),
            0x59 => Some(Instruction::LD(Register8Bit::E, Register8Bit::C)),
            0x5a => Some(Instruction::LD(Register8Bit::E, Register8Bit::D)),
            0x5b => Some(Instruction::LD(Register8Bit::E, Register8Bit::E)),
            0x5c => Some(Instruction::LD(Register8Bit::E, Register8Bit::H)),
            0x5d => Some(Instruction::LD(Register8Bit::E, Register8Bit::L)),
            0x5e => Some(Instruction::LD(Register8Bit::E, Register8Bit::HLI)),

            0x60 => Some(Instruction::LD(Register8Bit::H, Register8Bit::B)),
            0x61 => Some(Instruction::LD(Register8Bit::H, Register8Bit::C)),
            0x62 => Some(Instruction::LD(Register8Bit::H, Register8Bit::D)),
            0x63 => Some(Instruction::LD(Register8Bit::H, Register8Bit::E)),
            0x64 => Some(Instruction::LD(Register8Bit::H, Register8Bit::H)),
            0x65 => Some(Instruction::LD(Register8Bit::H, Register8Bit::L)),
            0x66 => Some(Instruction::LD(Register8Bit::H, Register8Bit::HLI)),

            0x68 => Some(Instruction::LD(Register8Bit::L, Register8Bit::B)),
            0x69 => Some(Instruction::LD(Register8Bit::L, Register8Bit::C)),
            0x6a => Some(Instruction::LD(Register8Bit::L, Register8Bit::D)),
            0x6b => Some(Instruction::LD(Register8Bit::L, Register8Bit::E)),
            0x6c => Some(Instruction::LD(Register8Bit::L, Register8Bit::H)),
            0x6d => Some(Instruction::LD(Register8Bit::L, Register8Bit::L)),
            0x6e => Some(Instruction::LD(Register8Bit::L, Register8Bit::HLI)),

            0x70 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::B)),
            0x71 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::C)),
            0x72 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::D)),
            0x73 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::E)),
            0x74 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::H)),
            0x75 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::L)),
            0x36 => Some(Instruction::LD(Register8Bit::HLI, Register8Bit::D8)),

            _ => {
                eprintln!("[INS] Missing byte Instruction 0x{:x}", byte);
                None
            }
        }
    }
}
