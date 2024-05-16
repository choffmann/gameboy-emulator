use super::registers::Register;

#[derive(Debug)]
pub enum FlagCondition {
    NZ, // Not zero
    Z,  // Zero
    NC, // Not carry
    C,  // Carry
}

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
    Push(Register),           // Push register onto stack
    Pop(Register),            // Pop register from stack
    Add(Register),            // Add register to A
    Add16(Register),          // Add register to HL
    Add16SP,                  // Add SP to HL
    Adc(Register),            // Add register to A with carry
    Sub(Register),            // Subtract register from A
    Sbc(Register),            // Subtract register from A with carry
    And(Register),            // Logical AND register with A
    Or(Register),             // Logical OR register with A
    Xor(Register),            // Logical XOR register with A
    Cp(Register),             // Compare register with A
    Inc(Register),            // Increment register
    Inc16(Register),          // Increment register
    Dec(Register),            // Decrement register
    Dec16(Register),          // Decrement register

    Swap(Register), // Swap upper and lower nibbles of register
    DAA,            // Decimal adjust register A
    CPL,            // Complement A
    CCF,            // Complement carry flag
    SCF,            // Set carry flag
    HALT,           // Halt CPU
    STOP,           // Stop CPU
    DI,             // Disable interrupts
    EI,             // Enable interrupts

    RLCA,          // Rotate A left
    RLA,           // Rotate A left through carry
    RRCA,          // Rotate A right
    RRA,           // Rotate A right through carry
    RLC(Register), // Rotate register left
    RL(Register),  // Rotate register left through carry
    RRC(Register), // Rotate register right
    RR(Register),  // Rotate register right through carry
    SLA(Register), // Shift register left
    SRA(Register), // Shift register right
    SRL(Register), // Shift register right

    Bit(u8, Register), // Test bit in register
    Set(u8, Register), // Set bit in register
    Res(u8, Register), // Reset bit in register

    Jp,                    // Jump to address
    JpCond(FlagCondition), // Conditional jump to address
    JpHL,                       // Jump to HL
    Jr,                     // Jump relative
    JrCond(FlagCondition),  // Conditional jump relative

    Call,                    // Call address
    CallCond(FlagCondition), // Conditional call address

    Rst(u8),                // Restart to address
    Ret,                    // Return from subroutine
    RetCond(FlagCondition), // Conditional return from subroutine
    Reti,                   // Return from interrupt
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
            0x37 => Some(Instruction::Swap(Register::A)),
            0x30 => Some(Instruction::Swap(Register::B)),
            0x31 => Some(Instruction::Swap(Register::C)),
            0x32 => Some(Instruction::Swap(Register::D)),
            0x33 => Some(Instruction::Swap(Register::E)),
            0x34 => Some(Instruction::Swap(Register::H)),
            0x35 => Some(Instruction::Swap(Register::L)),
            0x36 => Some(Instruction::Swap(Register::HL)),

            0x07 => Some(Instruction::RLC(Register::A)),
            0x00 => Some(Instruction::RLC(Register::B)),
            0x01 => Some(Instruction::RLC(Register::C)),
            0x02 => Some(Instruction::RLC(Register::D)),
            0x03 => Some(Instruction::RLC(Register::E)),
            0x04 => Some(Instruction::RLC(Register::H)),
            0x05 => Some(Instruction::RLC(Register::L)),
            0x06 => Some(Instruction::RLC(Register::HL)),

            0x17 => Some(Instruction::RL(Register::A)),
            0x10 => Some(Instruction::RL(Register::B)),
            0x11 => Some(Instruction::RL(Register::C)),
            0x12 => Some(Instruction::RL(Register::D)),
            0x13 => Some(Instruction::RL(Register::E)),
            0x14 => Some(Instruction::RL(Register::H)),
            0x15 => Some(Instruction::RL(Register::L)),
            0x16 => Some(Instruction::RL(Register::HL)),

            0x0F => Some(Instruction::RRC(Register::A)),
            0x08 => Some(Instruction::RRC(Register::B)),
            0x09 => Some(Instruction::RRC(Register::C)),
            0x0A => Some(Instruction::RRC(Register::D)),
            0x0B => Some(Instruction::RRC(Register::E)),
            0x0C => Some(Instruction::RRC(Register::H)),
            0x0D => Some(Instruction::RRC(Register::L)),
            0x0E => Some(Instruction::RRC(Register::HL)),

            0x1F => Some(Instruction::RR(Register::A)),
            0x18 => Some(Instruction::RR(Register::B)),
            0x19 => Some(Instruction::RR(Register::C)),
            0x1A => Some(Instruction::RR(Register::D)),
            0x1B => Some(Instruction::RR(Register::E)),
            0x1C => Some(Instruction::RR(Register::H)),
            0x1D => Some(Instruction::RR(Register::L)),
            0x1E => Some(Instruction::RR(Register::HL)),

            0x27 => Some(Instruction::SLA(Register::A)),
            0x20 => Some(Instruction::SLA(Register::B)),
            0x21 => Some(Instruction::SLA(Register::C)),
            0x22 => Some(Instruction::SLA(Register::D)),
            0x23 => Some(Instruction::SLA(Register::E)),
            0x24 => Some(Instruction::SLA(Register::H)),
            0x25 => Some(Instruction::SLA(Register::L)),
            0x26 => Some(Instruction::SLA(Register::HL)),

            0x2F => Some(Instruction::SRA(Register::A)),
            0x28 => Some(Instruction::SRA(Register::B)),
            0x29 => Some(Instruction::SRA(Register::C)),
            0x2A => Some(Instruction::SRA(Register::D)),
            0x2B => Some(Instruction::SRA(Register::E)),
            0x2C => Some(Instruction::SRA(Register::H)),
            0x2D => Some(Instruction::SRA(Register::L)),
            0x2E => Some(Instruction::SRA(Register::HL)),

            0x3F => Some(Instruction::SRL(Register::A)),
            0x38 => Some(Instruction::SRL(Register::B)),
            0x39 => Some(Instruction::SRL(Register::C)),
            0x3A => Some(Instruction::SRL(Register::D)),
            0x3B => Some(Instruction::SRL(Register::E)),
            0x3C => Some(Instruction::SRL(Register::H)),
            0x3D => Some(Instruction::SRL(Register::L)),
            0x3E => Some(Instruction::SRL(Register::HL)),

            0x40 => Some(Instruction::Bit(0, Register::B)),
            0x41 => Some(Instruction::Bit(0, Register::C)),
            0x42 => Some(Instruction::Bit(0, Register::D)),
            0x43 => Some(Instruction::Bit(0, Register::E)),
            0x44 => Some(Instruction::Bit(0, Register::H)),
            0x45 => Some(Instruction::Bit(0, Register::L)),
            0x46 => Some(Instruction::Bit(0, Register::HL)),

            0x47 => Some(Instruction::Bit(1, Register::B)),
            0x48 => Some(Instruction::Bit(1, Register::C)),
            0x49 => Some(Instruction::Bit(1, Register::D)),
            0x4A => Some(Instruction::Bit(1, Register::E)),
            0x4B => Some(Instruction::Bit(1, Register::H)),
            0x4C => Some(Instruction::Bit(1, Register::L)),
            0x4D => Some(Instruction::Bit(1, Register::HL)),
            0x4E => Some(Instruction::Bit(1, Register::A)),

            0x50 => Some(Instruction::Bit(2, Register::B)),
            0x51 => Some(Instruction::Bit(2, Register::C)),
            0x52 => Some(Instruction::Bit(2, Register::D)),
            0x53 => Some(Instruction::Bit(2, Register::E)),
            0x54 => Some(Instruction::Bit(2, Register::H)),
            0x55 => Some(Instruction::Bit(2, Register::L)),
            0x56 => Some(Instruction::Bit(2, Register::HL)),
            0x57 => Some(Instruction::Bit(2, Register::A)),

            0x58 => Some(Instruction::Bit(3, Register::B)),
            0x59 => Some(Instruction::Bit(3, Register::C)),
            0x5A => Some(Instruction::Bit(3, Register::D)),
            0x5B => Some(Instruction::Bit(3, Register::E)),
            0x5C => Some(Instruction::Bit(3, Register::H)),
            0x5D => Some(Instruction::Bit(3, Register::L)),
            0x5E => Some(Instruction::Bit(3, Register::HL)),
            0x5F => Some(Instruction::Bit(3, Register::A)),

            0x60 => Some(Instruction::Bit(4, Register::B)),
            0x61 => Some(Instruction::Bit(4, Register::C)),
            0x62 => Some(Instruction::Bit(4, Register::D)),
            0x63 => Some(Instruction::Bit(4, Register::E)),
            0x64 => Some(Instruction::Bit(4, Register::H)),
            0x65 => Some(Instruction::Bit(4, Register::L)),
            0x66 => Some(Instruction::Bit(4, Register::HL)),
            0x67 => Some(Instruction::Bit(4, Register::A)),

            0x68 => Some(Instruction::Bit(5, Register::B)),
            0x69 => Some(Instruction::Bit(5, Register::C)),
            0x6A => Some(Instruction::Bit(5, Register::D)),
            0x6B => Some(Instruction::Bit(5, Register::E)),
            0x6C => Some(Instruction::Bit(5, Register::H)),
            0x6D => Some(Instruction::Bit(5, Register::L)),
            0x6E => Some(Instruction::Bit(5, Register::HL)),
            0x6F => Some(Instruction::Bit(5, Register::A)),

            0x70 => Some(Instruction::Bit(6, Register::B)),
            0x71 => Some(Instruction::Bit(6, Register::C)),
            0x72 => Some(Instruction::Bit(6, Register::D)),
            0x73 => Some(Instruction::Bit(6, Register::E)),
            0x74 => Some(Instruction::Bit(6, Register::H)),
            0x75 => Some(Instruction::Bit(6, Register::L)),
            0x76 => Some(Instruction::Bit(6, Register::HL)),
            0x77 => Some(Instruction::Bit(6, Register::A)),

            0x78 => Some(Instruction::Bit(7, Register::B)),
            0x79 => Some(Instruction::Bit(7, Register::C)),
            0x7A => Some(Instruction::Bit(7, Register::D)),
            0x7B => Some(Instruction::Bit(7, Register::E)),
            0x7C => Some(Instruction::Bit(7, Register::H)),
            0x7D => Some(Instruction::Bit(7, Register::L)),
            0x7E => Some(Instruction::Bit(7, Register::HL)),
            0x7F => Some(Instruction::Bit(7, Register::A)),

            0xC0 => Some(Instruction::Set(0, Register::B)),
            0xC1 => Some(Instruction::Set(0, Register::C)),
            0xC2 => Some(Instruction::Set(0, Register::D)),
            0xC3 => Some(Instruction::Set(0, Register::E)),
            0xC4 => Some(Instruction::Set(0, Register::H)),
            0xC5 => Some(Instruction::Set(0, Register::L)),
            0xC6 => Some(Instruction::Set(0, Register::HL)),
            0xC7 => Some(Instruction::Set(0, Register::A)),

            0xC8 => Some(Instruction::Set(1, Register::B)),
            0xC9 => Some(Instruction::Set(1, Register::C)),
            0xCA => Some(Instruction::Set(1, Register::D)),
            0xCB => Some(Instruction::Set(1, Register::E)),
            0xCC => Some(Instruction::Set(1, Register::H)),
            0xCD => Some(Instruction::Set(1, Register::L)),
            0xCE => Some(Instruction::Set(1, Register::HL)),
            0xCF => Some(Instruction::Set(1, Register::A)),

            0xD0 => Some(Instruction::Set(2, Register::B)),
            0xD1 => Some(Instruction::Set(2, Register::C)),
            0xD2 => Some(Instruction::Set(2, Register::D)),
            0xD3 => Some(Instruction::Set(2, Register::E)),
            0xD4 => Some(Instruction::Set(2, Register::H)),
            0xD5 => Some(Instruction::Set(2, Register::L)),
            0xD6 => Some(Instruction::Set(2, Register::HL)),
            0xD7 => Some(Instruction::Set(2, Register::A)),

            0xD8 => Some(Instruction::Set(3, Register::B)),
            0xD9 => Some(Instruction::Set(3, Register::C)),
            0xDA => Some(Instruction::Set(3, Register::D)),
            0xDB => Some(Instruction::Set(3, Register::E)),
            0xDC => Some(Instruction::Set(3, Register::H)),
            0xDD => Some(Instruction::Set(3, Register::L)),
            0xDE => Some(Instruction::Set(3, Register::HL)),
            0xDF => Some(Instruction::Set(3, Register::A)),

            0xE0 => Some(Instruction::Set(4, Register::B)),
            0xE1 => Some(Instruction::Set(4, Register::C)),
            0xE2 => Some(Instruction::Set(4, Register::D)),
            0xE3 => Some(Instruction::Set(4, Register::E)),
            0xE4 => Some(Instruction::Set(4, Register::H)),
            0xE5 => Some(Instruction::Set(4, Register::L)),
            0xE6 => Some(Instruction::Set(4, Register::HL)),
            0xE7 => Some(Instruction::Set(4, Register::A)),

            0xE8 => Some(Instruction::Set(5, Register::B)),
            0xE9 => Some(Instruction::Set(5, Register::C)),
            0xEA => Some(Instruction::Set(5, Register::D)),
            0xEB => Some(Instruction::Set(5, Register::E)),
            0xEC => Some(Instruction::Set(5, Register::H)),
            0xED => Some(Instruction::Set(5, Register::L)),
            0xEE => Some(Instruction::Set(5, Register::HL)),
            0xEF => Some(Instruction::Set(5, Register::A)),

            0xF0 => Some(Instruction::Set(6, Register::B)),
            0xF1 => Some(Instruction::Set(6, Register::C)),
            0xF2 => Some(Instruction::Set(6, Register::D)),
            0xF3 => Some(Instruction::Set(6, Register::E)),
            0xF4 => Some(Instruction::Set(6, Register::H)),
            0xF5 => Some(Instruction::Set(6, Register::L)),
            0xF6 => Some(Instruction::Set(6, Register::HL)),
            0xF7 => Some(Instruction::Set(6, Register::A)),

            0xF8 => Some(Instruction::Set(7, Register::B)),
            0xF9 => Some(Instruction::Set(7, Register::C)),
            0xFA => Some(Instruction::Set(7, Register::D)),
            0xFB => Some(Instruction::Set(7, Register::E)),
            0xFC => Some(Instruction::Set(7, Register::H)),
            0xFD => Some(Instruction::Set(7, Register::L)),
            0xFE => Some(Instruction::Set(7, Register::HL)),
            0xFF => Some(Instruction::Set(7, Register::A)),

            0x80 => Some(Instruction::Res(0, Register::B)),
            0x81 => Some(Instruction::Res(0, Register::C)),
            0x82 => Some(Instruction::Res(0, Register::D)),
            0x83 => Some(Instruction::Res(0, Register::E)),
            0x84 => Some(Instruction::Res(0, Register::H)),
            0x85 => Some(Instruction::Res(0, Register::L)),
            0x86 => Some(Instruction::Res(0, Register::HL)),
            0x87 => Some(Instruction::Res(0, Register::A)),

            0x88 => Some(Instruction::Res(1, Register::B)),
            0x89 => Some(Instruction::Res(1, Register::C)),
            0x8A => Some(Instruction::Res(1, Register::D)),
            0x8B => Some(Instruction::Res(1, Register::E)),
            0x8C => Some(Instruction::Res(1, Register::H)),
            0x8D => Some(Instruction::Res(1, Register::L)),
            0x8E => Some(Instruction::Res(1, Register::HL)),
            0x8F => Some(Instruction::Res(1, Register::A)),

            0x90 => Some(Instruction::Res(2, Register::B)),
            0x91 => Some(Instruction::Res(2, Register::C)),
            0x92 => Some(Instruction::Res(2, Register::D)),
            0x93 => Some(Instruction::Res(2, Register::E)),
            0x94 => Some(Instruction::Res(2, Register::H)),
            0x95 => Some(Instruction::Res(2, Register::L)),
            0x96 => Some(Instruction::Res(2, Register::HL)),
            0x97 => Some(Instruction::Res(2, Register::A)),

            0x98 => Some(Instruction::Res(3, Register::B)),
            0x99 => Some(Instruction::Res(3, Register::C)),
            0x9A => Some(Instruction::Res(3, Register::D)),
            0x9B => Some(Instruction::Res(3, Register::E)),
            0x9C => Some(Instruction::Res(3, Register::H)),
            0x9D => Some(Instruction::Res(3, Register::L)),
            0x9E => Some(Instruction::Res(3, Register::HL)),
            0x9F => Some(Instruction::Res(3, Register::A)),

            0xA0 => Some(Instruction::Res(4, Register::B)),
            0xA1 => Some(Instruction::Res(4, Register::C)),
            0xA2 => Some(Instruction::Res(4, Register::D)),
            0xA3 => Some(Instruction::Res(4, Register::E)),
            0xA4 => Some(Instruction::Res(4, Register::H)),
            0xA5 => Some(Instruction::Res(4, Register::L)),
            0xA6 => Some(Instruction::Res(4, Register::HL)),
            0xA7 => Some(Instruction::Res(4, Register::A)),

            0xA8 => Some(Instruction::Res(5, Register::B)),
            0xA9 => Some(Instruction::Res(5, Register::C)),
            0xAA => Some(Instruction::Res(5, Register::D)),
            0xAB => Some(Instruction::Res(5, Register::E)),
            0xAC => Some(Instruction::Res(5, Register::H)),
            0xAD => Some(Instruction::Res(5, Register::L)),
            0xAE => Some(Instruction::Res(5, Register::HL)),
            0xAF => Some(Instruction::Res(5, Register::A)),

            0xB0 => Some(Instruction::Res(6, Register::B)),
            0xB1 => Some(Instruction::Res(6, Register::C)),
            0xB2 => Some(Instruction::Res(6, Register::D)),
            0xB3 => Some(Instruction::Res(6, Register::E)),
            0xB4 => Some(Instruction::Res(6, Register::H)),
            0xB5 => Some(Instruction::Res(6, Register::L)),
            0xB6 => Some(Instruction::Res(6, Register::HL)),
            0xB7 => Some(Instruction::Res(6, Register::A)),

            0xB8 => Some(Instruction::Res(7, Register::B)),
            0xB9 => Some(Instruction::Res(7, Register::C)),
            0xBA => Some(Instruction::Res(7, Register::D)),
            0xBB => Some(Instruction::Res(7, Register::E)),
            0xBC => Some(Instruction::Res(7, Register::H)),
            0xBD => Some(Instruction::Res(7, Register::L)),
            0xBE => Some(Instruction::Res(7, Register::HL)),
            0xBF => Some(Instruction::Res(7, Register::A)),

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

            0xF9 => Some(Instruction::Ld16(Register::SP, Register::HL)),
            0xF8 => Some(Instruction::Ld16(Register::SP, Register::D8)),
            0x08 => Some(Instruction::Ld16(Register::D16, Register::SP)),

            0xF5 => Some(Instruction::Push(Register::AF)),
            0xC5 => Some(Instruction::Push(Register::BC)),
            0xD5 => Some(Instruction::Push(Register::DE)),
            0xE5 => Some(Instruction::Push(Register::HL)),

            0xF1 => Some(Instruction::Pop(Register::AF)),
            0xC1 => Some(Instruction::Pop(Register::BC)),
            0xD1 => Some(Instruction::Pop(Register::DE)),
            0xE1 => Some(Instruction::Pop(Register::HL)),

            0x87 => Some(Instruction::Add(Register::A)),
            0x80 => Some(Instruction::Add(Register::B)),
            0x81 => Some(Instruction::Add(Register::C)),
            0x82 => Some(Instruction::Add(Register::D)),
            0x83 => Some(Instruction::Add(Register::E)),
            0x84 => Some(Instruction::Add(Register::H)),
            0x85 => Some(Instruction::Add(Register::L)),
            0x86 => Some(Instruction::Add(Register::HL)),
            0xC6 => Some(Instruction::Add(Register::D8)),

            0x8F => Some(Instruction::Adc(Register::A)),
            0x88 => Some(Instruction::Adc(Register::B)),
            0x89 => Some(Instruction::Adc(Register::C)),
            0x8A => Some(Instruction::Adc(Register::D)),
            0x8B => Some(Instruction::Adc(Register::E)),
            0x8C => Some(Instruction::Adc(Register::H)),
            0x8D => Some(Instruction::Adc(Register::L)),
            0x8E => Some(Instruction::Adc(Register::HL)),
            0xCE => Some(Instruction::Adc(Register::D8)),

            0x97 => Some(Instruction::Sub(Register::A)),
            0x90 => Some(Instruction::Sub(Register::B)),
            0x91 => Some(Instruction::Sub(Register::C)),
            0x92 => Some(Instruction::Sub(Register::D)),
            0x93 => Some(Instruction::Sub(Register::E)),
            0x94 => Some(Instruction::Sub(Register::H)),
            0x95 => Some(Instruction::Sub(Register::L)),
            0x96 => Some(Instruction::Sub(Register::HL)),
            0xD6 => Some(Instruction::Sub(Register::D8)),

            0x9F => Some(Instruction::Sbc(Register::A)),
            0x98 => Some(Instruction::Sbc(Register::B)),
            0x99 => Some(Instruction::Sbc(Register::C)),
            0x9A => Some(Instruction::Sbc(Register::D)),
            0x9B => Some(Instruction::Sbc(Register::E)),
            0x9C => Some(Instruction::Sbc(Register::H)),
            0x9D => Some(Instruction::Sbc(Register::L)),
            0x9E => Some(Instruction::Sbc(Register::HL)),
            0xDE => Some(Instruction::Sbc(Register::D8)),

            0xA7 => Some(Instruction::And(Register::A)),
            0xA0 => Some(Instruction::And(Register::B)),
            0xA1 => Some(Instruction::And(Register::C)),
            0xA2 => Some(Instruction::And(Register::D)),
            0xA3 => Some(Instruction::And(Register::E)),
            0xA4 => Some(Instruction::And(Register::H)),
            0xA5 => Some(Instruction::And(Register::L)),
            0xA6 => Some(Instruction::And(Register::HL)),
            0xE6 => Some(Instruction::And(Register::D8)),

            0xB7 => Some(Instruction::Or(Register::A)),
            0xB0 => Some(Instruction::Or(Register::B)),
            0xB1 => Some(Instruction::Or(Register::C)),
            0xB2 => Some(Instruction::Or(Register::D)),
            0xB3 => Some(Instruction::Or(Register::E)),
            0xB4 => Some(Instruction::Or(Register::H)),
            0xB5 => Some(Instruction::Or(Register::L)),
            0xB6 => Some(Instruction::Or(Register::HL)),
            0xF6 => Some(Instruction::Or(Register::D8)),

            0xAF => Some(Instruction::Xor(Register::A)),
            0xA8 => Some(Instruction::Xor(Register::B)),
            0xA9 => Some(Instruction::Xor(Register::C)),
            0xAA => Some(Instruction::Xor(Register::D)),
            0xAB => Some(Instruction::Xor(Register::E)),
            0xAC => Some(Instruction::Xor(Register::H)),
            0xAD => Some(Instruction::Xor(Register::L)),
            0xAE => Some(Instruction::Xor(Register::HL)),
            0xEE => Some(Instruction::Xor(Register::D8)),

            0xBF => Some(Instruction::Cp(Register::A)),
            0xB8 => Some(Instruction::Cp(Register::B)),
            0xB9 => Some(Instruction::Cp(Register::C)),
            0xBA => Some(Instruction::Cp(Register::D)),
            0xBB => Some(Instruction::Cp(Register::E)),
            0xBC => Some(Instruction::Cp(Register::H)),
            0xBD => Some(Instruction::Cp(Register::L)),
            0xBE => Some(Instruction::Cp(Register::HL)),
            0xFE => Some(Instruction::Cp(Register::D8)),

            0x3C => Some(Instruction::Inc(Register::A)),
            0x04 => Some(Instruction::Inc(Register::B)),
            0x0C => Some(Instruction::Inc(Register::C)),
            0x14 => Some(Instruction::Inc(Register::D)),
            0x1C => Some(Instruction::Inc(Register::E)),
            0x24 => Some(Instruction::Inc(Register::H)),
            0x2C => Some(Instruction::Inc(Register::L)),
            0x34 => Some(Instruction::Inc(Register::HL)),

            0x3D => Some(Instruction::Dec(Register::A)),
            0x05 => Some(Instruction::Dec(Register::B)),
            0x0D => Some(Instruction::Dec(Register::C)),
            0x15 => Some(Instruction::Dec(Register::D)),
            0x1D => Some(Instruction::Dec(Register::E)),
            0x25 => Some(Instruction::Dec(Register::H)),
            0x2D => Some(Instruction::Dec(Register::L)),
            0x35 => Some(Instruction::Dec(Register::HL)),

            0x09 => Some(Instruction::Add16(Register::BC)),
            0x19 => Some(Instruction::Add16(Register::DE)),
            0x29 => Some(Instruction::Add16(Register::HL)),
            0x39 => Some(Instruction::Add16(Register::SP)),

            0xE8 => Some(Instruction::Add16SP),

            0x03 => Some(Instruction::Inc16(Register::BC)),
            0x13 => Some(Instruction::Inc16(Register::DE)),
            0x23 => Some(Instruction::Inc16(Register::HL)),
            0x33 => Some(Instruction::Inc16(Register::SP)),

            0x0B => Some(Instruction::Dec16(Register::BC)),
            0x1B => Some(Instruction::Dec16(Register::DE)),
            0x2B => Some(Instruction::Dec16(Register::HL)),
            0x3B => Some(Instruction::Dec16(Register::SP)),

            0x27 => Some(Instruction::DAA),
            0x2F => Some(Instruction::CPL),
            0x3F => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),

            0x76 => Some(Instruction::HALT),
            0x10 => Some(Instruction::STOP),
            0xF3 => Some(Instruction::DI),
            0xFB => Some(Instruction::EI),

            0x07 => Some(Instruction::RLCA),
            0x17 => Some(Instruction::RLA),
            0x0F => Some(Instruction::RRCA),
            0x1F => Some(Instruction::RRA),

            0xC3 => Some(Instruction::Jp),
            0xC2 => Some(Instruction::JpCond(FlagCondition::NZ)),
            0xCA => Some(Instruction::JpCond(FlagCondition::Z)),
            0xD2 => Some(Instruction::JpCond(FlagCondition::NC)),
            0xDA => Some(Instruction::JpCond(FlagCondition::C)),

            0xE9 => Some(Instruction::JpHL),

            0x18 => Some(Instruction::Jr),
            0x20 => Some(Instruction::JrCond(FlagCondition::NZ)),
            0x28 => Some(Instruction::JrCond(FlagCondition::Z)),
            0x30 => Some(Instruction::JrCond(FlagCondition::NC)),
            0x38 => Some(Instruction::JrCond(FlagCondition::C)),

            0xCD => Some(Instruction::Call),
            0xC4 => Some(Instruction::CallCond(FlagCondition::NZ)),
            0xCC => Some(Instruction::CallCond(FlagCondition::Z)),
            0xD4 => Some(Instruction::CallCond(FlagCondition::NC)),
            0xDC => Some(Instruction::CallCond(FlagCondition::C)),

            0xC7 => Some(Instruction::Rst(0x00)),
            0xCF => Some(Instruction::Rst(0x08)),
            0xD7 => Some(Instruction::Rst(0x10)),
            0xDF => Some(Instruction::Rst(0x18)),
            0xE7 => Some(Instruction::Rst(0x20)),
            0xEF => Some(Instruction::Rst(0x28)),
            0xF7 => Some(Instruction::Rst(0x30)),
            0xFF => Some(Instruction::Rst(0x38)),

            0xC9 => Some(Instruction::Ret),

            0xC0 => Some(Instruction::RetCond(FlagCondition::NZ)),
            0xC8 => Some(Instruction::RetCond(FlagCondition::Z)),
            0xD0 => Some(Instruction::RetCond(FlagCondition::NC)),
            0xD8 => Some(Instruction::RetCond(FlagCondition::C)),

            0xD9 => Some(Instruction::Reti),

            _ => None,
        }
    }
}
