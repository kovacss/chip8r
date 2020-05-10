pub struct CPU {
    // heap
    pub memory: Vec<u8>,
    pub pc: u16,

    // stack
    pub stack: Vec<u16>,
    pub sp: u8,
    
    // Regsiters
    pub registers: Vec<u16>,
    pub i: u16,

    // Screen 64*32
    pub screen: Vec<u16> 
}

pub fn init_cpu() -> CPU {
    CPU {
        memory: vec![0; 4096],
        pc: 0,
        stack: vec![0; 16],
        sp: 0,
        registers: vec![0; 16],
        i: 0,
        screen: vec![0; 64*32]
    }
}