use cstate::{CpuState, Reg8, Reg16};
use datatypes::{Byte, Word};
use operand::{
    Operand,
    b_operand_value,
    b_operand_set,
    w_operand_value,
    w_operand_set,
};
use oplib;


pub type FlagFn = fn(&CpuState) -> bool;


pub fn push(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    oplib::push(cs, cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    let popped_val = oplib::pop(cs);
    cs.setreg_w(&reg, popped_val);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    oplib::call(cs, immediate);
}

pub fn ret(cs: &mut CpuState) {
    oplib::ret(cs);
}

pub fn xchg(cs: &mut CpuState, left: Operand, right: Operand) {
    let left_val = b_operand_value(cs, &left);
    let right_val = b_operand_value(cs, &right);
    b_operand_set(cs, &left, right_val);
    b_operand_set(cs, &right, left_val);
}

pub fn xchg_reg(cs: &mut CpuState, left: Reg16, right: Reg16) {
    let left_value = cs.getreg_w(&left);
    let right_value = cs.getreg_w(&right);
    cs.setreg_w(&left, right_value);
    cs.setreg_w(&right, left_value);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    oplib::jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    oplib::jump_w(cs, immediate);
}

pub fn jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    oplib::jump_b(cs, immediate);
}

pub fn jmp_flags(cs: &mut CpuState, flag0_fn: FlagFn, flag1_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag0_value = flag0_fn(cs);
    let flag1_value = flag1_fn(cs);
    let flags_value = (flag0_fn(cs) || flag1_fn(cs));

    if !(flags_value ^ invert) {
        return;
    }

    oplib::jump_b(cs, immediate);
}

pub fn stc(cs: &mut CpuState) {
    cs.set_carry();
}
