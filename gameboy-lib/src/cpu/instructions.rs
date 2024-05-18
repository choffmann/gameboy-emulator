use super::registers::Register;

#[derive(Debug)]
pub enum FlagCondition {
    NZ, // Not zero
    Z,  // Zero
    NC, // Not carry
    C,  // Carry
}

#[derive(Debug)]
pub enum LoadInstruction {
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
}

#[derive(Debug)]
pub enum ArithmeticInstruction {
    Add(Register),   // Add register to A
    Add16(Register), // Add register to HL
    Add16SP,         // Add SP to HL
    Adc(Register),   // Add register to A with carry
    Sub(Register),   // Subtract register from A
    Sbc(Register),   // Subtract register from A with carry
    And(Register),   // Logical AND register with A
    Or(Register),    // Logical OR register with A
    Xor(Register),   // Logical XOR register with A
    Cp(Register),    // Compare register with A
    Inc(Register),   // Increment register
    Inc16(Register), // Increment register
    Dec(Register),   // Decrement register
    Dec16(Register), // Decrement register
}

#[derive(Debug)]
pub enum MiscInstruction {
    Nop,            // No operation
    Swap(Register), // Swap upper and lower nibbles of register
    DAA,            // Decimal adjust register A
    CPL,            // Complement A
    CCF,            // Complement carry flag
    SCF,            // Set carry flag
    HALT,           // Halt CPU
    STOP,           // Stop CPU
    DI,             // Disable interrupts
    EI,             // Enable interrupts
}

#[derive(Debug)]
pub enum RotateInstruction {
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
}

#[derive(Debug)]
pub enum JumpInstruction {
    Jp,                    // Jump to address
    JpCond(FlagCondition), // Conditional jump to address
    JpHL,                  // Jump to HL
    Jr,                    // Jump relative
    JrCond(FlagCondition), // Conditional jump relative
}

#[derive(Debug)]
pub enum BitInstruction {
    Bit(u8, Register), // Test bit in register
    Set(u8, Register), // Set bit in register
    Res(u8, Register), // Reset bit in register
}

#[derive(Debug)]
pub enum CallInstruction {
    Call,                    // Call address
    CallCond(FlagCondition), // Conditional call address
}

#[derive(Debug)]
pub enum ReturnInstruction {
    Rst(u8),                // Restart to address
    Ret,                    // Return from subroutine
    RetCond(FlagCondition), // Conditional return from subroutine
    Reti,                   // Return from interrupt
}

#[derive(Debug)]
pub enum Instruction {
    Load(LoadInstruction),             
    Arithmetic(ArithmeticInstruction), 
    Misc(MiscInstruction),             
    Rotate(RotateInstruction),         
    Jump(JumpInstruction),             
    Bit(BitInstruction),               
    Call(CallInstruction),
    Return(ReturnInstruction),
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
            0x37 => Some(Instruction::Misc(MiscInstruction::Swap(Register::A))),
            0x30 => Some(Instruction::Misc(MiscInstruction::Swap(Register::B))),
            0x31 => Some(Instruction::Misc(MiscInstruction::Swap(Register::C))),
            0x32 => Some(Instruction::Misc(MiscInstruction::Swap(Register::D))),
            0x33 => Some(Instruction::Misc(MiscInstruction::Swap(Register::E))),
            0x34 => Some(Instruction::Misc(MiscInstruction::Swap(Register::H))),
            0x35 => Some(Instruction::Misc(MiscInstruction::Swap(Register::L))),
            0x36 => Some(Instruction::Misc(MiscInstruction::Swap(Register::HL))),

