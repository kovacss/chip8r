use crate::cpu::CPU;

const WIDTH: u16 = 64;
const HEIGHT: u16 = 32;

const Sprites: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // D
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub fn display_graphic(start_x: u16, start_y: u16, bytes_to_read: u16, base_address: u16, memory: &Vec<u8>, screen: &mut Vec<bool>) -> bool{
    let mut collision = false;

    for idx in base_address..bytes_to_read {
        let sprite = memory[usize::from(idx)];
        let y = start_y + (idx - base_address);

        for sprite_idx in 0..8 {
            let x = start_x + sprite_idx;
            let pixel_coordinate = x + (y * WIDTH);

            let bit = sprite >> (7 - sprite_idx) & 1;
            let existing_pixel = screen[usize::from(pixel_coordinate)];
            if (bit == 1) != existing_pixel {
                screen[usize::from(pixel_coordinate)] = bit == 1;
                if bit == 1 {
                    collision = true;
                }
            }
        }
    }
    collision
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_graphic_test() {
        let memory = Sprites.clone();
        let mut screen = screen_init();

        let (x, y) = (0, 0);
        let base_address = 0;
        let collision = display_graphic(x, y, 2, base_address, &memory.to_vec(), &mut screen);

        println!("screen values {:?}", screen);
        assert_eq!(false, collision);
    }

    fn screen_init() -> Vec<bool> {
        let mut screen: Vec<bool> = vec![false; usize::from(WIDTH * HEIGHT)];
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                screen[usize::from(x + y * WIDTH)] = false;
            }
        }
        screen
    }
}