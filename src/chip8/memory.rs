#[derive(Debug)]
pub struct Chip8Memory {
    mem: [u8; 4096],
    stack_ptr: u16,
}

impl Chip8Memory {
    // creates a new blank instance of memory (all 0s)
    pub fn new() -> Chip8Memory {
        Chip8Memory {
            mem: [0; 4096],
            stack_ptr: 0,
        }
    }

    // gets a specific memory address immutably
    pub fn get_memory_at(&self, addr: usize) -> &u8 {
        assert!(addr < 4096);

        &self.mem[addr]
    }

    // gets a specific memory address mutably
    pub fn get_memory_at_mut(&mut self, addr: usize) -> &mut u8 {
        assert!(addr < 4096);

        &mut self.mem[addr]
    }

    // loads the program into memory starting at 0x200
    pub fn load_program_into_mem(&mut self, program: &Vec<u16>) {
        program
            .iter()
            .map(|opcode| (((opcode & 0xFF00) >> 8) as u8, (opcode & 0x00FF) as u8)) // split into higher and lower bytes
            .enumerate()
            .for_each(|(i, (higher, lower))| {
                // program starts at addr 0x200, each opcode takes up 2 bytes
                let (high_addr, low_addr) = (0x200 + 2 * i, 0x201 + 2 * i);

                *self.get_memory_at_mut(high_addr) = higher;
                *self.get_memory_at_mut(low_addr) = lower;
            });
    }

    // pushes a value to the stack
    // pops a value from the stack
}