            0x07 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::A))),
            0x00 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::B))),
            0x01 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::C))),
            0x02 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::D))),
            0x03 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::E))),
            0x04 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::H))),
            0x05 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::L))),
            0x06 => Some(Instruction::Rotate(RotateInstruction::RLC(Register::HL))),

            0x17 => Some(Instruction::Rotate(RotateInstruction::RL(Register::A))),
            0x10 => Some(Instruction::Rotate(RotateInstruction::RL(Register::B))),
            0x11 => Some(Instruction::Rotate(RotateInstruction::RL(Register::C))),
            0x12 => Some(Instruction::Rotate(RotateInstruction::RL(Register::D))),
            0x13 => Some(Instruction::Rotate(RotateInstruction::RL(Register::E))),
            0x14 => Some(Instruction::Rotate(RotateInstruction::RL(Register::H))),
            0x15 => Some(Instruction::Rotate(RotateInstruction::RL(Register::L))),
            0x16 => Some(Instruction::Rotate(RotateInstruction::RL(Register::HL))),

            0x0F => Some(Instruction::Rotate(RotateInstruction::RRC(Register::A))),
            0x08 => Some(Instruction::Rotate(RotateInstruction::RRC(Register::B))),
            0x09 => Some(Instruction::Rotate(RotateInstruction::RRC(Register::C))),
            0x0A => Some(Instruction::Rotate(RotateInstruction::RRC(Register::D))),
            0x0B => Some(Instruction::Rotate(RotateInstruction::RRC(Register::E))),
            0x0C => Some(Instruction::Rotate(RotateInstruction::RRC(Register::H))),
            0x0D => Some(Instruction::Rotate(RotateInstruction::RRC(Register::L))),
            0x0E => Some(Instruction::Rotate(RotateInstruction::RRC(Register::HL))),

            0x1F => Some(Instruction::Rotate(RotateInstruction::RR(Register::A))),
            0x18 => Some(Instruction::Rotate(RotateInstruction::RR(Register::B))),
            0x19 => Some(Instruction::Rotate(RotateInstruction::RR(Register::C))),
            0x1A => Some(Instruction::Rotate(RotateInstruction::RR(Register::D))),
            0x1B => Some(Instruction::Rotate(RotateInstruction::RR(Register::E))),
            0x1C => Some(Instruction::Rotate(RotateInstruction::RR(Register::H))),
            0x1D => Some(Instruction::Rotate(RotateInstruction::RR(Register::L))),
            0x1E => Some(Instruction::Rotate(RotateInstruction::RR(Register::HL))),

            0x27 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::A))),
            0x20 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::B))),
            0x21 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::C))),
            0x22 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::D))),
            0x23 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::E))),
            0x24 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::H))),
            0x25 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::L))),
            0x26 => Some(Instruction::Rotate(RotateInstruction::SLA(Register::HL))),

            0x2F => Some(Instruction::Rotate(RotateInstruction::SRA(Register::A))),
            0x28 => Some(Instruction::Rotate(RotateInstruction::SRA(Register::B))),
            0x29 => Some(Instruction::Rotate(RotateInstruction::SRA(Register::C))),
            0x2A => Some(Instruction::Rotate(RotateInstruction::SRA(Register::D))),
            0x2B => Some(Instruction::Rotate(RotateInstruction::SRA(Register::E))),
            0x2C => Some(Instruction::Rotate(RotateInstruction::SRA(Register::H))),
            0x2D => Some(Instruction::Rotate(RotateInstruction::SRA(Register::L))),
            0x2E => Some(Instruction::Rotate(RotateInstruction::SRA(Register::HL))),

            0x3F => Some(Instruction::Rotate(RotateInstruction::SRL(Register::A))),
            0x38 => Some(Instruction::Rotate(RotateInstruction::SRL(Register::B))),
            0x39 => Some(Instruction::Rotate(RotateInstruction::SRL(Register::C))),
            0x3A => Some(Instruction::Rotate(RotateInstruction::SRL(Register::D))),
            0x3B => Some(Instruction::Rotate(RotateInstruction::SRL(Register::E))),
            0x3C => Some(Instruction::Rotate(RotateInstruction::SRL(Register::H))),
            0x3D => Some(Instruction::Rotate(RotateInstruction::SRL(Register::L))),
            0x3E => Some(Instruction::Rotate(RotateInstruction::SRL(Register::HL))),

            0x40 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::B))),
            0x41 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::C))),
            0x42 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::D))),
            0x43 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::E))),
            0x44 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::H))),
            0x45 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::L))),
            0x46 => Some(Instruction::Bit(BitInstruction::Bit(0, Register::HL))),

            0x47 => Some(Instruction::Bit(BitInstruction::Bit(1, Register::B))),
            0x48 => Some(Instruction::Bit(BitInstruction::Bit(1, Register::C))),
            0x49 => Some(Instruction::Bit(BitInstruction::Bit(1, Register::D))),
            0x4A => Some(Instruction::Bit(BitInstruction::Bit(1, Register::E))),
            0x4B => Some(Instruction::Bit(BitInstruction::Bit(1, Register::H))),
            0x4C => Some(Instruction::Bit(BitInstruction::Bit(1, Register::L))),
            0x4D => Some(Instruction::Bit(BitInstruction::Bit(1, Register::HL))),
            0x4E => Some(Instruction::Bit(BitInstruction::Bit(1, Register::A))),

            0x50 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::B))),
            0x51 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::C))),
            0x52 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::D))),
            0x53 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::E))),
            0x54 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::H))),
            0x55 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::L))),
            0x56 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::HL))),
            0x57 => Some(Instruction::Bit(BitInstruction::Bit(2, Register::A))),

            0x58 => Some(Instruction::Bit(BitInstruction::Bit(3, Register::B))),
            0x59 => Some(Instruction::Bit(BitInstruction::Bit(3, Register::C))),
            0x5A => Some(Instruction::Bit(BitInstruction::Bit(3, Register::D))),
            0x5B => Some(Instruction::Bit(BitInstruction::Bit(3, Register::E))),
            0x5C => Some(Instruction::Bit(BitInstruction::Bit(3, Register::H))),
            0x5D => Some(Instruction::Bit(BitInstruction::Bit(3, Register::L))),
            0x5E => Some(Instruction::Bit(BitInstruction::Bit(3, Register::HL))),
            0x5F => Some(Instruction::Bit(BitInstruction::Bit(3, Register::A))),

            0x60 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::B))),
            0x61 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::C))),
            0x62 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::D))),
            0x63 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::E))),
            0x64 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::H))),
            0x65 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::L))),
            0x66 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::HL))),
            0x67 => Some(Instruction::Bit(BitInstruction::Bit(4, Register::A))),

            0x68 => Some(Instruction::Bit(BitInstruction::Bit(5, Register::B))),
            0x69 => Some(Instruction::Bit(BitInstruction::Bit(5, Register::C))),
            0x6A => Some(Instruction::Bit(BitInstruction::Bit(5, Register::D))),
            0x6B => Some(Instruction::Bit(BitInstruction::Bit(5, Register::E))),
            0x6C => Some(Instruction::Bit(BitInstruction::Bit(5, Register::H))),
            0x6D => Some(Instruction::Bit(BitInstruction::Bit(5, Register::L))),
            0x6E => Some(Instruction::Bit(BitInstruction::Bit(5, Register::HL))),
            0x6F => Some(Instruction::Bit(BitInstruction::Bit(5, Register::A))),

            0x70 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::B))),
            0x71 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::C))),
            0x72 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::D))),
            0x73 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::E))),
            0x74 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::H))),
            0x75 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::L))),
            0x76 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::HL))),
            0x77 => Some(Instruction::Bit(BitInstruction::Bit(6, Register::A))),

            0x78 => Some(Instruction::Bit(BitInstruction::Bit(7, Register::B))),
            0x79 => Some(Instruction::Bit(BitInstruction::Bit(7, Register::C))),
            0x7A => Some(Instruction::Bit(BitInstruction::Bit(7, Register::D))),
            0x7B => Some(Instruction::Bit(BitInstruction::Bit(7, Register::E))),
            0x7C => Some(Instruction::Bit(BitInstruction::Bit(7, Register::H))),
            0x7D => Some(Instruction::Bit(BitInstruction::Bit(7, Register::L))),
            0x7E => Some(Instruction::Bit(BitInstruction::Bit(7, Register::HL))),
            0x7F => Some(Instruction::Bit(BitInstruction::Bit(7, Register::A))),

            0xC0 => Some(Instruction::Bit(BitInstruction::Set(0, Register::B))),
            0xC1 => Some(Instruction::Bit(BitInstruction::Set(0, Register::C))),
            0xC2 => Some(Instruction::Bit(BitInstruction::Set(0, Register::D))),
            0xC3 => Some(Instruction::Bit(BitInstruction::Set(0, Register::E))),
            0xC4 => Some(Instruction::Bit(BitInstruction::Set(0, Register::H))),
            0xC5 => Some(Instruction::Bit(BitInstruction::Set(0, Register::L))),
            0xC6 => Some(Instruction::Bit(BitInstruction::Set(0, Register::HL))),
            0xC7 => Some(Instruction::Bit(BitInstruction::Set(0, Register::A))),

            0xC8 => Some(Instruction::Bit(BitInstruction::Set(1, Register::B))),
            0xC9 => Some(Instruction::Bit(BitInstruction::Set(1, Register::C))),
            0xCA => Some(Instruction::Bit(BitInstruction::Set(1, Register::D))),
            0xCB => Some(Instruction::Bit(BitInstruction::Set(1, Register::E))),
            0xCC => Some(Instruction::Bit(BitInstruction::Set(1, Register::H))),
            0xCD => Some(Instruction::Bit(BitInstruction::Set(1, Register::L))),
            0xCE => Some(Instruction::Bit(BitInstruction::Set(1, Register::HL))),
            0xCF => Some(Instruction::Bit(BitInstruction::Set(1, Register::A))),

            0xD0 => Some(Instruction::Bit(BitInstruction::Set(2, Register::B))),
            0xD1 => Some(Instruction::Bit(BitInstruction::Set(2, Register::C))),
            0xD2 => Some(Instruction::Bit(BitInstruction::Set(2, Register::D))),
            0xD3 => Some(Instruction::Bit(BitInstruction::Set(2, Register::E))),
            0xD4 => Some(Instruction::Bit(BitInstruction::Set(2, Register::H))),
            0xD5 => Some(Instruction::Bit(BitInstruction::Set(2, Register::L))),
            0xD6 => Some(Instruction::Bit(BitInstruction::Set(2, Register::HL))),
            0xD7 => Some(Instruction::Bit(BitInstruction::Set(2, Register::A))),

            0xD8 => Some(Instruction::Bit(BitInstruction::Set(3, Register::B))),
            0xD9 => Some(Instruction::Bit(BitInstruction::Set(3, Register::C))),
            0xDA => Some(Instruction::Bit(BitInstruction::Set(3, Register::D))),
            0xDB => Some(Instruction::Bit(BitInstruction::Set(3, Register::E))),
            0xDC => Some(Instruction::Bit(BitInstruction::Set(3, Register::H))),
            0xDD => Some(Instruction::Bit(BitInstruction::Set(3, Register::L))),
            0xDE => Some(Instruction::Bit(BitInstruction::Set(3, Register::HL))),
            0xDF => Some(Instruction::Bit(BitInstruction::Set(3, Register::A))),

            0xE0 => Some(Instruction::Bit(BitInstruction::Set(4, Register::B))),
            0xE1 => Some(Instruction::Bit(BitInstruction::Set(4, Register::C))),
            0xE2 => Some(Instruction::Bit(BitInstruction::Set(4, Register::D))),
            0xE3 => Some(Instruction::Bit(BitInstruction::Set(4, Register::E))),
            0xE4 => Some(Instruction::Bit(BitInstruction::Set(4, Register::H))),
            0xE5 => Some(Instruction::Bit(BitInstruction::Set(4, Register::L))),
            0xE6 => Some(Instruction::Bit(BitInstruction::Set(4, Register::HL))),
            0xE7 => Some(Instruction::Bit(BitInstruction::Set(4, Register::A))),

            0xE8 => Some(Instruction::Bit(BitInstruction::Set(5, Register::B))),
            0xE9 => Some(Instruction::Bit(BitInstruction::Set(5, Register::C))),
            0xEA => Some(Instruction::Bit(BitInstruction::Set(5, Register::D))),
            0xEB => Some(Instruction::Bit(BitInstruction::Set(5, Register::E))),
            0xEC => Some(Instruction::Bit(BitInstruction::Set(5, Register::H))),
            0xED => Some(Instruction::Bit(BitInstruction::Set(5, Register::L))),
            0xEE => Some(Instruction::Bit(BitInstruction::Set(5, Register::HL))),
            0xEF => Some(Instruction::Bit(BitInstruction::Set(5, Register::A))),

            0xF0 => Some(Instruction::Bit(BitInstruction::Set(6, Register::B))),
            0xF1 => Some(Instruction::Bit(BitInstruction::Set(6, Register::C))),
            0xF2 => Some(Instruction::Bit(BitInstruction::Set(6, Register::D))),
            0xF3 => Some(Instruction::Bit(BitInstruction::Set(6, Register::E))),
            0xF4 => Some(Instruction::Bit(BitInstruction::Set(6, Register::H))),
            0xF5 => Some(Instruction::Bit(BitInstruction::Set(6, Register::L))),
            0xF6 => Some(Instruction::Bit(BitInstruction::Set(6, Register::HL))),
            0xF7 => Some(Instruction::Bit(BitInstruction::Set(6, Register::A))),

            0xF8 => Some(Instruction::Bit(BitInstruction::Set(7, Register::B))),
            0xF9 => Some(Instruction::Bit(BitInstruction::Set(7, Register::C))),
            0xFA => Some(Instruction::Bit(BitInstruction::Set(7, Register::D))),
            0xFB => Some(Instruction::Bit(BitInstruction::Set(7, Register::E))),
            0xFC => Some(Instruction::Bit(BitInstruction::Set(7, Register::H))),
            0xFD => Some(Instruction::Bit(BitInstruction::Set(7, Register::L))),
            0xFE => Some(Instruction::Bit(BitInstruction::Set(7, Register::HL))),
            0xFF => Some(Instruction::Bit(BitInstruction::Set(7, Register::A))),

            0x80 => Some(Instruction::Bit(BitInstruction::Res(0, Register::B))),
            0x81 => Some(Instruction::Bit(BitInstruction::Res(0, Register::C))),
            0x82 => Some(Instruction::Bit(BitInstruction::Res(0, Register::D))),
            0x83 => Some(Instruction::Bit(BitInstruction::Res(0, Register::E))),
            0x84 => Some(Instruction::Bit(BitInstruction::Res(0, Register::H))),
            0x85 => Some(Instruction::Bit(BitInstruction::Res(0, Register::L))),
            0x86 => Some(Instruction::Bit(BitInstruction::Res(0, Register::HL))),
            0x87 => Some(Instruction::Bit(BitInstruction::Res(0, Register::A))),

            0x88 => Some(Instruction::Bit(BitInstruction::Res(1, Register::B))),
            0x89 => Some(Instruction::Bit(BitInstruction::Res(1, Register::C))),
            0x8A => Some(Instruction::Bit(BitInstruction::Res(1, Register::D))),
            0x8B => Some(Instruction::Bit(BitInstruction::Res(1, Register::E))),
            0x8C => Some(Instruction::Bit(BitInstruction::Res(1, Register::H))),
            0x8D => Some(Instruction::Bit(BitInstruction::Res(1, Register::L))),
            0x8E => Some(Instruction::Bit(BitInstruction::Res(1, Register::HL))),
            0x8F => Some(Instruction::Bit(BitInstruction::Res(1, Register::A))),

            0x90 => Some(Instruction::Bit(BitInstruction::Res(2, Register::B))),
            0x91 => Some(Instruction::Bit(BitInstruction::Res(2, Register::C))),
            0x92 => Some(Instruction::Bit(BitInstruction::Res(2, Register::D))),
            0x93 => Some(Instruction::Bit(BitInstruction::Res(2, Register::E))),
            0x94 => Some(Instruction::Bit(BitInstruction::Res(2, Register::H))),
            0x95 => Some(Instruction::Bit(BitInstruction::Res(2, Register::L))),
            0x96 => Some(Instruction::Bit(BitInstruction::Res(2, Register::HL))),
            0x97 => Some(Instruction::Bit(BitInstruction::Res(2, Register::A))),

            0x98 => Some(Instruction::Bit(BitInstruction::Res(3, Register::B))),
            0x99 => Some(Instruction::Bit(BitInstruction::Res(3, Register::C))),
            0x9A => Some(Instruction::Bit(BitInstruction::Res(3, Register::D))),
            0x9B => Some(Instruction::Bit(BitInstruction::Res(3, Register::E))),
            0x9C => Some(Instruction::Bit(BitInstruction::Res(3, Register::H))),
            0x9D => Some(Instruction::Bit(BitInstruction::Res(3, Register::L))),
            0x9E => Some(Instruction::Bit(BitInstruction::Res(3, Register::HL))),
            0x9F => Some(Instruction::Bit(BitInstruction::Res(3, Register::A))),

            0xA0 => Some(Instruction::Bit(BitInstruction::Res(4, Register::B))),
            0xA1 => Some(Instruction::Bit(BitInstruction::Res(4, Register::C))),
            0xA2 => Some(Instruction::Bit(BitInstruction::Res(4, Register::D))),
            0xA3 => Some(Instruction::Bit(BitInstruction::Res(4, Register::E))),
            0xA4 => Some(Instruction::Bit(BitInstruction::Res(4, Register::H))),
            0xA5 => Some(Instruction::Bit(BitInstruction::Res(4, Register::L))),
            0xA6 => Some(Instruction::Bit(BitInstruction::Res(4, Register::HL))),
            0xA7 => Some(Instruction::Bit(BitInstruction::Res(4, Register::A))),

            0xA8 => Some(Instruction::Bit(BitInstruction::Res(5, Register::B))),
            0xA9 => Some(Instruction::Bit(BitInstruction::Res(5, Register::C))),
            0xAA => Some(Instruction::Bit(BitInstruction::Res(5, Register::D))),
            0xAB => Some(Instruction::Bit(BitInstruction::Res(5, Register::E))),
            0xAC => Some(Instruction::Bit(BitInstruction::Res(5, Register::H))),
            0xAD => Some(Instruction::Bit(BitInstruction::Res(5, Register::L))),
            0xAE => Some(Instruction::Bit(BitInstruction::Res(5, Register::HL))),
            0xAF => Some(Instruction::Bit(BitInstruction::Res(5, Register::A))),

            0xB0 => Some(Instruction::Bit(BitInstruction::Res(6, Register::B))),
            0xB1 => Some(Instruction::Bit(BitInstruction::Res(6, Register::C))),
            0xB2 => Some(Instruction::Bit(BitInstruction::Res(6, Register::D))),
            0xB3 => Some(Instruction::Bit(BitInstruction::Res(6, Register::E))),
            0xB4 => Some(Instruction::Bit(BitInstruction::Res(6, Register::H))),
            0xB5 => Some(Instruction::Bit(BitInstruction::Res(6, Register::L))),
            0xB6 => Some(Instruction::Bit(BitInstruction::Res(6, Register::HL))),
            0xB7 => Some(Instruction::Bit(BitInstruction::Res(6, Register::A))),

            0xB8 => Some(Instruction::Bit(BitInstruction::Res(7, Register::B))),
            0xB9 => Some(Instruction::Bit(BitInstruction::Res(7, Register::C))),
            0xBA => Some(Instruction::Bit(BitInstruction::Res(7, Register::D))),
            0xBB => Some(Instruction::Bit(BitInstruction::Res(7, Register::E))),
            0xBC => Some(Instruction::Bit(BitInstruction::Res(7, Register::H))),
            0xBD => Some(Instruction::Bit(BitInstruction::Res(7, Register::L))),
            0xBE => Some(Instruction::Bit(BitInstruction::Res(7, Register::HL))),
            0xBF => Some(Instruction::Bit(BitInstruction::Res(7, Register::A))),

            _ => None,
        }
    }

    fn from_byte_without_prefix(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::Misc(MiscInstruction::Nop)),
            0x06 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::D8))),
            0x0E => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::D8))),
            0x16 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::D8))),
            0x1E => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::D8))),
            0x26 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::D8))),
            0x2E => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::D8))),

            0x7F => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::A))),
            0x78 => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::B))),
            0x79 => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::C))),
            0x7A => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::D))),
            0x7B => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::E))),
            0x7C => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::H))),
            0x7D => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::L))),
            0x0A => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::BC))),
            0x1A => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::DE))),
            0x7E => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::HL))),
            0xFA => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::D16))),
            0x3E => Some(Instruction::Load(LoadInstruction::Ld8(Register::A, Register::D8))),

            0x40 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::B))),
            0x41 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::C))),
            0x42 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::D))),
            0x43 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::E))),
            0x44 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::H))),
            0x45 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::L))),
            0x46 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::HL))),

            0x48 => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::B))),
            0x49 => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::C))),
            0x4A => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::D))),
            0x4B => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::E))),
            0x4C => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::H))),
            0x4D => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::L))),
            0x4E => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::HL))),

            0x50 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::B))),
            0x51 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::C))),
            0x52 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::D))),
            0x53 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::E))),
            0x54 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::H))),
            0x55 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::L))),
            0x56 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::HL))),

            0x58 => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::B))),
            0x59 => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::C))),
            0x5A => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::D))),
            0x5B => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::E))),
            0x5C => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::H))),
            0x5D => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::L))),
            0x5E => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::HL))),

            0x60 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::B))),
            0x61 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::C))),
            0x62 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::D))),
            0x63 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::E))),
            0x64 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::H))),
            0x65 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::L))),
            0x66 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::HL))),

            0x68 => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::B))),
            0x69 => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::C))),
            0x6A => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::D))),
            0x6B => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::E))),
            0x6C => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::H))),
            0x6D => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::L))),
            0x6E => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::HL))),

            0x70 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::B))),
            0x71 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::C))),
            0x72 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::D))),
            0x73 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::E))),
            0x74 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::H))),
            0x75 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::L))),
            0x36 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::D8))),

            0x47 => Some(Instruction::Load(LoadInstruction::Ld8(Register::B, Register::A))),
            0x4F => Some(Instruction::Load(LoadInstruction::Ld8(Register::C, Register::A))),
            0x57 => Some(Instruction::Load(LoadInstruction::Ld8(Register::D, Register::A))),
            0x5F => Some(Instruction::Load(LoadInstruction::Ld8(Register::E, Register::A))),
            0x67 => Some(Instruction::Load(LoadInstruction::Ld8(Register::H, Register::A))),
            0x6F => Some(Instruction::Load(LoadInstruction::Ld8(Register::L, Register::A))),
            0x02 => Some(Instruction::Load(LoadInstruction::Ld8(Register::BC, Register::A))),
            0x12 => Some(Instruction::Load(LoadInstruction::Ld8(Register::DE, Register::A))),
            0x77 => Some(Instruction::Load(LoadInstruction::Ld8(Register::HL, Register::A))),
            0xEA => Some(Instruction::Load(LoadInstruction::Ld8(Register::D16, Register::A))),

            0xF2 => Some(Instruction::Load(LoadInstruction::LdAc)),
            0xE2 => Some(Instruction::Load(LoadInstruction::LdCa)),

            0x3A => Some(Instruction::Load(LoadInstruction::LdHd)),
            0x32 => Some(Instruction::Load(LoadInstruction::LdHd)),

            0x2A => Some(Instruction::Load(LoadInstruction::LdHi)),
            0x22 => Some(Instruction::Load(LoadInstruction::LdHi)),

            0xE0 => Some(Instruction::Load(LoadInstruction::LdNa)),
            0xF0 => Some(Instruction::Load(LoadInstruction::LdAn)),

            0x01 => Some(Instruction::Load(LoadInstruction::Ld16(Register::BC, Register::D16))),
            0x11 => Some(Instruction::Load(LoadInstruction::Ld16(Register::DE, Register::D16))),
            0x21 => Some(Instruction::Load(LoadInstruction::Ld16(Register::HL, Register::D16))),
            0x31 => Some(Instruction::Load(LoadInstruction::Ld16(Register::SP, Register::D16))),

            0xF9 => Some(Instruction::Load(LoadInstruction::Ld16(Register::SP, Register::HL))),
            0xF8 => Some(Instruction::Load(LoadInstruction::Ld16(Register::SP, Register::D8))),
            0x08 => Some(Instruction::Load(LoadInstruction::Ld16(Register::D16, Register::SP))),

            0xF5 => Some(Instruction::Load(LoadInstruction::Push(Register::AF))),
            0xC5 => Some(Instruction::Load(LoadInstruction::Push(Register::BC))),
            0xD5 => Some(Instruction::Load(LoadInstruction::Push(Register::DE))),
            0xE5 => Some(Instruction::Load(LoadInstruction::Push(Register::HL))),

            0xF1 => Some(Instruction::Load(LoadInstruction::Pop(Register::AF))),
            0xC1 => Some(Instruction::Load(LoadInstruction::Pop(Register::BC))),
            0xD1 => Some(Instruction::Load(LoadInstruction::Pop(Register::DE))),
            0xE1 => Some(Instruction::Load(LoadInstruction::Pop(Register::HL))),

            0x87 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::A))),
            0x80 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::B))),
            0x81 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::C))),
            0x82 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::D))),
            0x83 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::E))),
            0x84 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::H))),
            0x85 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::L))),
            0x86 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::HL))),
            0xC6 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add(Register::D8))),

            0x8F => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::A))),
            0x88 => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::B))),
            0x89 => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::C))),
            0x8A => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::D))),
            0x8B => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::E))),
            0x8C => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::H))),
            0x8D => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::L))),
            0x8E => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::HL))),
            0xCE => Some(Instruction::Arithmetic(ArithmeticInstruction::Adc(Register::D8))),

            0x97 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::A))),
            0x90 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::B))),
            0x91 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::C))),
            0x92 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::D))),
            0x93 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::E))),
            0x94 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::H))),
            0x95 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::L))),
            0x96 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::HL))),
            0xD6 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sub(Register::D8))),

            0x9F => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::A))),
            0x98 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::B))),
            0x99 => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::C))),
            0x9A => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::D))),
            0x9B => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::E))),
            0x9C => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::H))),
            0x9D => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::L))),
            0x9E => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::HL))),
            0xDE => Some(Instruction::Arithmetic(ArithmeticInstruction::Sbc(Register::D8))),

            0xA7 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::A))),
            0xA0 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::B))),
            0xA1 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::C))),
            0xA2 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::D))),
            0xA3 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::E))),
            0xA4 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::H))),
            0xA5 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::L))),
            0xA6 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::HL))),
            0xE6 => Some(Instruction::Arithmetic(ArithmeticInstruction::And(Register::D8))),

            0xB7 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::A))),
            0xB0 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::B))),
            0xB1 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::C))),
            0xB2 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::D))),
            0xB3 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::E))),
            0xB4 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::H))),
            0xB5 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::L))),
            0xB6 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::HL))),
            0xF6 => Some(Instruction::Arithmetic(ArithmeticInstruction::Or(Register::D8))),

            0xAF => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::A))),
            0xA8 => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::B))),
            0xA9 => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::C))),
            0xAA => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::D))),
            0xAB => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::E))),
            0xAC => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::H))),
            0xAD => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::L))),
            0xAE => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::HL))),
            0xEE => Some(Instruction::Arithmetic(ArithmeticInstruction::Xor(Register::D8))),

            0xBF => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::A))),
            0xB8 => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::B))),
            0xB9 => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::C))),
            0xBA => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::D))),
            0xBB => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::E))),
            0xBC => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::H))),
            0xBD => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::L))),
            0xBE => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::HL))),
            0xFE => Some(Instruction::Arithmetic(ArithmeticInstruction::Cp(Register::D8))),

            0x3C => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::A))),
            0x04 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::B))),
            0x0C => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::C))),
            0x14 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::D))),
            0x1C => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::E))),
            0x24 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::H))),
            0x2C => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::L))),
            0x34 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc(Register::HL))),

            0x3D => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::A))),
            0x05 => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::B))),
            0x0D => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::C))),
            0x15 => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::D))),
            0x1D => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::E))),
            0x25 => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::H))),
            0x2D => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::L))),
            0x35 => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec(Register::HL))),

            0x09 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add16(Register::BC))),
            0x19 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add16(Register::DE))),
            0x29 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add16(Register::HL))),
            0x39 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add16(Register::SP))),

            0xE8 => Some(Instruction::Arithmetic(ArithmeticInstruction::Add16SP)),

            0x03 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc16(Register::BC))),
            0x13 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc16(Register::DE))),
            0x23 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc16(Register::HL))),
            0x33 => Some(Instruction::Arithmetic(ArithmeticInstruction::Inc16(Register::SP))),

            0x0B => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec16(Register::BC))),
            0x1B => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec16(Register::DE))),
            0x2B => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec16(Register::HL))),
            0x3B => Some(Instruction::Arithmetic(ArithmeticInstruction::Dec16(Register::SP))),

            0x27 => Some(Instruction::Misc(MiscInstruction::DAA)),
            0x2F => Some(Instruction::Misc(MiscInstruction::CPL)),
            0x3F => Some(Instruction::Misc(MiscInstruction::CCF)),
            0x37 => Some(Instruction::Misc(MiscInstruction::SCF)),

            0x76 => Some(Instruction::Misc(MiscInstruction::HALT)),
            0x10 => Some(Instruction::Misc(MiscInstruction::STOP)),
            0xF3 => Some(Instruction::Misc(MiscInstruction::DI)),
            0xFB => Some(Instruction::Misc(MiscInstruction::EI)),

            0x07 => Some(Instruction::Rotate(RotateInstruction::RLCA)),
            0x17 => Some(Instruction::Rotate(RotateInstruction::RLA)),
            0x0F => Some(Instruction::Rotate(RotateInstruction::RRCA)),
            0x1F => Some(Instruction::Rotate(RotateInstruction::RRA)),

            0xC3 => Some(Instruction::Jump(JumpInstruction::Jp)),
            0xC2 => Some(Instruction::Jump(JumpInstruction::JpCond(FlagCondition::NZ))),
            0xCA => Some(Instruction::Jump(JumpInstruction::JpCond(FlagCondition::Z))),
            0xD2 => Some(Instruction::Jump(JumpInstruction::JpCond(FlagCondition::NC))),
            0xDA => Some(Instruction::Jump(JumpInstruction::JpCond(FlagCondition::C))),

            0xE9 => Some(Instruction::Jump(JumpInstruction::JpHL)),

            0x18 => Some(Instruction::Jump(JumpInstruction::Jr)),
            0x20 => Some(Instruction::Jump(JumpInstruction::JrCond(FlagCondition::NZ))),
            0x28 => Some(Instruction::Jump(JumpInstruction::JrCond(FlagCondition::Z))),
            0x30 => Some(Instruction::Jump(JumpInstruction::JrCond(FlagCondition::NC))),
            0x38 => Some(Instruction::Jump(JumpInstruction::JrCond(FlagCondition::C))),

            0xCD => Some(Instruction::Call(CallInstruction::Call)),
            0xC4 => Some(Instruction::Call(CallInstruction::CallCond(FlagCondition::NZ))),
            0xCC => Some(Instruction::Call(CallInstruction::CallCond(FlagCondition::Z))),
            0xD4 => Some(Instruction::Call(CallInstruction::CallCond(FlagCondition::NC))),
            0xDC => Some(Instruction::Call(CallInstruction::CallCond(FlagCondition::C))),

            0xC7 => Some(Instruction::Return(ReturnInstruction::Rst(0x00))),
            0xCF => Some(Instruction::Return(ReturnInstruction::Rst(0x08))),
            0xD7 => Some(Instruction::Return(ReturnInstruction::Rst(0x10))),
            0xDF => Some(Instruction::Return(ReturnInstruction::Rst(0x18))),
            0xE7 => Some(Instruction::Return(ReturnInstruction::Rst(0x20))),
            0xEF => Some(Instruction::Return(ReturnInstruction::Rst(0x28))),
            0xF7 => Some(Instruction::Return(ReturnInstruction::Rst(0x30))),
            0xFF => Some(Instruction::Return(ReturnInstruction::Rst(0x38))),

            0xC9 => Some(Instruction::Return(ReturnInstruction::Ret)),

            0xC0 => Some(Instruction::Return(ReturnInstruction::RetCond(FlagCondition::NZ))),
            0xC8 => Some(Instruction::Return(ReturnInstruction::RetCond(FlagCondition::Z))),
            0xD0 => Some(Instruction::Return(ReturnInstruction::RetCond(FlagCondition::NC))),
            0xD8 => Some(Instruction::Return(ReturnInstruction::RetCond(FlagCondition::C))),

            0xD9 => Some(Instruction::Return(ReturnInstruction::Reti)),

            _ => None,
        }
    }
}
