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
    kk: u16
}

fn parse_variables_from_op_code(opcode: &u16) -> Variables {
    Variables {
        addr: opcode & 0x0FFF,
        nibble: (*opcode & 0x000F) as u8,
        x: ((opcode & 0x0F00) >> 8) as u8,
        y: ((opcode & 0x00F0) >> 4) as u8,
        kk: (opcode & 0x00FF) as u16
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

pub fn execute_op_code(cpu: &mut CPU, op_codes: &Vec<OpCodeLookup>) -> bool {
    let opcode = cpu.get_next_opcode();
    let parse_result = parse_opcode(op_codes, &opcode);
    
    if parse_result.is_none() {
        return false;
    }

    let (instruction, variables) = parse_result.unwrap();

    // print!("Executing Instruction {:?} from opcode -{:4x?}", instruction, opcode);
    // print!("\t - addr: {:?}", variables.addr);
    // print!("\t - kk: {:?}", variables.kk);
    // print!("\t - nibble: {:?}", variables.nibble);
    // print!("\t - x: {:?}", variables.x);
    // println!("\t - y: {:?}", variables.y);

    match instruction {
        OpCode::RET => {
           cpu.pc = cpu.stack[0];
           cpu.pc -= 2;
           cpu.sp -= 1;
           println!("RET {} - SP {}", cpu.pc, cpu.sp);
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
            let reg_value = cpu.get_register_value(variables.x);
            if reg_value == variables.kk {
                cpu.pc += 2;
            }
        },
        OpCode::SNE_VX_BYTE => {
            let reg_value = cpu.get_register_value(variables.x);
            if reg_value != variables.kk {
                cpu.pc += 2;
            }
        },
        OpCode::SE_VX_VY => {
            let reg_value_x = cpu.get_register_value(variables.x);
            let reg_value_y = cpu.registers[usize::from(variables.y)];
            if reg_value_x == reg_value_y {
                cpu.pc += 2;
            }
        },
        OpCode::LD_VX_BYTE => {
            cpu.registers[usize::from(variables.x)] = variables.kk;
        },
        OpCode::ADD_VX_BYTE => {
            cpu.registers[usize::from(variables.x)] += variables.kk;
        },
        OpCode::LD_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_register_value(variables.y);
        },
        OpCode::OR_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_register_value(variables.x) | cpu.get_register_value(variables.y);
        },
        OpCode::AND_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_register_value(variables.x) & cpu.get_register_value(variables.y);
        },
        OpCode::XOR_VX_VY => {
            cpu.registers[usize::from(variables.x)] = cpu.get_register_value(variables.x) ^ cpu.get_register_value(variables.y);
        },
        OpCode::ADD_VX_VY => {
            let result: u16 = cpu.get_register_value(variables.x) + cpu.get_register_value(variables.y);
            if result > 255 {
                cpu.registers[usize::from(variables.x)] = result >> 8;
                cpu.set_regF(1);
            } else {
                cpu.registers[usize::from(variables.x)] += cpu.get_register_value(variables.y)
            }
        },
        OpCode::SUB_VX_VY => {
            let result: u16 = cpu.get_register_value(variables.x) - cpu.get_register_value(variables.y);
            if result > 0 {
                cpu.set_regF(1);
            } else {
                cpu.set_regF(0);
            }
            cpu.set_register_value(variables.x, result);
        },
        OpCode::SHR_VX_VY => {
            let lsb: u16 = cpu.get_register_value(variables.x) >> 5;
            cpu.set_regF(match lsb {
                1 => 1,
                _ => 0
            });
            cpu.registers[usize::from(variables.x)] /= 2;
        },
        OpCode::SUBN_VX_VY => {
            let result: u16 = cpu.get_register_value(variables.y) - cpu.get_register_value(variables.x);
            if result > 0 {
                cpu.set_regF(1);
            } else {
                cpu.set_regF(0);
            }
            cpu.set_register_value(variables.x, result);
        },
        OpCode::SHL_VX_VY => {
            let lsb: u16 = cpu.get_register_value(variables.x) >> 5;
            cpu.set_regF(match lsb {
                1 => 1,
                _ => 0
            });
            cpu.registers[usize::from(variables.x)] *= 2;
        },
        OpCode::SNE_VX_VY => {
            let reg_value_a = cpu.get_register_value(variables.x);
            let reg_value_b = cpu.get_register_value(variables.y);
            if reg_value_a != reg_value_b {
                cpu.pc += 2;
                cpu.pc -= 2;
            }
        },
        OpCode::LD_I_ADDR => {
            cpu.i = variables.addr;
        },
        OpCode::JP_V0_ADDR => {
            cpu.pc = cpu.registers[0] + variables.addr;
            cpu.pc -= 2;
        },
        OpCode::RND_VX_BYTE => {
            use rand::prelude::*;
            cpu.registers[usize::from(variables.x)] = (random::<u8>() & 0x00ff) as  u16 & variables.kk;
        },
        OpCode::DRW => {
            let start_x = cpu.get_register_value(variables.x);
            let start_y = cpu.get_register_value(variables.y);
            let collision = graphic::update_screen(start_x, start_y, variables.nibble.into(), cpu.i, &cpu.memory, &mut cpu.screen);
            cpu.set_regF(if collision {
                1
            } else {
                0
            });
            return true;
        }
        // TODO Key handling
        OpCode::SKP_VX => {
            // let key = 0; // TODO set Key
            // if key == cpu.registers[usize::from(variables.x)] {
                // cpu.pc += 2;
            // }
            cpu.pc -= 4;
        },
        // TODO Key handling
        OpCode::SKNP_VX => {
            // let key = 0; // TODO set Key
            // if key == cpu.registers[usize::from(variables.x)] {
                // cpu.pc += 2;
            // }
            cpu.pc -= 2
        },
        // TODO Key handling
        OpCode::LD_VX_K => {
            // let key = 0; // TODO set Key
            // if key == cpu.registers[usize::from(variables.x)] {
                // cpu.pc += 2;
            // }
            cpu.pc -= 2
        },
        OpCode::ADD_I_VX => {
            cpu.i = cpu.i + cpu.registers[usize::from(variables.x)];
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
