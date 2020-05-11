use std::fs;

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

pub mod cpu;
pub mod opcodes;
pub mod graphic;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.memory = graphic::get_sprites().to_vec();
    cpu.memory.resize(0x200, 0);

    println!("Len after loading sprites {}", cpu.memory.len());

    cpu.memory.append(&mut load_game("blitz.rom"));
    let opcodes = opcodes::initialise_opcodes();

    println!("Len after loading game {}", cpu.memory.len());

    while true {
        // let opcode = get_opcode(&cpu.memory, usize::from(cpu.pc));
        opcodes::execute_op_code(&mut cpu, &opcodes);

        cpu.pc += 2;
        
        // println!("-----END-----");
        // if (opcode & 0xF000) == 0xD000 {
            println!("----------------------------------------------------------------");
            for y in 0..graphic::HEIGHT {
                for x in 0..graphic::WIDTH {
                    let pixel = cpu.screen[usize::from(x + y * graphic::WIDTH)];
                    if pixel == true {
                        print!("X");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("----------------------------------------------------------------");
            // thread::sleep(time::Duration::from_millis(1400));
        // }
 
    }
}
