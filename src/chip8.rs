use self::output::Chip8Output;

pub mod cpu;
mod input;
mod memory;
mod output;
mod registers;
mod timers;
mod sprites;

use cpu::*;
use input::*;
use memory::*;
use output::*;
use registers::*;
use timers::*;

#[derive(Debug)]
pub struct Chip8 {
    pub memory: Chip8Memory,
    pub registers: Chip8Registers,
    pub timers: Chip8Timers,
    pub output: Chip8Output,
    pub input: Chip8Input,
}

impl Chip8 {
    // new, blank chip8 instance
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Chip8Memory::new(),
            registers: Chip8Registers::new(),
            timers: Chip8Timers::new(),
            output: Chip8Output::new(),
            input: Chip8Input::new(),
        }
    }

    // new chip8 instance with pre-loaded program
    pub fn new_with_program(program: &Vec<u16>) -> Chip8 {
        let mut inst = Chip8::new();

        inst.memory.load_program_into_mem(program);
        inst.memory.load_fonts_into_mem();

        inst
    }
}
