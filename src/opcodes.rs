use crossterm::event::poll;
use crate::cpu::CPU;
use crate::graphic;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OpCode {
    CLS,
    RET,
    JP_ADDR,
    CALL_ADDR,
    SE_VX_BYTE,
    SE_VX_VY,
    LD_VX_BYTE,
    ADD_VX_BYTE,
    LD_VX_VY,
    OR_VX_VY,
    AND_VX_VY,
    XOR_VX_VY,
    ADD_VX_VY,
    SUB_VX_VY,
    SHR_VX_VY,
    SUBN_VX_VY,
    SHL_VX_VY,
    SNE_VX_VY,
    SNE_VX_BYTE,
    LD_I_ADDR,
    JP_V0_ADDR,
    RND_VX_BYTE,
    DRW,
    SKP_VX,
    SKNP_VX,
    LD_VX_DT,
    LD_VX_K,
    LD_DT_VX,
    LD_ST_VX,
    LD_B_VX,
    ADD_I_VX,
    LD_F_VX,
    LD_I_VX,
    LD_VX_I
}

pub struct OpCodeLookup {
    pub op_code: OpCode,
    pub mask: u16,
    pub id: u16
}


pub struct Variables { 
    // A 12-bit value, the lowest 12 bits of the instruction
    addr: u16, 
    // A 4-bit value, the lowest 4 bits of the instruction
    nibble: u8,
    // A 4-bit value, the lower 4 bits of the high byte of the instruction
    x: u8,
    // A 4-bit value, the upper 4 bits of the low byte of the instruction
    y: u8,
    // An 8-bit value, the lowest 8 bits of the instruction
    kk: u8
}

fn parse_variables_from_op_code(opcode: &u16) -> Variables {
    Variables {
        addr: opcode & 0x0FFF,
        nibble: (*opcode & 0x000F) as u8,
        x: ((opcode & 0x0F00) >> 8) as u8,
        y: ((opcode & 0x00F0) >> 4) as u8,
        kk: (opcode & 0x00FF) as u8
    }
}

fn parse_opcode<'a>(op_codes: &'a Vec<OpCodeLookup>, opcode: &u16) -> Option<(&'a OpCode, Variables)> {
    let op_code = find_opcode_id(&op_codes, &opcode);
    if op_code == None {
        return None;
    }
    Some((
        op_code.unwrap(),
        parse_variables_from_op_code(opcode)
    ))
}

