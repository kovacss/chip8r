use std::collections::VecDeque;

const SCREEN_SIZE: usize = 64 * 32;

pub struct CPU {
    // heap
    pub memory: Vec<u8>,
    pub pc: u16,

    // stack
    pub stack: VecDeque<u16>,
    pub sp: u8,
    
    // Regsiters
    pub registers: Vec<u8>,
    pub i: u16,

    // Screen 64*32
    pub screen: Vec<bool>,

    // key pressed
    pub key_pressed: Option<u16>,

    // Delay timer
    pub dt: u16,
    // Sound timer
    pub st: u16
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
            screen: vec![false; SCREEN_SIZE],
            key_pressed: None,
            dt: 0,
            st: 0
        };

        cpu.memory = graphic::get_sprites().to_vec();
        cpu.memory.resize(0x200, 0);

        cpu
    }

    pub fn clear_screen(&mut self) {
        self.screen = vec![false; SCREEN_SIZE];
    }

    pub fn get_next_opcode(&self) -> u16 {
        let idx = usize::from(self.pc);
        let op_code =  ((self.memory[idx] as u16) << 8) | self.memory[idx + 1] as u16;
        op_code
    }

    pub fn update_memory(&mut self, idx: u16, value: u8) {
        self.memory[usize::from(idx)] = value;
    }

    pub fn get_reg(&self, reg_number: u8) -> u8 {
        self.registers[usize::from(reg_number)]
    }

    pub fn set_register_value(&mut self, reg_number: u8, value: u8) {
        self.registers[usize::from(reg_number)] = value;
    }

    pub fn set_reg_f(&mut self, value: u8) {
        self.set_register_value(0xF, value);
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn dump_registers(&self) {
        for idx in 0..16 {
          print!("{}, ", self.registers[idx]);
        }
        print!(" -- [i] {}", self.i);
        println!();
    }
}