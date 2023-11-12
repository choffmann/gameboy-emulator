#[derive(Debug)]
pub enum Instruction {
    // Arithmetic Instructions
    ADD(Logic8BitRegister),
    ADC(Logic8BitRegister),
    SUB(Logic8BitRegister),
    SBC(Logic8BitRegister),
    INC(Logic8BitRegister),
    DEC(Logic8BitRegister),
    AND(Logic8BitRegister),
    OR(Logic8BitRegister),
    XOR(Logic8BitRegister),
    CP(Logic8BitRegister),

    // Jump Instructions
    JP(JumpCondition),
    JR(JumpCondition),
    JPI
}

#[derive(Debug)]
pub enum Logic8BitRegister {
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
pub enum Logic16BitRegister {
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
                println!("Missing byte Instruction 0x{:x}", byte);
                None
            }
        }
    }

    fn from_byte_without_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            0x3c => Some(Instruction::INC(Logic8BitRegister::A)),
            0x04 => Some(Instruction::INC(Logic8BitRegister::B)),
            0x0c => Some(Instruction::INC(Logic8BitRegister::C)),
            0x14 => Some(Instruction::INC(Logic8BitRegister::D)),
            0x1c => Some(Instruction::INC(Logic8BitRegister::E)),
            0x24 => Some(Instruction::INC(Logic8BitRegister::H)),
            0x2c => Some(Instruction::INC(Logic8BitRegister::L)),
            0x34 => Some(Instruction::INC(Logic8BitRegister::HLI)),

            0x87 => Some(Instruction::ADD(Logic8BitRegister::A)),
            0x80 => Some(Instruction::ADD(Logic8BitRegister::B)),
            0x81 => Some(Instruction::ADD(Logic8BitRegister::C)),
            0x82 => Some(Instruction::ADD(Logic8BitRegister::D)),
            0x83 => Some(Instruction::ADD(Logic8BitRegister::E)),
            0x84 => Some(Instruction::ADD(Logic8BitRegister::H)),
            0x85 => Some(Instruction::ADD(Logic8BitRegister::L)),
            0x86 => Some(Instruction::ADD(Logic8BitRegister::HLI)),
            0xc6 => Some(Instruction::ADD(Logic8BitRegister::D8)),

            0x8f => Some(Instruction::ADC(Logic8BitRegister::A)),
            0x88 => Some(Instruction::ADC(Logic8BitRegister::B)),
            0x89 => Some(Instruction::ADC(Logic8BitRegister::C)),
            0x8A => Some(Instruction::ADC(Logic8BitRegister::D)),
            0x8B => Some(Instruction::ADC(Logic8BitRegister::E)),
            0x8C => Some(Instruction::ADC(Logic8BitRegister::H)),
            0x8D => Some(Instruction::ADC(Logic8BitRegister::L)),

            0x97 => Some(Instruction::SUB(Logic8BitRegister::A)),
            0x90 => Some(Instruction::SUB(Logic8BitRegister::B)),
            0x91 => Some(Instruction::SUB(Logic8BitRegister::C)),
            0x92 => Some(Instruction::SUB(Logic8BitRegister::D)),
            0x93 => Some(Instruction::SUB(Logic8BitRegister::E)),
            0x94 => Some(Instruction::SUB(Logic8BitRegister::H)),
            0x95 => Some(Instruction::SUB(Logic8BitRegister::L)),

            0x9f => Some(Instruction::SBC(Logic8BitRegister::A)),
            0x98 => Some(Instruction::SBC(Logic8BitRegister::B)),
            0x99 => Some(Instruction::SBC(Logic8BitRegister::C)),
            0x9a => Some(Instruction::SBC(Logic8BitRegister::D)),
            0x9b => Some(Instruction::SBC(Logic8BitRegister::E)),
            0x9c => Some(Instruction::SBC(Logic8BitRegister::H)),
            0x9d => Some(Instruction::SBC(Logic8BitRegister::L)),

            0xa7 => Some(Instruction::AND(Logic8BitRegister::A)),
            0xa0 => Some(Instruction::AND(Logic8BitRegister::B)),
            0xa1 => Some(Instruction::AND(Logic8BitRegister::C)),
            0xa2 => Some(Instruction::AND(Logic8BitRegister::D)),
            0xa3 => Some(Instruction::AND(Logic8BitRegister::E)),
            0xa4 => Some(Instruction::AND(Logic8BitRegister::H)),
            0xa6 => Some(Instruction::AND(Logic8BitRegister::HLI)),
            0xe6 => Some(Instruction::AND(Logic8BitRegister::D8)),

            0xb7 => Some(Instruction::OR(Logic8BitRegister::A)),
            0xb0 => Some(Instruction::OR(Logic8BitRegister::B)),
            0xb1 => Some(Instruction::OR(Logic8BitRegister::C)),
            0xb2 => Some(Instruction::OR(Logic8BitRegister::D)),
            0xb3 => Some(Instruction::OR(Logic8BitRegister::E)),
            0xb4 => Some(Instruction::OR(Logic8BitRegister::H)),
            0xb5 => Some(Instruction::OR(Logic8BitRegister::L)),
            0xb6 => Some(Instruction::OR(Logic8BitRegister::HLI)),
            0xf6 => Some(Instruction::OR(Logic8BitRegister::D8)),

            0xaf => Some(Instruction::XOR(Logic8BitRegister::A)),
            0xa8 => Some(Instruction::XOR(Logic8BitRegister::B)),
            0xa9 => Some(Instruction::XOR(Logic8BitRegister::C)),
            0xaa => Some(Instruction::XOR(Logic8BitRegister::D)),
            0xab => Some(Instruction::XOR(Logic8BitRegister::E)),
            0xac => Some(Instruction::XOR(Logic8BitRegister::H)),
            0xad => Some(Instruction::XOR(Logic8BitRegister::L)),
            0xae => Some(Instruction::XOR(Logic8BitRegister::HLI)),
            0xee => Some(Instruction::XOR(Logic8BitRegister::D8)),

            0xbf => Some(Instruction::CP(Logic8BitRegister::A)),
            0xb8 => Some(Instruction::CP(Logic8BitRegister::B)),
            0xb9 => Some(Instruction::CP(Logic8BitRegister::C)),
            0xba => Some(Instruction::CP(Logic8BitRegister::D)),
            0xbb => Some(Instruction::CP(Logic8BitRegister::E)),
            0xbc => Some(Instruction::CP(Logic8BitRegister::H)),
            0xbd => Some(Instruction::CP(Logic8BitRegister::L)),
            0xbe => Some(Instruction::CP(Logic8BitRegister::HLI)),
            0xfe => Some(Instruction::CP(Logic8BitRegister::D8)),

            0x3d => Some(Instruction::DEC(Logic8BitRegister::A)),
            0x05 => Some(Instruction::DEC(Logic8BitRegister::B)),
            0x0d => Some(Instruction::DEC(Logic8BitRegister::C)),
            0x15 => Some(Instruction::DEC(Logic8BitRegister::D)),
            0x1d => Some(Instruction::DEC(Logic8BitRegister::E)),
            0x25 => Some(Instruction::DEC(Logic8BitRegister::H)),
            0x2d => Some(Instruction::DEC(Logic8BitRegister::L)),
            0x35 => Some(Instruction::DEC(Logic8BitRegister::HLI)),

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

            _ => {
                println!("Missing byte Instruction 0x{:x}", byte);
                None
            }
        }
    }
}