pub fn initialise_opcodes() -> Vec<OpCodeLookup> {
    vec![
        // CLS
        OpCodeLookup {
            op_code: OpCode::CLS,
            mask: 0xFFFF,
            id: 0x00E0
        },
        // RET
        OpCodeLookup {
            op_code: OpCode::RET,
            mask: 0xFFFF,
            id: 0x00EE 
        },
        // JP addr
        OpCodeLookup {
            op_code: OpCode::JP_ADDR,
            mask: 0xF000,
            id: 0x1000
        },
        // CALL addr
        OpCodeLookup {
            op_code: OpCode::CALL_ADDR,
            mask: 0xF000,
            id: 0x2000 
        },
        // SE Vx, byte
        OpCodeLookup {
            op_code: OpCode::SE_VX_BYTE,
            mask: 0xF000,
            id: 0x3000 
        },
        // SNE Vx, byte
        OpCodeLookup {
            op_code: OpCode::SNE_VX_BYTE,
            mask: 0xF000,
            id: 0x4000 
        },
        // SE Vx, Vy
        OpCodeLookup {
            op_code: OpCode::SE_VX_VY,
            mask: 0xF000,
            id: 0x5000 
        },
        // LD Vx, byte
        OpCodeLookup {
            op_code: OpCode::LD_VX_BYTE,
            mask: 0xF000,
            id: 0x6000 
        },
        // ADD Vx, byte
        OpCodeLookup {
            op_code: OpCode::ADD_VX_BYTE,
            mask: 0xF000,
            id: 0x7000 
        },
        // LD Vx, Vy
        OpCodeLookup {
            op_code: OpCode::LD_VX_VY,
            mask: 0xF00F,
            id: 0x8001
        },
        // OR Vx, Vy
        OpCodeLookup {
            op_code: OpCode::OR_VX_VY,
            mask: 0xF00F,
            id: 0x8002
        },
        // AND Vx, Vy
        OpCodeLookup {
            op_code: OpCode::AND_VX_VY,
            mask: 0xF00F,
            id: 0x8002
        },
        // XOR Vx, Vy
        OpCodeLookup {
            op_code: OpCode::XOR_VX_VY,
            mask: 0xF00F,
            id: 0x8003
        },
        // ADD Vx, Vy
        OpCodeLookup {
            op_code: OpCode::ADD_VX_VY,
            mask: 0xF00F,
            id: 0x8004
        },
        // SUB Vx, Vy
        OpCodeLookup {
            op_code: OpCode::SUB_VX_VY,
            mask: 0xF00F,
            id: 0x8005
        },
        // SHR Vx {, Vy}
        OpCodeLookup {
            op_code: OpCode::SHR_VX_VY,
            mask: 0xF00F,
            id: 0x8006
        },
        // SUBN Vx, Vy
        OpCodeLookup {
            op_code: OpCode::SUBN_VX_VY,
            mask: 0xF00F,
            id: 0x8007
        },
        // SHL Vx {, Vy}
        OpCodeLookup {
            op_code: OpCode::SHL_VX_VY,
            mask: 0xF00F,
            id: 0x800E
        },
        // SNE Vx, Vy
        OpCodeLookup {
            op_code: OpCode::SNE_VX_VY,
            mask: 0xF00F,
            id: 0x9000
        },
        // LD I, addr
        OpCodeLookup {
            op_code: OpCode::LD_I_ADDR,
            mask: 0xF000,
            id: 0xA000
        },
        // JP V0, addr
        OpCodeLookup {
            op_code: OpCode::JP_V0_ADDR,
            mask: 0xF000,
            id: 0xB000
        },
        // RND Vx, byte
        OpCodeLookup {
            op_code: OpCode::RND_VX_BYTE,
            mask: 0xF000,
            id: 0xC000
        },
        // DRW Vx, Vy, nibble
        OpCodeLookup {
            op_code: OpCode::DRW,
            mask: 0xF000,
            id: 0xD000
        },
        // SKP Vx
        OpCodeLookup {
            op_code: OpCode::SKP_VX,
            mask: 0xF0FF,
            id: 0xE09E
        },
        // SKNP Vx
        OpCodeLookup {
            op_code: OpCode::SKNP_VX,
            mask: 0xF0FF,
            id: 0xE0A1
        },
        // LD Vx, DT
        OpCodeLookup {
            op_code: OpCode::LD_VX_DT,
            mask: 0xF0FF,
            id: 0xF007
        },
        // LD Vx, K
        OpCodeLookup {
            op_code: OpCode::LD_VX_K,
            mask: 0xF0FF,
            id: 0xF00A
        },
        // LD DT, Vx
        OpCodeLookup {
            op_code: OpCode::LD_DT_VX,
            mask: 0xF0FF,
            id: 0xF015 
        },
        // LD ST, Vx
        OpCodeLookup {
            op_code: OpCode::LD_ST_VX,
            mask: 0xF0FF,
            id: 0xF018 
        },
        // ADD I, Vx
        OpCodeLookup {
            op_code: OpCode::ADD_I_VX,
            mask: 0xF0FF,
            id: 0xF01E 
        },
        // LD F, Vx
        OpCodeLookup {
            op_code: OpCode::LD_F_VX,
            mask: 0xF0FF,
            id: 0xF029 
        },
        // LD B, Vx
        OpCodeLookup {
            op_code: OpCode::LD_B_VX,
            mask: 0xF0FF,
            id: 0xF033 
        },
        // LD [I], Vx
        OpCodeLookup {
            op_code: OpCode::LD_I_VX,
            mask: 0xF0FF,
            id: 0xF055 
        },
        // LD Vx, [I]
        OpCodeLookup {
            op_code: OpCode::LD_VX_I,
            mask: 0xF0FF,
            id: 0xF065 
        }
    ]
}

pub fn find_opcode_id<'a>(opcodes: &'a Vec<OpCodeLookup>, opcode: &u16) -> Option<&'a OpCode> {
    for opcode_def in opcodes.iter() {
        if opcode & opcode_def.mask == opcode_def.id {
            return Some(&opcode_def.op_code);
        }
    }
    None
}

