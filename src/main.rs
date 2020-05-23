use crossterm::terminal::enable_raw_mode;
use std::fs;
use std::time::Duration;
use std::{thread, time};

pub mod cpu;
pub mod opcodes;
pub mod graphic;
pub mod instructions;
pub mod keyboard;

fn load_game(rom_path: &str) -> Vec<u8> {
    println!("Loading rom - {}", rom_path);
    let rom_content = fs::read(rom_path);
  
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
    cpu.memory.append(&mut load_game("./roms/keypadtest.rom"));
    let remaining_size = 0xFFF - cpu.memory.len();
    cpu.memory.append(&mut vec![0; usize::from(remaining_size)]);

    loop {
        
        if !keyboard::update_events(&mut cpu) {
            return
        }

       // cpu.dump_registers();

        let update_screen = opcodes::execute_op_code(&mut cpu, &opcodes);

        cpu.update_timers();

        cpu.pc += 2;
        
        if update_screen {
            graphic::draw_screen(&mut display, &cpu.screen);
        }

        thread::sleep(refresh_rate);
    }
}
