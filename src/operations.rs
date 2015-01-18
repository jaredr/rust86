use oplib;
use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::ModrmResult;


pub fn ret(cs: &mut CpuState) {
    println!("(op) ret");
    oplib::ret(cs);
}

pub fn inc(cs: &mut CpuState, reg: Reg16) {
    println!("(op) inc");
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_add(cs, cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn dec(cs: &mut CpuState, reg: Reg16) {
    println!("(op) dec");
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_sub(cs, cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn stc(cs: &mut CpuState) {
    println!("(op) stc");
    cs.set_carry();
}

pub fn push(cs: &mut CpuState, reg: Reg16) {
    println!("(op) push");
    let cur_val = cs.getreg_w(&reg);
    oplib::push(cs, cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    println!("(op) pop");
    let popped_val = oplib::pop(cs);
    cs.setreg_w(&reg, popped_val);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    println!("(op) b_jmp");
    oplib::jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    println!("(op) w_jmp");
    oplib::jump_w(cs, immediate);
}

pub type FlagFn = fn(&CpuState) -> bool;

pub fn b_jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    println!("(op) b_jmp_flag");
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    oplib::jump_b(cs, immediate);
}

pub fn b_jmp_flags(cs: &mut CpuState, flag0_fn: FlagFn, flag1_fn: FlagFn, invert: bool, immediate: Byte) {
    println!("(op) b_jmp_inv_flags");
    let flag0_value = flag0_fn(cs);
    let flag1_value = flag1_fn(cs);
    let flags_value = (flag0_fn(cs) || flag1_fn(cs));

    if !(flags_value ^ invert) {
        return;
    }

    oplib::jump_b(cs, immediate);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    println!("(op) call");
    oplib::call(cs, immediate);
}

pub fn b_add(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_add");
    let cur_val = cs.getreg_b(&reg);
    let new_val = oplib::b_add(cs, cur_val, immediate);
    cs.setreg_b(&reg, new_val);
}

pub fn w_add(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_add");
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_add(cs, cur_val, immediate);
    cs.setreg_w(&reg, new_val);
}

pub fn w_sub_ri(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_sub_ri");
    let reg_val = cs.getreg_w(&reg);
    let new_val = oplib::w_sub(cs, reg_val, immediate);
    cs.setreg_w(&reg, new_val);
}

pub fn b_cmp_ri(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_cmp_ri");
    let reg_val = cs.getreg_b(&reg);
    oplib::b_sub(cs, reg_val, immediate);
}

pub fn w_cmp_ri(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_cmp_ri");
    let reg_val = cs.getreg_w(&reg);
    oplib::w_sub(cs, reg_val, immediate);
}

pub fn b_mov_ir(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_mov_ir");
    cs.setreg_b(&reg, immediate);
}

pub fn w_mov_ir(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_mov_ir");
    cs.setreg_w(&reg, immediate);
}

pub fn b_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    println!("(op) b_mov_ge");
    let dest_value = oplib::modrm_reg8(dest.unwrap_register());
    let src_value = oplib::modrm_value_b(cs, &src);
    cs.setreg_b(&dest_value, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    println!("(op) w_mov_ge");

    let dest = oplib::modrm_reg16(dest.unwrap_register());
    let src_value = oplib::modrm_value_w(cs, &src);
    cs.setreg_w(&dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    println!("(op) b_mov_eg");

    let src_value = oplib::modrm_value_b(cs, &src);
    oplib::modrm_set_b(cs, &dest, src_value);
}

pub fn w_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    println!("(op) w_mov_eg");

    let src_value = oplib::modrm_value_w(cs, &src);
    oplib::modrm_set_w(cs, &dest, src_value);
}

pub fn b_xchg_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) b_xchg_eg");

    let left_value = oplib::modrm_value_b(cs, &left);
    let right_value = oplib::modrm_value_b(cs, &right);
    oplib::modrm_set_b(cs, &left, right_value);
    oplib::modrm_set_b(cs, &right, left_value);
}

pub fn w_or_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_or_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_or(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_xor_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_xor_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_xor(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_add_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_add_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_add(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_sbb_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_sbb_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);
    let carry_value = match cs.carry() {
        true => 1,
        false => 0,
    };

    let result = oplib::w_sub(cs, left_value, right_value + carry_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_sub_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_sub_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);
    let result = oplib::w_sub(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn b_cmp_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) b_cmp_eg");

    let left_value = oplib::modrm_value_b(cs, &left);
    let right_value = oplib::modrm_value_b(cs, &right);
    oplib::b_sub(cs, left_value, right_value);
}

pub fn w_cmp_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) w_cmp_eg");

    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);
    oplib::w_sub(cs, left_value, right_value);
}

pub fn w_adc_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    println!("(op) w_adc_ei");

    let effective_value = oplib::modrm_value_w(cs, &effective);
    let carry_value = match cs.carry() {
        true => 1,
        false => 0,
    };

    let result = oplib::w_add(cs, effective_value, immediate + carry_value);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn w_add_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    println!("(op) w_add_ei");

    let effective_value = oplib::modrm_value_w(cs, &effective);
    let result = oplib::w_add(cs, effective_value, immediate);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn b_cmp_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    println!("(op) b_cmp_ei");

    let effective = oplib::modrm_value_b(cs, &effective);
    oplib::b_sub(cs, effective, immediate);
}

pub fn b_or_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    println!("(op) b_or_ei");

    let effective_value = oplib::modrm_value_b(cs, &effective);
    let result = oplib::b_or(cs, effective_value, immediate);
    oplib::modrm_set_b(cs, &effective, result);
}

pub fn w_sub_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    println!("(op) w_sub_ei");

    let effective_value = oplib::modrm_value_w(cs, &effective);
    let result = oplib::w_sub(cs, effective_value, immediate);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn w_cmp_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    println!("(op) w_cmp_ei");

    let effective = oplib::modrm_value_w(cs, &effective);
    oplib::w_sub(cs, effective, immediate);
}

pub fn b_mov_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    println!("(op) b_mov_ei");
    oplib::modrm_set_b(cs, &effective, immediate);
}

pub fn w_mov_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    println!("(op) w_mov_ei");
    oplib::modrm_set_w(cs, &effective, immediate);
}

pub fn b_inc_e(cs: &mut CpuState, effective: ModrmResult) {
    println!("(op) b_inc_e");

    let cur_val = oplib::modrm_value_b(cs, &effective);
    let new_val = oplib::b_add(cs, cur_val, 1);
    oplib::modrm_set_b(cs, &effective, new_val);
}

pub fn b_dec_e(cs: &mut CpuState, effective: ModrmResult) {
    println!("(op) b_dec_e");

    let cur_val = oplib::modrm_value_b(cs, &effective);
    let new_val = oplib::b_sub(cs, cur_val, 1);
    oplib::modrm_set_b(cs, &effective, new_val);
}
