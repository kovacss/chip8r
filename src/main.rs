use std::fs;
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
    let refresh_rate = time::Duration::from_millis(16);
    let opcodes = opcodes::initialise_opcodes();

    let mut cpu = cpu::CPU::new();
    let mut display = graphic::Display::new();

    println!("Loading game ..");
    cpu.memory.append(&mut load_game("maze.rom"));

    display.clear_screen();

    loop {
        let update_screen = opcodes::execute_op_code(&mut cpu, &opcodes);

        cpu.pc += 2;
        
        if update_screen {
            graphic::draw_screen(&mut display, &cpu.screen);
        }

        thread::sleep(refresh_rate);
    }
}
