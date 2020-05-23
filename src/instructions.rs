use crate::cpu::CPU;
use crate::graphic;
use crate::keyboard;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Instruction {
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

pub fn clear_screen(cpu: &mut CPU) -> bool {
    cpu.clear_screen();
    return true;
}

pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.stack.pop_front().unwrap();
    cpu.sp -= 1;
}

pub fn jp_addr(cpu: &mut CPU, addr: u16) {
    cpu.pc = addr;
    cpu.pc -= 2;
}

pub fn call_addr(cpu: &mut CPU, addr: u16) {
    cpu.sp += 1;
    cpu.stack.push_front(cpu.pc);
    cpu.pc = addr;
    cpu.pc -= 2;
}

pub fn se_xv_byte(cpu: &mut CPU, x: u8, kk: u8)  {
    let reg_value = cpu.get_reg(x);
    if reg_value == kk {
        cpu.pc += 2;
    }
}

pub fn se_xv_vy(cpu: &mut CPU, x: u8, y: u8)  {
    let reg_value_x = cpu.get_reg(x);
    let reg_value_y = cpu.get_reg(y);
    if reg_value_x == reg_value_y {
        cpu.pc += 2;
    }
}

pub fn sne_xv_byte(cpu: &mut CPU, x: u8, kk: u8)  {
    let reg_value = cpu.get_reg(x);
    if reg_value != kk {
        cpu.pc += 2;
    }
}

pub fn se_vx_vy(cpu: &mut CPU, x: u8, y: u8)  {
    let reg_value_x = cpu.get_reg(x);
    let reg_value_y = cpu.get_reg(y);
    if reg_value_x == reg_value_y {
        cpu.pc += 2;
    }
}

pub fn ld_vx_byte(cpu: &mut CPU, x: u8, kk: u8) {
    cpu.registers[usize::from(x)] = kk
}

pub fn add_vx_byte(cpu: &mut CPU, x: u8, kk: u8) {
    let (result, _overflow) = cpu.registers[usize::from(x)].overflowing_add(kk);
    cpu.registers[usize::from(x)] = result;
}

pub fn ld_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.registers[usize::from(x)] = cpu.get_reg(y);
}

pub fn or_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.registers[usize::from(x)] = cpu.get_reg(x) | cpu.get_reg(y);
}

pub fn and_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.registers[usize::from(x)] = cpu.get_reg(x) & cpu.get_reg(y);
}

pub fn xor_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.registers[usize::from(x)] = cpu.get_reg(x) ^ cpu.get_reg(y);
}

pub fn add_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let (result, overflow) = cpu.get_reg(x).overflowing_add(cpu.get_reg(y));
    if overflow {
        cpu.set_reg_f(1);
    } else {
        cpu.set_reg_f(0);
    }
    cpu.registers[usize::from(x)] = result;
}

pub fn sub_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let reg_value_x =  cpu.get_reg(x);
    let reg_value_y = cpu.get_reg(y);
    let (result, _overflow) = reg_value_y.overflowing_sub(reg_value_x);
    if reg_value_x >= reg_value_y {
        cpu.set_reg_f(1);
    } else {
        cpu.set_reg_f(0);
    }
    cpu.set_register_value(x, result);
}

pub fn shr_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let quirks = true;

    let reg_idx = if quirks {
        x
    } else {
        y
    };

    cpu.set_reg_f(cpu.get_reg(reg_idx) & 0x01);
    cpu.registers[usize::from(x)] >>= 1;
}

pub fn subn_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let (result, _overflow) = cpu.get_reg(y).overflowing_sub(cpu.get_reg(x));
    if cpu.get_reg(x) >= cpu.get_reg(y) {
        cpu.set_reg_f(0);
    } else {
        cpu.set_reg_f(1);
    }
    cpu.set_register_value(x, result);
}

pub fn shl_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let quirks = true;

    let reg_idx = if quirks {
        x
    } else {
        y
    };

    cpu.set_reg_f((cpu.get_reg(reg_idx) >> 7) & 0x01);
    cpu.registers[usize::from(reg_idx)] <<= 1;
}

