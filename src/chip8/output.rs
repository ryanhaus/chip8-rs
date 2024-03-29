use super::sprites::Chip8Sprite;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Chip8Pixel {
    White,
    Black,
}

#[derive(Debug)]
pub struct Chip8Output {
    pub display: [[Chip8Pixel; 64]; 32],
}

impl Chip8Output {
    // creates a new Chip8Output instance
    pub fn new() -> Chip8Output {
        Chip8Output {
            display: [[Chip8Pixel::Black; 64]; 32],
        }
    }

    // clears display (all set to black)
    pub fn clear_display(&mut self) {
        for pixel_row_mut in &mut self.display {
            for pixel_mut in pixel_row_mut {
                *pixel_mut = Chip8Pixel::Black;
            }
        }
    }

    // toggles a pixel on the display, returns true if the pixel was flipped from white to black
    pub fn toggle_pixel(&mut self, x: usize, y: usize) -> bool {
        assert!(x < 64);
        assert!(y < 32);

        // store original pixel value
        let orig_pixel = self.display[y][x];

        // flip the pixel
        self.display[y][x] = if orig_pixel == Chip8Pixel::White { Chip8Pixel::Black } else { Chip8Pixel::White };

        // if it was originally white, then return true
        orig_pixel == Chip8Pixel::White
    }

    // takes in a Chip8Sprite instance and draws it to the screen at the appropriate place, returns true if any pixels were flipped from white to black
    pub fn draw_sprite_on_display(&mut self, x: usize, y: usize, sprite: Chip8Sprite) -> bool {
        let mut flipped_from_white = false;

        for (i, pixel_row) in sprite.pixels.iter().enumerate() {
            for (j, &pixel) in pixel_row.iter().enumerate() {
                if pixel == Chip8Pixel::White {
                    flipped_from_white |= self.toggle_pixel(x + j, y + i);
                }
            }
        }

        flipped_from_white
    }

    // prints the display
    pub fn print_display(&self) {
        for pixel_row in &self.display {
            for &pixel in pixel_row {
                print!("{}", if pixel == Chip8Pixel::White { "▓▓" } else { "▒▒" });
            }

            println!();
        }
    }
}
