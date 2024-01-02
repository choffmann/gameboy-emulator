mod utils;

use std::ops::Deref;
use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct CPU(gameboy::cpu::cpu::CPU);

#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CPU {
        utils::set_panic_hook();
        let cpu = gameboy::cpu::cpu::CPU::default();
        return CPU(cpu);
    }

    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self)?)
    }
}

#[wasm_bindgen]
pub fn load_boot_rom() -> Vec<u8> {
    let boot_rom = "31FEFFAF21FF9F32CB7C20FB2126FF0E113E8032E20C3EF3E2323E77773EFCE0471104012110801ACD9500CD9600137BFE3420F311D80006081A1322230520F93E19EA1099212F990E0C3D2808320D20F92E0F18F3673E6457E0423E91E040041E020E0CF044FE9020FA0D20F71D20F20E13247C1E83FE6228061EC1FE6420067BE20C3E87E2F04290E0421520D205204F162018CB4F0604C5CB1117C1CB11170520F522232223C9CEED6666CC0D000B03730083000C000D0008111F8889000EDCCC6EE6DDDDD999BBBB67636E0EECCCDDDC999FBBB9333E3C42B9A5B9A5423C21040111A8001A13BE20FE237DFE3420F506197886230520FB8620FE3E01E050";
    boot_rom.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .map(|s| u8::from_str_radix(s.as_str(), 16).unwrap())
        .collect()
}
