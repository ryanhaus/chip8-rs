#[derive(Debug)]
pub struct Chip8Timers {
    delay: u8, // NOTE: may be something else like u16
    sound: u8,
}

impl Chip8Timers {
    pub fn new() -> Chip8Timers {
        Chip8Timers { delay: 0, sound: 0 }
    }
}
