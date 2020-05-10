use std::fs;
use std::{thread, time};

fn load_game(romPath: &str) -> Vec<u8> {

    println!("{}", fs::read(romPath).unwrap().len());

    fs::read(romPath).unwrap()
}

fn get_opcode(memory: &Vec<u8>, idx: usize) -> u16 {
    let opCode =  ((memory[idx] as u16) << 8) | memory[idx + 1] as u16;
    opCode
}

pub mod cpu;
pub mod opcodes;
pub mod graphic;

fn main() {
    let mut cpu = cpu::init_cpu();
    cpu.memory = load_game("maze.rom");
    let opcodes = opcodes::initialise_opcodes();

    while true {
        let opcode = get_opcode(&cpu.memory, usize::from(cpu.pc));
        opcodes::execute_op_code(&mut cpu, &opcodes, &opcode);
        cpu.pc += 2;
        thread::sleep(time::Duration::from_millis(500));
    }
}
