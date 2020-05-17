use std::collections::VecDeque;

pub struct CPU {
    // heap
    pub memory: Vec<u8>,
    pub pc: u16,

    // stack
    pub stack: VecDeque<u16>,
    pub sp: u8,
    
    // Regsiters
    pub registers: Vec<u16>,
    pub i: u16,

    // Screen 64*32
    pub screen: Vec<bool> 
}

use crate::graphic;

impl CPU {

    pub fn new() -> Self {
        let mut cpu = CPU {
            memory: vec![0; 4096],
            pc: 0x200,
            stack: VecDeque::new(),
            sp: 0,
            registers: vec![0; 16],
            i: 0,
            screen: vec![false; 64*32]
        };

        cpu.memory = graphic::get_sprites().to_vec();
        cpu.memory.resize(0x200, 0);

        cpu
    }

    pub fn get_next_opcode(&self) -> u16 {
        let idx = usize::from(self.pc);
        let op_code =  ((self.memory[idx] as u16) << 8) | self.memory[idx + 1] as u16;
        op_code
    }

    pub fn get_register_value(&self, reg_number: u8) -> u16 {
        self.registers[usize::from(reg_number)]
    }

    pub fn set_register_value(&mut self, reg_number: u8, value: u16) {
        self.registers[usize::from(reg_number)] = value;
    }

    pub fn set_regF(&mut self, value: u16) {
        self.registers[0xF] = value;
    }
}