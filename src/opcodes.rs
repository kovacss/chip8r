use crossterm::event::poll;
use crate::cpu::CPU;
use crate::instructions;
use crate::instructions::{Instruction};

pub struct InstructionLookup {
    pub instruction: Instruction,
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

fn parse_opcode<'a>(op_codes: &'a Vec<InstructionLookup>, opcode: &u16) -> Option<(&'a Instruction, Variables)> {
    let op_code = find_opcode_id(&op_codes, &opcode);
    if op_code == None {
        return None;
    }
    Some((
        op_code.unwrap(),
        parse_variables_from_op_code(opcode)
    ))
}

pub fn initialise_opcodes() -> Vec<InstructionLookup> {
    vec![
        // CLS
        InstructionLookup {
            instruction: Instruction::CLS,
            mask: 0xFFFF,
            id: 0x00E0
        },
        // RET
        InstructionLookup {
            instruction: Instruction::RET,
            mask: 0xFFFF,
            id: 0x00EE 
        },
        // JP addr
        InstructionLookup {
            instruction: Instruction::JP_ADDR,
            mask: 0xF000,
            id: 0x1000
        },
        // CALL addr
        InstructionLookup {
            instruction: Instruction::CALL_ADDR,
            mask: 0xF000,
            id: 0x2000 
        },
        // SE Vx, byte
        InstructionLookup {
            instruction: Instruction::SE_VX_BYTE,
            mask: 0xF000,
            id: 0x3000 
        },
        // SNE Vx, byte
        InstructionLookup {
            instruction: Instruction::SNE_VX_BYTE,
            mask: 0xF000,
            id: 0x4000 
        },
        // SE Vx, Vy
        InstructionLookup {
            instruction: Instruction::SE_VX_VY,
            mask: 0xF000,
            id: 0x5000 
        },
        // LD Vx, byte
        InstructionLookup {
            instruction: Instruction::LD_VX_BYTE,
            mask: 0xF000,
            id: 0x6000 
        },
        // ADD Vx, byte
        InstructionLookup {
            instruction: Instruction::ADD_VX_BYTE,
            mask: 0xF000,
            id: 0x7000 
        },
        // LD Vx, Vy
        InstructionLookup {
            instruction: Instruction::LD_VX_VY,
            mask: 0xF00F,
            id: 0x8001
        },
        // OR Vx, Vy
        InstructionLookup {
            instruction: Instruction::OR_VX_VY,
            mask: 0xF00F,
            id: 0x8002
        },
        // AND Vx, Vy
        InstructionLookup {
            instruction: Instruction::AND_VX_VY,
            mask: 0xF00F,
            id: 0x8002
        },
        // XOR Vx, Vy
        InstructionLookup {
            instruction: Instruction::XOR_VX_VY,
            mask: 0xF00F,
            id: 0x8003
        },
        // ADD Vx, Vy
        InstructionLookup {
            instruction: Instruction::ADD_VX_VY,
            mask: 0xF00F,
            id: 0x8004
        },
        // SUB Vx, Vy
        InstructionLookup {
            instruction: Instruction::SUB_VX_VY,
            mask: 0xF00F,
            id: 0x8005
        },
        // SHR Vx {, Vy}
        InstructionLookup {
            instruction: Instruction::SHR_VX_VY,
            mask: 0xF00F,
            id: 0x8006
        },
        // SUBN Vx, Vy
        InstructionLookup {
            instruction: Instruction::SUBN_VX_VY,
            mask: 0xF00F,
            id: 0x8007
        },
        // SHL Vx {, Vy}
        InstructionLookup {
            instruction: Instruction::SHL_VX_VY,
            mask: 0xF00F,
            id: 0x800E
        },
        // SNE Vx, Vy
        InstructionLookup {
            instruction: Instruction::SNE_VX_VY,
            mask: 0xF00F,
            id: 0x9000
        },
        // LD I, addr
        InstructionLookup {
            instruction: Instruction::LD_I_ADDR,
            mask: 0xF000,
            id: 0xA000
        },
        // JP V0, addr
        InstructionLookup {
            instruction: Instruction::JP_V0_ADDR,
            mask: 0xF000,
            id: 0xB000
        },
        // RND Vx, byte
        InstructionLookup {
            instruction: Instruction::RND_VX_BYTE,
            mask: 0xF000,
            id: 0xC000
        },
        // DRW Vx, Vy, nibble
        InstructionLookup {
            instruction: Instruction::DRW,
            mask: 0xF000,
            id: 0xD000
        },
        // SKP Vx
        InstructionLookup {
            instruction: Instruction::SKP_VX,
            mask: 0xF0FF,
            id: 0xE09E
        },
        // SKNP Vx
        InstructionLookup {
            instruction: Instruction::SKNP_VX,
            mask: 0xF0FF,
            id: 0xE0A1
        },
        // LD Vx, DT
        InstructionLookup {
            instruction: Instruction::LD_VX_DT,
            mask: 0xF0FF,
            id: 0xF007
        },
        // LD Vx, K
        InstructionLookup {
            instruction: Instruction::LD_VX_K,
            mask: 0xF0FF,
            id: 0xF00A
        },
        // LD DT, Vx
        InstructionLookup {
            instruction: Instruction::LD_DT_VX,
            mask: 0xF0FF,
            id: 0xF015 
        },
        // LD ST, Vx
        InstructionLookup {
            instruction: Instruction::LD_ST_VX,
            mask: 0xF0FF,
            id: 0xF018 
        },
        // ADD I, Vx
        InstructionLookup {
            instruction: Instruction::ADD_I_VX,
            mask: 0xF0FF,
            id: 0xF01E 
        },
        // LD F, Vx
        InstructionLookup {
            instruction: Instruction::LD_F_VX,
            mask: 0xF0FF,
            id: 0xF029 
        },
        // LD B, Vx
        InstructionLookup {
            instruction: Instruction::LD_B_VX,
            mask: 0xF0FF,
            id: 0xF033 
        },
        // LD [I], Vx
        InstructionLookup {
            instruction: Instruction::LD_I_VX,
            mask: 0xF0FF,
            id: 0xF055 
        },
        // LD Vx, [I]
        InstructionLookup {
            instruction: Instruction::LD_VX_I,
            mask: 0xF0FF,
            id: 0xF065 
        }
    ]
}

