#[derive(Debug)]
pub struct Chip8Timers {
    delay: u8, // NOTE: may be something else like u16
    sound: u8,
}

impl Chip8Timers {
    pub fn new() -> Chip8Timers {
        Chip8Timers { delay: 0, sound: 0 }
    }

    // gets the delay timer immutably
    pub fn get_delay(&self) -> &u8 {
        &self.delay
    }

    // gets the delay timer mutably
    pub fn get_delay_mut(&mut self) -> &mut u8 {
        &mut self.delay
    }

    // gets the sound timer immutably
    pub fn get_sound(&self) -> &u8 {
        &self.sound
    }

    // gets the sound timer mutably
    pub fn get_sound_mut(&mut self) -> &mut u8 {
        &mut self.sound
    }

    // ticks the timers (decrases the timers by 1 if they are greater than zero)
    pub fn timer_tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}