use crossterm::{event::read, event::Event, event::KeyEvent, event::KeyCode};
fn update_keyboard(cpu: &mut CPU, key_pressed: char) {
  match key_pressed {
    '0' => cpu.key_pressed = Some(0x0),
    '1' => cpu.key_pressed = Some(0x1),
    '2' => cpu.key_pressed = Some(0x2),
    '3' => cpu.key_pressed = Some(0x3),
    '4' => cpu.key_pressed = Some(0x4),
    '5' => cpu.key_pressed = Some(0x5),
    '6' => cpu.key_pressed = Some(0x6),
    '7' => cpu.key_pressed = Some(0x7),
    '8' => cpu.key_pressed = Some(0x8),
    '9' => cpu.key_pressed = Some(0x9),
    'a' => cpu.key_pressed = Some(0xA),
    'b' => cpu.key_pressed = Some(0xB),
    'c' => cpu.key_pressed = Some(0xC),
    'd' => cpu.key_pressed = Some(0xD),
    'e' => cpu.key_pressed = Some(0xE),
    'f' => cpu.key_pressed = Some(0xF),
    _ => {
      // println!("key {} not found", key_pressed);
      cpu.key_pressed = None
    }
  }
}

use std::time::Duration;
fn update_events(cpu: &mut CPU) {
    let event_available = poll(Duration::from_millis(0));
    if let Ok(true) = event_available{
      let event = read().unwrap();
      match event  {
        Event::Key(KeyEvent { code, .. }) => {
          match code {
            KeyCode::Esc => {
              panic!();
            },
            KeyCode::Char(c) => {
              update_keyboard(cpu, c);
            }
            _ => {}
          }
        }
        _ => {}
      };
    }
}

use std::{thread, time};
fn wait_for_key_events(cpu: &mut CPU) {
    let refresh_rate = time::Duration::from_millis(16);
    
    loop {
        update_events(cpu);
        if let Some(_k) = cpu.key_pressed {
            cpu.key_pressed;
            break;
        }
    }
}

