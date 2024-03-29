use super::memory::Chip8Memory;
use super::output::Chip8Pixel;

#[derive(Debug)]
pub struct Chip8Sprite {
    pub pixels: Vec<[Chip8Pixel; 8]>, // a sprite is made up of up to 15 rows of 8 pixels
}

impl Chip8Sprite {
    pub fn new(memory: &Chip8Memory, starting_addr: usize, sprite_height: usize) -> Chip8Sprite {
        assert!(sprite_height < 16);

        // create new blank instance
        let mut inst = Chip8Sprite {
            pixels: Vec::with_capacity(sprite_height),
        };

        // read the bytes that make up the sprite from memory
        let sprite_bytes = (0..sprite_height)
            .map(|x| starting_addr + x)
            .map(|addr| memory.get_memory_at(addr).clone());

        // iterate over the sprite bytes and convert them to pixel arrays (a bit of 1 -> White, 0 -> Black), and then push the bit array to the sprite instance
        for byte in sprite_bytes {
            // convert to bit array iterator
            let bit_arr = (0..8)
                .map(|bit| (byte & (0x80 >> bit)) > 0);

            // blank mutable pixel array
            let mut pixel_array = [Chip8Pixel::Black; 8];

            // go through the bit array and assign the appropriate values to pixel_array
            bit_arr
                .map(|bit| if bit { Chip8Pixel::White } else { Chip8Pixel::Black })
                .enumerate()
                .for_each(|(i, pixel)| pixel_array[i] = pixel);

            // push the pixel array to the sprite instance
            inst.pixels.push(pixel_array);
        }

        inst
    }
}