use std::fs;

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

fn main() {
    let game = cpu::init_cpu();
    let game = load_game("maze.rom");
    let opcodes= opcodes::initialise_opcodes();
    let mut idx = 0;
    while idx < game.len() {
        let opcode = get_opcode(&game, idx);
        let res = opcodes::find_opcode_id(&opcodes, &opcode);
        println!("OpCode found !{:4x?}", opcode);
        idx += 2;
    }
}
