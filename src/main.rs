mod chip8;
use std::thread;
use std::time;

fn main() {
    let program = include_bytes!("program.ch8");
    let mut program = Vec::from(program);
    if program.len() % 2 == 1 { program.push(0); }
    let program = program
        .chunks(2)
        .map(|x| ((x[0] as u16) << 8) + (x[1] as u16))
        .collect::<Vec<_>>();

    let mut c8 = chip8::Chip8::new_with_program(&program);

    println!("{c8:?}");

    loop {
        for _ in 0..17 {
            c8.execute_next_instruction();
        }

        print!("\x1B[2J\x1B[1;1H");
        c8.output.print_display();

        thread::sleep(time::Duration::from_millis(33));
    }
}
