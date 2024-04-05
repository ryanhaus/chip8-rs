use wasm_bindgen::prelude::*;
mod chip8;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn test_c8(instr_count: usize) -> String {
    let program = include_bytes!("program.ch8");
    let mut program = Vec::from(program);
    if program.len() % 2 == 1 { program.push(0); }
    let program = program
        .chunks(2)
        .map(|x| ((x[0] as u16) << 8) + (x[1] as u16))
        .collect::<Vec<_>>();

    let mut c8 = chip8::Chip8::new(&program);

    println!("{c8:?}");

    for _ in 0..instr_count {
        c8.execute_next_instruction();
    }
    
    c8.output.get_display_as_str()
}