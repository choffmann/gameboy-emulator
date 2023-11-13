use std::num::ParseIntError;
use crate::cpu::cpu::CPU;

mod cpu;
mod memory;

fn main() {
    let boot_rom = read_boot_rom();
    let mut cpu = CPU::boot(boot_rom);
    loop {
        cpu.step();
    }
}

fn read_boot_rom() -> Vec<u8> {
    let file = std::fs::read_to_string("./rom/dmg_boot.txt").expect("Can't read file");
    return decode_hex(&file).unwrap();
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
