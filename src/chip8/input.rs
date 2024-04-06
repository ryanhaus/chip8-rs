#[derive(Debug)]
pub struct Chip8Input {
    keys_status: [bool; 16], // the status of every key, in order of 123C456D789EA0BF
}

impl Chip8Input {
    pub fn new() -> Chip8Input {
        Chip8Input {
            keys_status: [false; 16],
        }
    }

    // returns the key_status array immutably
    pub fn get_keys_status(&self) -> &[bool; 16] {
        &self.keys_status
    }

    // returns the key_status array mutably
    pub fn get_keys_status_mut(&mut self) -> &mut [bool; 16] {
        &mut self.keys_status
    }

    // gets the first currently pressed key (returns the position in the keys_status array)
    pub fn get_current_key(&self) -> Option<usize> {
        self.keys_status
            .iter()
            .enumerate()
            .filter(|(i, x)| **x == true)
            .map(|(i, x)| i)
            .next()
    }

    // awaits a keypress (until get_current_key != None) and returns the index of the key in the keys_status array
    pub fn await_key_press(&self) -> usize {
        while self.get_current_key() == None {}

        self.get_current_key().unwrap()
    }
}