pub fn sne_vx_vy(cpu: &mut CPU, x: u8, y: u8) {
    let reg_value_a = cpu.get_reg(x);
    let reg_value_b = cpu.get_reg(y);
    if reg_value_a != reg_value_b {
        cpu.pc += 2;
    }
}

pub fn ld_i_addr(cpu: &mut CPU, addr: u16) {
    cpu.i = addr;
}

pub fn jp_v0_addr(cpu: &mut CPU, addr: u16) {
    cpu.pc = (cpu.registers[0] as u16) + addr;
    cpu.pc -= 2;
}

pub fn random_vx_byte(cpu: &mut CPU, x: u8, kk: u8) {
    use rand::prelude::*;
    cpu.registers[usize::from(x)] = random::<u8>() & kk;
}

pub fn draw(cpu: &mut CPU, x: u8, y: u8, nibble: u8) -> bool {
    let start_x = cpu.get_reg(x);
    let start_y = cpu.get_reg(y);
    let collision = graphic::update_screen(start_x.into(), start_y.into(), nibble.into(), cpu.i, &cpu.memory, &mut cpu.screen);
    cpu.set_reg_f(if collision {
        1
    } else {
        0
    });
    return true;
}

pub fn skip_vx(cpu: &mut CPU, x: u8) {
    match cpu.key_pressed {
        Some(key) => {
            if key == cpu.get_reg(x).into() {
                cpu.pc += 2;
            }
        }
        _ => {}
    }
}

pub fn skipn_vx(cpu: &mut CPU, x: u8) {
    let expected_key = cpu.get_reg(x);
    match cpu.key_pressed {
        Some(key) => {
            if key != expected_key.into() {
                cpu.pc += 2;
            }
        }
        _ => cpu.pc += 2
    }
}

use std::convert::TryInto;
pub fn load_vx_k(cpu: &mut CPU, x: u8) {
    keyboard::wait_for_key_events(cpu);
    cpu.set_register_value(x, cpu.key_pressed.unwrap().try_into().unwrap());
    cpu.key_pressed = None;
}

pub fn load_bytes_vx(cpu: &mut CPU, x: u8) {
    let reg_value = cpu.get_reg(x);
    cpu.update_memory(cpu.i, reg_value  / 100);
    cpu.update_memory(cpu.i + 1, reg_value % 100 / 10);
    cpu.update_memory(cpu.i + 2, reg_value % 10);
}

pub fn load_vx_i(cpu: &mut CPU, x: u8) {
    for idx in 0..(x + 1) {
        // println!("Getting Memory idx {} value {}", cpu.i + idx as u16, cpu.memory[usize::from(cpu.i + idx as u16)]);
        cpu.set_register_value(idx, cpu.memory[usize::from(cpu.i + idx as u16)].into());
    }
    // Quirks
    cpu.i += (x + 1) as u16;
}

pub fn load_i_vx(cpu: &mut CPU, x: u8) {
    let reg_idx = x;
    for idx in 0..(reg_idx + 1) {
        cpu.update_memory(cpu.i + idx as u16, cpu.get_reg(idx) as u8);
    }
    // Quirks
    cpu.i += (x + 1) as u16;
}

pub fn load_f_vx(cpu: &mut CPU, x: u8) {
    let reg_value = cpu.get_reg(x);
    cpu.i = (reg_value * 5).into();
}

pub fn add_i_vx(cpu: &mut CPU, x: u8) {
    cpu.i = cpu.i + cpu.registers[usize::from(x)] as u16;
}

pub fn load_st_vx(cpu: &mut CPU, x: u8) {
    cpu.st = cpu.get_reg(x).into();
}

pub fn load_dt_vx(cpu: &mut CPU, x: u8) {
    cpu.dt = cpu.get_reg(x).into();
}

pub fn load_vx_dt(cpu: &mut CPU, x: u8) {
    cpu.set_register_value(x, cpu.dt.try_into().unwrap());
}