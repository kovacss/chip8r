use crossterm::terminal::enable_raw_mode;
use crossterm::event::poll;
use std::fs;
use std::time::Duration;
use std::{thread, time};

pub mod cpu;
pub mod opcodes;
pub mod graphic;

fn load_game(rom_path: &str) -> Vec<u8> {

  let rom_content = fs::read(rom_path);
  println!("Loading rom - {}", rom_path);
  
  match rom_content {
      Ok(game) => game,
      Err(msg) => {
          println!("Could not load rom {}", msg);
          panic!();
      }
  }
}



fn main() {
    enable_raw_mode().unwrap();

    let refresh_rate = time::Duration::from_millis(16);
    let opcodes = opcodes::initialise_opcodes();

    let mut cpu = cpu::CPU::new();
    let mut display = graphic::Display::new();

    println!("Loading game ..");
    cpu.memory.append(&mut load_game("keypadtest.rom"));
    let remaining_size = 0xFFF - cpu.memory.len();
    cpu.memory.append(&mut vec![0; usize::from(remaining_size)]);

    // display.clear_screen();

    loop {
        
        // for idx in 0..16 {
        //   print!("{}, ", cpu.registers[idx]);
        // }
        // print!(" -- [i] {}", cpu.i);
        // println!();

        let update_screen = opcodes::execute_op_code(&mut cpu, &opcodes);

        cpu.update_timers();

        cpu.pc += 2;
        
        if update_screen {
            graphic::draw_screen(&mut display, &cpu.screen);
            
            // println!("----------------------------------------------------------------");
            // for y in 0..graphic::HEIGHT {
            //     for x in 0..graphic::WIDTH {
            //         let pixel = cpu.screen[usize::from(x + y * graphic::WIDTH)];
            //         if pixel == true {
            //             print!("X");
            //         } else {
            //             print!(" ");
            //         }
            //     }
            //     println!();
            // }
            // println!("----------------------------------------------------------------");
        }

        thread::sleep(refresh_rate);
    }
}
