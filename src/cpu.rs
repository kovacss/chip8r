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

impl CPU {

    pub fn new() -> Self {
        CPU {
            memory: vec![0; 4096],
            pc: 0x200,
            stack: VecDeque::new(),
            sp: 0,
            registers: vec![0; 16],
            i: 0,
            screen: vec![false; 64*32]
        }
    }

    pub fn get_next_opcode(&self) -> u16 {
        let idx = usize::from(self.pc);
        let op_code =  ((self.memory[idx] as u16) << 8) | self.memory[idx + 1] as u16;
        op_code
    }
}