pub fn execute_op_code(cpu: &mut CPU, op_codes: &Vec<OpCodeLookup>) -> bool {
    update_events(cpu);

    let opcode = cpu.get_next_opcode();
    let parse_result = parse_opcode(op_codes, &opcode);
    
    if parse_result.is_none() {
        return false;
    }

    let (instruction, variables) = parse_result.unwrap();

    // println!("Executing Instruction {:?} from opcode - {:?} - {:?} - {:?} - {:?} - {:?} - {:?}", instruction, opcode, variables.x, variables.y, variables.kk, variables.addr, variables.nibble);
   

    match instruction {
        OpCode::CLS => {
            cpu.screen = vec![false; 64*32];
            cpu.clear_screen();
            return true;
        },
        OpCode::RET => {
           cpu.pc = cpu.stack.pop_front().unwrap();
        //    cpu.stack.clear();
        //    cpu.pc -= 2;
           cpu.sp -= 1;
        //    println!("RET {} - SP {}", cpu.pc, cpu.sp);
        },
        OpCode::JP_ADDR => {
            cpu.pc = variables.addr;
            cpu.pc -= 2;
        },
        OpCode::CALL_ADDR => {
            cpu.sp += 1;
            cpu.stack.push_front(cpu.pc);
            cpu.pc = variables.addr;
            cpu.pc -= 2;
        },
        OpCode::SE_VX_BYTE => {
            let reg_value = cpu.get_reg(variables.x);
            if reg_value == variables.kk {
                cpu.pc += 2;
            }
        },
        OpCode::SNE_VX_BYTE => {
            let reg_value = cpu.get_reg(variables.x);
            if reg_value != variables.kk {
                cpu.pc += 2;
            }
        },
        OpCode::SE_VX_VY => {
            let reg_value_x = cpu.get_reg(variables.x);
            let reg_value_y = cpu.get_reg(variables.y);
            if reg_value_x == reg_value_y {
                cpu.pc += 2;
            }
        },
        OpCode::LD_VX_BYTE => {
            cpu.registers[usize::from(variables.x)] = variables.kk;
        },
        OpCode::ADD_VX_BYTE => {
            let (result, _overflow) = cpu.registers[usize::from(variables.x)].overflowing_add(variables.kk);
            cpu.registers[usize::from(variables.x)] = result;
        },
        OpCode::LD_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_reg(variables.y);
        },
        OpCode::OR_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_reg(variables.x) | cpu.get_reg(variables.y);
        },
        OpCode::AND_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_reg(variables.x) & cpu.get_reg(variables.y);
        },
        OpCode::XOR_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_reg(variables.x) ^ cpu.get_reg(variables.y);
        },
        OpCode::ADD_VX_VY => {
            let (result, overflow) = cpu.get_reg(variables.x).overflowing_add(cpu.get_reg(variables.y));
            if overflow {
                cpu.set_regF(1);
            } else {
                cpu.set_regF(0);
                // cpu.registers[usize::from(variables.x)] += cpu.get_register_value(variables.y)
            }

            cpu.registers[usize::from(variables.x)] = result;
        },
        OpCode::SUB_VX_VY => {
            let reg_value_x =  cpu.get_reg(variables.x);
            let reg_value_y = cpu.get_reg(variables.y);
            let (result, overflow) = reg_value_y.overflowing_sub(reg_value_x);
            if reg_value_x >= reg_value_y {
                cpu.set_regF(1);
            } else {
                cpu.set_regF(0);
            }
            cpu.set_register_value(variables.x, result);
        },
        OpCode::SHR_VX_VY => {
            let quirks = true;

            let reg_idx = if quirks {
                variables.x
            } else {
                variables.y
            };

            cpu.set_regF(cpu.get_reg(reg_idx) & 0x01);
            cpu.registers[usize::from(variables.x)] >>= 1;
        },
        OpCode::SUBN_VX_VY => {
            let (result, overflow) = cpu.get_reg(variables.y).overflowing_sub(cpu.get_reg(variables.x));
            if cpu.get_reg(variables.x) >= cpu.get_reg(variables.y) {
                cpu.set_regF(0);
            } else {
                cpu.set_regF(1);
            }
            cpu.set_register_value(variables.x, result);
        },
        OpCode::SHL_VX_VY => {
            let quirks = true;

            let reg_idx = if quirks {
                variables.x
            } else {
                variables.y
            };

            cpu.set_regF((cpu.get_reg(reg_idx) >> 7) & 0x01);
            cpu.registers[usize::from(reg_idx)] <<= 1;
        },
        OpCode::SNE_VX_VY => {
            let reg_value_a = cpu.get_reg(variables.x);
            let reg_value_b = cpu.get_reg(variables.y);
            if reg_value_a != reg_value_b {
                cpu.pc += 2;
            }
        },
        OpCode::LD_I_ADDR => {
            cpu.i = variables.addr;
        },
        OpCode::JP_V0_ADDR => {
            cpu.pc = (cpu.registers[0] as u16) + variables.addr;
            cpu.pc -= 2;
        },
        OpCode::RND_VX_BYTE => {
            use rand::prelude::*;
            cpu.registers[usize::from(variables.x)] = random::<u8>() & variables.kk;
        },
        OpCode::DRW => {
            let start_x = cpu.get_reg(variables.x);
            let start_y = cpu.get_reg(variables.y);
            let collision = graphic::update_screen(start_x.into(), start_y.into(), variables.nibble.into(), cpu.i, &cpu.memory, &mut cpu.screen);
            cpu.set_regF(if collision {
                1
            } else {
                0
            });
            return true;
        }
        // TODO Key handling
        OpCode::SKP_VX => {
            match cpu.key_pressed {
                Some(key) => {
                    if key == cpu.get_reg(variables.x).into() {
                        // println!("KEY {} pressed",  key);
                        cpu.pc += 2;
                    }
                }
                _ => {}
            }
            // cpu.pc -= 4;
        },
        // TODO Key handling
        OpCode::SKNP_VX => {
            let expected_key = cpu.get_reg(variables.x);
            // println!("Key handled {}", expected_key);
            match cpu.key_pressed {
                Some(key) => {
                    if key != expected_key.into() {
                        // println!("KEY {} not pressed",  key);
                        cpu.pc += 2;
                    }
                }
                _ => cpu.pc += 2
            }
            // cpu.pc -= 2
        },
        OpCode::LD_VX_K => {
            use std::convert::TryInto;
            println!("Wait key event");
            wait_for_key_events(cpu);
            cpu.set_register_value(variables.x, cpu.key_pressed.unwrap().try_into().unwrap());
            cpu.key_pressed = None;
        },
        OpCode::LD_B_VX => {
            use std::convert::TryInto;
            let reg_value = cpu.get_reg(variables.x);
            // println!("reg_value {} cpu.i {}, {}", reg_value, cpu.i, cpu.memory.len());
            // let b: u8 = if reg_value < 100 {
            //     0
            // } else {
            //     (reg_value  / 100).try_into().unwrap()
            // };
            // let c: u8 = if reg_value < 10 || cpu.memory[usize::from(cpu.i)] == 0 {
            //     0
            // } else {    
            //     (((reg_value % 10) as u16) / ((cpu.memory[usize::from(cpu.i)] * 10) as u16)).try_into().unwrap()
            // };
            // let d: u8 = if reg_value < 10 || cpu.memory[usize::from(cpu.i + 1)] == 0 {
            //     reg_value as u8
            // } else {
            //     (((reg_value % 100) as u16) / ((cpu.memory[usize::from(cpu.i + 1)] * 10) as u16)).try_into().unwrap()
            // };
            cpu.update_memory(cpu.i, reg_value  / 100);
            cpu.update_memory(cpu.i + 1, reg_value % 100 / 10);
            cpu.update_memory(cpu.i + 2, reg_value % 10);
            // println!("a {}, b{}, c{}",  b, c, d);
        },
        OpCode::LD_VX_I => {
            let reg_idx = variables.x;
            for idx in 0..(reg_idx + 1) {
                // println!("Getting Memory idx {} value {}", cpu.i + idx as u16, cpu.memory[usize::from(cpu.i + idx as u16)]);
                cpu.set_register_value(idx, cpu.memory[usize::from(cpu.i + idx as u16)].into());
            }
            // Quirks
            cpu.i += (variables.x + 1) as u16;
        },
        OpCode::LD_I_VX => {
            let reg_idx = variables.x;
            for idx in 0..(reg_idx + 1) {
                cpu.update_memory(cpu.i + idx as u16, cpu.get_reg(idx) as u8);
            }
            // Quirks
            cpu.i += (variables.x + 1) as u16;
        },
        OpCode::LD_F_VX => {
            let reg_value = cpu.get_reg(variables.x);
            cpu.i = (reg_value * 5).into();
        },
        OpCode::ADD_I_VX => {
            cpu.i = cpu.i + cpu.registers[usize::from(variables.x)] as u16;
        },
        OpCode::LD_ST_VX => {
            cpu.st = cpu.get_reg(variables.x).into();
        },
        OpCode::LD_DT_VX => {
            cpu.dt = cpu.get_reg(variables.x).into();
        },
        OpCode::LD_VX_DT => {
            use std::convert::TryInto;
            cpu.set_register_value(variables.x, cpu.dt.try_into().unwrap());
        },
        _ => {
            println!("Instruction {:?} no implemented", instruction);
            panic!();
         }
    };
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::cpu;

    #[test]
    fn op_code_CLS() {
        let opcode = 0x00E0;
        find_op_code_test(opcode, OpCode::CLS);
    }

    #[test]
    fn op_code_RET() {
        let opcode = 0x00EE;
        find_op_code_test(opcode, OpCode::RET);
    }

    #[test]
    fn op_code_JP_ADDR() {
        let opcode = 0x1000;
        find_op_code_test(opcode, OpCode::JP_ADDR);
    }

    #[test]
    fn op_code_CALL_ADDR() {
        let opcode = 0x2000;
        find_op_code_test(opcode, OpCode::CALL_ADDR);
    }

    fn find_op_code_test(opcode: u16, expected_op_code: OpCode) {
        let op_codes = initialise_opcodes();
        let op_code_id = find_opcode_id(&op_codes, &opcode).unwrap();
        assert_eq!(expected_op_code, *op_code_id);
    }

    #[test]
    fn parse_opcode_test() {
        let op_codes = initialise_opcodes();
        let (op_code, variable) = parse_opcode(&op_codes, &0x42FC).unwrap();

        assert_eq!(OpCode::SNE_VX_BYTE, *op_code);
        assert_eq!(0x2, variable.x);
        assert_eq!(0x0F, variable.y);
        assert_eq!(0x0C, variable.nibble);
        assert_eq!(0x02FC, variable.addr);
        assert_eq!(0xFC, variable.kk);
    }

    #[test]
    fn execute_LD_VX_BYTE_test() {
        let mut cpu = cpu::CPU::new();
        let op_codes = initialise_opcodes();

        execute_op_code(&mut cpu, &op_codes, &0x6100);

        assert_eq![0x00, cpu.registers[1]];
    }

    #[test]
    fn execute_LD_I_ADDR_test() {
        let mut cpu = cpu::CPU::new();
        let op_codes = initialise_opcodes();

        execute_op_code(&mut cpu, &op_codes, &0xa2d8);

        assert_eq![0x2d8, cpu.i];
    }
}
