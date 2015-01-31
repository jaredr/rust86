use std::num::ToPrimitive;
use cstate::{CpuState, Reg16};
use datatypes::{Byte, Word};
use operand::{
    Operand,
    operand_value8,
    operand_set8,
    operand_value16,
    operand_set16,
};


pub type FlagFn = fn(&CpuState) -> bool;


pub fn push(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg16(&reg);
    cs.push(cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    let popped_val = cs.pop();
    cs.setreg16(&reg, popped_val);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    let ip = cs.getreg16(&Reg16::IP);
    cs.push(ip);
    jmp16(cs, immediate);
}

pub fn ret(cs: &mut CpuState) {
    let ip = cs.pop();
    cs.setreg16(&Reg16::IP, ip);
}

pub fn xchg8(cs: &mut CpuState, left: Operand, right: Operand) {
    let left_val = operand_value8(cs, &left);
    let right_val = operand_value8(cs, &right);
    operand_set8(cs, &left, right_val);
    operand_set8(cs, &right, left_val);
}

pub fn xchg16(cs: &mut CpuState, left: Operand, right: Operand) {
    let left_val = operand_value16(cs, &left);
    let right_val = operand_value16(cs, &right);
    operand_set16(cs, &left, right_val);
    operand_set16(cs, &right, left_val);
}

pub fn jmp8(cs: &mut CpuState, offset: Byte) {
    let ip = cs.getreg16(&Reg16::IP);
    let offset = offset.to_u16().unwrap();
    if offset < 127 {
        cs.setreg16(&Reg16::IP, ip + offset);
    } else {
        cs.setreg16(&Reg16::IP, ip - (256 - offset));
    }
}

pub fn jmp16(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg16(&Reg16::IP);
    cs.setreg16(&Reg16::IP, ip + offset);
}

pub fn jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    jmp8(cs, immediate);
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

    jmp8(cs, immediate);
}

pub fn stc(cs: &mut CpuState) {
    cs.set_carry();
}
