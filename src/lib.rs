extern crate console_error_panic_hook;
extern crate wasm_log;
use std::panic;
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
pub fn init_debug() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_log::init(wasm_log::Config::default());
}

#[wasm_bindgen]
pub fn reset_inst() {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    *c8 = chip8::Chip8::new();
    c8.memory.load_fonts_into_mem();
}

#[wasm_bindgen]
pub fn reset_pc() {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    *c8.registers.get_pc_register_mut() = 0x200;
}

#[wasm_bindgen]
pub fn get_default_program() -> Vec<u16> {
    let program = include_bytes!("program.ch8");

    program_8_to_16(program)
}

#[wasm_bindgen]
pub fn program_8_to_16(program: &[u8]) -> Vec<u16> {
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

#[wasm_bindgen]
pub fn get_display_as_ints() -> Vec<u8> {
    let c8 = CHIP8_INSTANCE.lock().unwrap();

    c8.output.get_display_as_ints().concat()
}

#[wasm_bindgen]
pub fn update_keys_status(keys_status: &[usize]) {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    let keys_status_c8 = c8.input.get_keys_status_mut();

    for (i, &key) in keys_status.iter().enumerate() {
        let key_bool = if key == 1 { true } else { false };

        keys_status_c8[i] = key_bool;
    }
}

#[wasm_bindgen]
pub fn timer_tick_and_get_sound() -> bool {
    let mut c8 = CHIP8_INSTANCE.lock().unwrap();

    c8.timers.timer_tick();

    *c8.timers.get_sound() > 0
}