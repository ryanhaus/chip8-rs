use self::output::Chip8Output;

pub mod cpu;
mod input;
mod memory;
mod output;
mod registers;
mod timers;

use cpu::*;
use input::*;
use memory::*;
use output::*;
use registers::*;
use timers::*;

#[derive(Debug)]
pub struct Chip8 {
    memory: Chip8Memory,
    registers: Chip8Registers,
    timers: Chip8Timers,
    output: Chip8Output,
    input: Chip8Input,
}

impl Chip8 {
    pub fn new(program: &Vec<u16>) -> Chip8 {
        let mut inst = Chip8 {
            memory: Chip8Memory::new(),
            registers: Chip8Registers::new(),
            timers: Chip8Timers::new(),
            output: Chip8Output::new(),
            input: Chip8Input::new(),
        };

        inst.memory.load_program_into_mem(program);

        inst
    }
}
