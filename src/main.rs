mod chip8;
use std::thread;
use std::time;

fn main() {


    let program = include_bytes!("program.ch8");
    let program = program
        .chunks(2)
        .map(|x| ((x[0] as u16) << 8) + (x[1] as u16))
        .collect::<Vec<_>>();

    let mut c8 = chip8::Chip8::new(&program);

    //c8.output.print_display();

    for i in 0..100 {
        c8.execute_next_instruction();
    }

    c8.output.print_display()
}
