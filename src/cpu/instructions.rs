#[derive(Debug)]
pub enum Instruction {
    // Arithemtic Instructions
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    INC(ArithmeticTarget),
}

#[derive(Debug)]
pub enum ArithmeticTarget {
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

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x3c => Some(Instruction::INC(ArithmeticTarget::A)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x0c => Some(Instruction::INC(ArithmeticTarget::C)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x1c => Some(Instruction::INC(ArithmeticTarget::E)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x2c => Some(Instruction::INC(ArithmeticTarget::L)),

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0xc6 => Some(Instruction::ADD(ArithmeticTarget::D8)),

            0x8f => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),

            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),

            0x9f => Some(Instruction::SBC(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SBC(ArithmeticTarget::L)),

            _ => {
                println!("Missing byte Instruction 0x{:x}", byte);
                None
            }
        }
    }
}
