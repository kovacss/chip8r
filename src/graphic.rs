use std::io::Stdout;
use std::io::Write;

pub const WIDTH: u16 = 64;
pub const HEIGHT: u16 = 32;

const SPRITES: &[u8] = &[
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

pub fn get_sprites() -> &'static [u8]  {
    SPRITES
}


use std::io::stdout;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Colorize}
};

pub struct Display {
    stdout: Stdout
}

impl Display {
    pub fn new() -> Self {
        Display {
            stdout: stdout()
        }
    }

    pub fn clear_screen(&mut self) {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16) {
        self.stdout
        .queue(cursor::MoveTo(x,y)).unwrap()
        .queue(style::PrintStyledContent( "█".white())).unwrap();
        self.stdout
        .queue(cursor::MoveTo(x+1,y)).unwrap()
        .queue(style::PrintStyledContent( "█".white())).unwrap();
    }

    pub fn apply(&mut self) {
        self.stdout.flush().unwrap();
    }
}
  
pub fn draw_screen(display: &mut Display, screen: &Vec<bool>) {
    // stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let pixel = screen[usize::from(x + y * WIDTH)];
        if pixel == true {
          // in this loop we are more efficient by not flushing the buffer.
          display.draw_pixel(x, y);
        }
      }
    }

    display.apply();
}

pub fn update_screen(start_x: u16, start_y: u16, bytes_to_read: u16, base_address: u16, memory: &Vec<u8>, screen: &mut Vec<bool>) -> bool{
    let mut collision = false;

    // println!("setting pixel x {} - y {} - bytes_to_read: {} - base_address: {}", start_x, start_y, bytes_to_read, base_address);
    for idx in 0..bytes_to_read {
        let sprite = memory[usize::from(base_address + idx)];
        let y = (start_y + idx) % HEIGHT;

        for sprite_idx in 0..8 {
            let x = (start_x + sprite_idx) % WIDTH;
            let pixel_coordinate = x + (y * WIDTH);

            let bit = sprite >> (7 - sprite_idx) & 1;
            let existing_pixel = screen[usize::from(pixel_coordinate)];
            // println!("setting pixel x {} - y {} - cordinate {}  - previous_color: {} - new_color: {}", x, y, pixel_coordinate, existing_pixel, bit == 1);
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
        let memory = SPRITES.clone();
        let mut screen = screen_init();

        let (x, y) = (0, 0);
        let base_address = 0;
        let collision = update_screen(x, y, 2, base_address, &memory.to_vec(), &mut screen);

        println!("screen values {:?}", screen);
        assert_eq!(false, collision);
    }

    fn screen_init() -> Vec<bool> {
        vec![false; usize::from(WIDTH * HEIGHT)]
    }
}