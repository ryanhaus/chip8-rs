#[derive(Debug)]
pub struct Chip8Registers {
    v: [u8; 16],
    i: u16,
    pc: u16,
}

impl Chip8Registers {
    // creates new blank instance (all 0s)
    pub fn new() -> Chip8Registers {
        Chip8Registers { v: [0; 16], i: 0, pc: 0x200 }
    }

    // for getting a V register immutably
    pub fn get_v_register(&self, reg: usize) -> &u8 {
        assert!(reg < 16);

        &self.v[reg]
    }

    // for getting a V register mutably
    pub fn get_v_register_mut(&mut self, reg: usize) -> &mut u8 {
        assert!(reg < 16);

        &mut self.v[reg]
    }

    // for getting the I regsiter immmutably
    pub fn get_i_register(&self) -> &u16 {
        &self.i
    }

    // for getting the I regsiter mutably
    pub fn get_i_register_mut(&mut self) -> &mut u16 {
        &mut self.i
    }

    // for getting PC register immutably
    pub fn get_pc_register(&self) -> &u16 {
        &self.pc
    }

    // for getting PC register mutably
    pub fn get_pc_register_mut(&mut self) -> &mut u16 {
        &mut self.pc
    }

    // makes jumping easier
    pub fn jump_to(&mut self, addr: u16) {
        *self.get_pc_register_mut() = addr;
    }

    // skips the next instruction by increasing pc by 2
    pub fn skip_next_instr(&mut self) {
        *self.get_pc_register_mut() += 2;
    }
}