pub fn find_opcode_id<'a>(opcodes: &'a Vec<InstructionLookup>, opcode: &u16) -> Option<&'a Instruction> {
    for opcode_def in opcodes.iter() {
        if opcode & opcode_def.mask == opcode_def.id {
            return Some(&opcode_def.instruction);
        }
    }
    None
}

pub fn execute_op_code(cpu: &mut CPU, op_codes: &Vec<InstructionLookup>) -> bool {
    let opcode = cpu.get_next_opcode();
    let parse_result = parse_opcode(op_codes, &opcode);
    
    if parse_result.is_none() {
        return false;
    }

    let (instruction, variables) = parse_result.unwrap();

    // println!("Executing Instruction {:?} from opcode - {:?} - {:?} - {:?} - {:?} - {:?} - {:?}", instruction, opcode, variables.x, variables.y, variables.kk, variables.addr, variables.nibble);

    match instruction {
        Instruction::CLS => {
            return instructions::clear_screen(cpu);
        },
        Instruction::RET => {
            instructions::ret(cpu);
        },
        Instruction::JP_ADDR => {
            instructions::jp_addr(cpu, variables.addr);
        },
        Instruction::CALL_ADDR => {
            instructions::call_addr(cpu, variables.addr);
        },
        Instruction::SE_VX_BYTE => {
            instructions::se_xv_byte(cpu, variables.x, variables.kk);
        },
        Instruction::SNE_VX_BYTE => {
            instructions::sne_xv_byte(cpu, variables.x, variables.kk);
        },
        Instruction::SE_VX_VY => {
            instructions::se_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::LD_VX_BYTE => {
            instructions::ld_vx_byte(cpu, variables.x, variables.kk);
        },
        Instruction::ADD_VX_BYTE => {
            instructions::add_vx_byte(cpu, variables.x, variables.kk);
        },
        Instruction::LD_VX_VY => {
            instructions::ld_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::OR_VX_VY => {
            instructions::or_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::AND_VX_VY => {
            instructions::and_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::XOR_VX_VY => {
            instructions::xor_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::ADD_VX_VY => {
            instructions::add_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::SUB_VX_VY => {
            instructions::sub_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::SHR_VX_VY => {
            instructions::shr_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::SUBN_VX_VY => {
            instructions::subn_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::SHL_VX_VY => {
            instructions::shl_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::SNE_VX_VY => {
            instructions::sne_vx_vy(cpu, variables.x, variables.y);
        },
        Instruction::LD_I_ADDR => {
            instructions::ld_i_addr(cpu, variables.addr);
        },
        Instruction::JP_V0_ADDR => {
            instructions::jp_v0_addr(cpu, variables.addr);
        },
        Instruction::RND_VX_BYTE => {
            instructions::random_vx_byte(cpu, variables.x, variables.kk);
        },
        Instruction::DRW => {
            return instructions::draw(cpu, variables.x, variables.y, variables.nibble);
        }
        Instruction::SKP_VX => {
            instructions::skip_vx(cpu, variables.x);
        },
        Instruction::SKNP_VX => {
            instructions::skipn_vx(cpu, variables.x);
        },
        Instruction::LD_VX_K => {
            instructions::load_vx_k(cpu, variables.x);
        },
        Instruction::LD_B_VX => {
            instructions::load_bytes_vx(cpu, variables.x);
        },
        Instruction::LD_VX_I => {
            instructions::load_vx_i(cpu, variables.x);
        },
        Instruction::LD_I_VX => {
            instructions::load_i_vx(cpu, variables.x);
        },
        Instruction::LD_F_VX => {
            instructions::load_f_vx(cpu, variables.x);
        },
        Instruction::ADD_I_VX => {
            instructions::add_i_vx(cpu, variables.x);
        },
        Instruction::LD_ST_VX => {
            instructions::load_st_vx(cpu, variables.x);
        },
        Instruction::LD_DT_VX => {
            instructions::load_dt_vx(cpu, variables.x);
        },
        Instruction::LD_VX_DT => {
            instructions::load_vx_dt(cpu, variables.x);
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
        find_op_code_test(opcode, Instruction::CLS);
    }

    #[test]
    fn op_code_RET() {
        let opcode = 0x00EE;
        find_op_code_test(opcode, Instruction::RET);
    }

    #[test]
    fn op_code_JP_ADDR() {
        let opcode = 0x1000;
        find_op_code_test(opcode, Instruction::JP_ADDR);
    }

    #[test]
    fn op_code_CALL_ADDR() {
        let opcode = 0x2000;
        find_op_code_test(opcode, Instruction::CALL_ADDR);
    }

    fn find_op_code_test(opcode: u16, expected_op_code: Instruction) {
        let op_codes = initialise_opcodes();
        let op_code_id = find_opcode_id(&op_codes, &opcode).unwrap();
        assert_eq!(expected_op_code, *op_code_id);
    }

    #[test]
    fn parse_opcode_test() {
        let op_codes = initialise_opcodes();
        let (op_code, variable) = parse_opcode(&op_codes, &0x42FC).unwrap();

        assert_eq!(Instruction::SNE_VX_BYTE, *op_code);
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
