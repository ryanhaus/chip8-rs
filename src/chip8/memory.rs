static FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,		// 0
	0x20, 0x60, 0x20, 0x20, 0x70,		// 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0,		// 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0,		// 3
	0x90, 0x90, 0xF0, 0x10, 0x10,		// 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0,		// 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0,		// 6
	0xF0, 0x10, 0x20, 0x40, 0x40,		// 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0,		// 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0,		// 9
	0xF0, 0x90, 0xF0, 0x90, 0x90,		// A
	0xE0, 0x90, 0xE0, 0x90, 0xE0,		// B
	0xF0, 0x80, 0x80, 0x80, 0xF0,		// C
	0xE0, 0x90, 0x90, 0x90, 0xE0,		// D
	0xF0, 0x80, 0xF0, 0x80, 0xF0,		// E
	0xF0, 0x80, 0xF0, 0x80, 0x80		// F
];

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
            stack_ptr: 0x0EFF,
        }
    }

    // gets a specific memory address immutably
    pub fn get_memory_at(&self, addr: usize) -> &u8 {
        assert!(addr < 4096);

        &self.mem[addr]
    }

    // gets a specific memory address immutably, but as a 16-bit number (big-endian)
    pub fn get_memory_at_u16(&self, addr: usize) -> u16 {
        let higher = self.get_memory_at(addr).clone() as u16;
        let lower = self.get_memory_at(addr + 1).clone() as u16;

        // convert to 16bit using big endian
        (higher << 8) + lower
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

    // loads the font data into memory
    pub fn load_fonts_into_mem(&mut self) {
        FONT_DATA
            .iter()
            .zip(self.mem.iter_mut())
            .for_each(|(&byte, memory_location)| *memory_location = byte);
    }

    // pushes an 8-bit value to the stack
    pub fn push_to_stack_u8(&mut self, val: u8) {
        *self.get_memory_at_mut(self.stack_ptr as usize) = val;
        self.stack_ptr -= 1;
    }

    // pushes a 16-bit value to the stack (higher byte first, lower byte second)
    pub fn push_to_stack_u16(&mut self, val: u16) {
        // separate bytes
        let higher = ((val & 0xFF00) >> 8) as u8;
        let lower = (val & 0x00FF) as u8;

        // push to stack
        self.push_to_stack_u8(higher);
        self.push_to_stack_u8(lower);
    }

    // pops an 8-bit value from the stack
    pub fn pop_from_stack_u8(&mut self) -> u8 {
        self.stack_ptr += 1;
        self.get_memory_at(self.stack_ptr as usize).clone()
    }

    // pops a 16-bit value from the satck (lower byte first, higher byte second)
    pub fn pop_from_stack_u16(&mut self) -> u16 {
        // pop the bytes and convert to 16-bit for proceeding arithmetic
        let lower = self.pop_from_stack_u8() as u16;
        let higher = self.pop_from_stack_u8() as u16;

        // combine the bytes
        (higher << 8) + lower
    }
}
