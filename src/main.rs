use crate::cpu::cpu::CPU;
use crate::cpu::instructions::Instruction;

mod cpu;
mod memory;

fn main() {
    let inst = Instruction::from_byte(0x80, false);
    let mut cpu = CPU::boot();
    cpu.execute(inst.unwrap());
}
