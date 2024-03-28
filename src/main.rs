mod chip8;
use std::fs;

fn main() {


    let program = include_bytes!("program.ch8");
    let program = program
        .chunks(2)
        .map(|x| ((x[0] as u16) << 8) + (x[1] as u16))
        .collect::<Vec<_>>();

    let c8 = chip8::Chip8::new(&program);

    for opcode in program {
        println!("{opcode:04X}: {:?}", chip8::Chip8::opcode_to_instruction(opcode));
    }

    println!("{c8:?}");
}
