use cstate::{CpuState, Reg16};
use datatypes::{Byte, Word};
use operand::{
    Operand,
    b_operand_value,
    b_operand_set,
    w_operand_value,
    w_operand_set,
};


pub type FlagFn = fn(&CpuState) -> bool;


pub fn push(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    cs.push(cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    let popped_val = cs.pop();
    cs.setreg_w(&reg, popped_val);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    let ip = cs.getreg_w(&Reg16::IP);
    cs.push(ip);
    jump_w(cs, immediate);
}

pub fn ret(cs: &mut CpuState) {
    let ip = cs.pop();
    cs.setreg_w(&Reg16::IP, ip);
}

pub fn b_xchg(cs: &mut CpuState, left: Operand, right: Operand) {
    let left_val = b_operand_value(cs, &left);
    let right_val = b_operand_value(cs, &right);
    b_operand_set(cs, &left, right_val);
    b_operand_set(cs, &right, left_val);
}

pub fn w_xchg(cs: &mut CpuState, left: Operand, right: Operand) {
    let left_val = w_operand_value(cs, &left);
    let right_val = w_operand_value(cs, &right);
    w_operand_set(cs, &left, right_val);
    w_operand_set(cs, &right, left_val);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    jump_w(cs, immediate);
}

pub fn jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    jump_b(cs, immediate);
}

pub fn jmp_flags(cs: &mut CpuState, 
                 flag0_fn: FlagFn,
                 flag1_fn: FlagFn,
                 invert: bool,
                 immediate: Byte) {
    let flags_value = flag0_fn(cs) || flag1_fn(cs);
    if !(flags_value ^ invert) {
        return;
    }

    jump_b(cs, immediate);
}

pub fn stc(cs: &mut CpuState) {
    cs.set_carry();
}

fn jump_b(cs: &mut CpuState, offset: Byte) {
    let ip = cs.getreg_w(&Reg16::IP);
    let offset = offset.to_u16().unwrap();
    if offset < 127 {
        cs.setreg_w(&Reg16::IP, ip + offset);
    } else {
        cs.setreg_w(&Reg16::IP, ip - (256 - offset));
    }
}

fn jump_w(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg_w(&Reg16::IP);
    cs.setreg_w(&Reg16::IP, ip + offset);
}
