use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use lazy_static::lazy_static;
mod chip8;

lazy_static! {
    static ref CHIP8_INSTANCE: Mutex<chip8::Chip8> = {
        let c8 = chip8::Chip8::new();
        Mutex::new(c8)
    };
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_default_program() -> Vec<u16> {
    let program = include_bytes!("program.ch8");
    let mut program = Vec::from(program);
    if program.len() % 2 == 1 { program.push(0); }
    let program = program
        .chunks(2)
        .map(|x| ((x[0] as u16) << 8) + (x[1] as u16))
        .collect::<Vec<_>>();

    program
}

#[wasm_bindgen]
pub fn load_program(program: &[u16]) {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    let program = Vec::from(program);

    c8.memory.load_program_into_mem(&program);
}

#[wasm_bindgen]
pub fn execute_instructions(instr_count: usize) {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    for _ in 0..instr_count {
        c8.execute_next_instruction();
    }
}

#[wasm_bindgen]
pub fn get_display_as_str() -> String {
    let c8 = CHIP8_INSTANCE.lock().unwrap();

    c8.output.get_display_as_str()
}