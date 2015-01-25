use oplib;
use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::ModrmResult;
use operand::{Operand, Flags, b_operand_value, b_operand_set};


pub type transform8 = fn(left: Byte, right: Byte) -> (Byte, Option<Flags>);
pub type transform16 = fn(left: Word, right: Word) -> (Word, Option<Flags>);

pub fn b_op(cs: &mut CpuState,
            dest: Operand,
            src: Operand,
            tf: transform8) {
    // Boil src and dest down to actual u8 values
    let dest_val = b_operand_value(cs, &dest);
    let src_val = b_operand_value(cs, &src);

    // Run the transform to get the new value for dest
    let (result_val, flags) = tf(dest_val, src_val);

    // Now assign that value to `dest`, and set flags
    match flags {
        Some(x) => cs.set_flags(x.carry, x.overflow, x.sign, x.zero),
        None => {},
    }
    b_operand_set(cs, &dest, result_val);
}


macro_rules! define_transform (
    (
        $name:ident,
        $size:ident,
        $arithmetic_fn:expr
    ) => {
        pub fn $name(left: $size, right: $size) -> ($size, Option<Flags>) {
            let (result, cf, of, sf, zf) = $arithmetic_fn(left, right);
            let flags = Flags {
                carry: cf,
                overflow: of,
                sign: sf,
                zero: zf,
            };
            (result, Some(flags))
        }
    }
);

define_transform!(tf_b_add, Byte, byteutils::b_add);
define_transform!(tf_b_sub, Byte, byteutils::b_sub);
define_transform!(tf_b_or, Byte, byteutils::b_or);
define_transform!(tf_b_xor, Byte, byteutils::b_xor);
define_transform!(tf_b_and, Byte, byteutils::b_and);


pub fn ret(cs: &mut CpuState) {
    oplib::ret(cs);
}

pub fn inc(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_add(cs, cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn dec(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_sub(cs, cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn xchg(cs: &mut CpuState, left: Reg16, right: Reg16) {
    let left_value = cs.getreg_w(&left);
    let right_value = cs.getreg_w(&right);
    cs.setreg_w(&left, right_value);
    cs.setreg_w(&right, left_value);
}


pub fn stc(cs: &mut CpuState) {
    cs.set_carry();
}

pub fn push(cs: &mut CpuState, reg: Reg16) {
    let cur_val = cs.getreg_w(&reg);
    oplib::push(cs, cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    let popped_val = oplib::pop(cs);
    cs.setreg_w(&reg, popped_val);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    oplib::jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    oplib::jump_w(cs, immediate);
}

pub type FlagFn = fn(&CpuState) -> bool;

pub fn b_jmp_flag(cs: &mut CpuState, flag_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag_value = flag_fn(cs);
    if !(flag_value ^ invert) {
        return
    }

    oplib::jump_b(cs, immediate);
}

pub fn b_jmp_flags(cs: &mut CpuState, flag0_fn: FlagFn, flag1_fn: FlagFn, invert: bool, immediate: Byte) {
    let flag0_value = flag0_fn(cs);
    let flag1_value = flag1_fn(cs);
    let flags_value = (flag0_fn(cs) || flag1_fn(cs));

    if !(flags_value ^ invert) {
        return;
    }

    oplib::jump_b(cs, immediate);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    oplib::call(cs, immediate);
}

pub fn b_add(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    let cur_val = cs.getreg_b(&reg);
    let new_val = oplib::b_add(cs, cur_val, immediate);
    cs.setreg_b(&reg, new_val);
}

pub fn w_add(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    let cur_val = cs.getreg_w(&reg);
    let new_val = oplib::w_add(cs, cur_val, immediate);
    cs.setreg_w(&reg, new_val);
}

pub fn w_sub_ri(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    let reg_val = cs.getreg_w(&reg);
    let new_val = oplib::w_sub(cs, reg_val, immediate);
    cs.setreg_w(&reg, new_val);
}

pub fn b_cmp_ri(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    let reg_val = cs.getreg_b(&reg);
    oplib::b_sub(cs, reg_val, immediate);
}

pub fn w_cmp_ri(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    let reg_val = cs.getreg_w(&reg);
    oplib::w_sub(cs, reg_val, immediate);
}

pub fn b_mov_ir(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    cs.setreg_b(&reg, immediate);
}

pub fn w_mov_ir(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    cs.setreg_w(&reg, immediate);
}

pub fn b_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    let dest_value = oplib::modrm_reg8(dest.unwrap_register());
    let src_value = oplib::modrm_value_b(cs, &src);
    cs.setreg_b(&dest_value, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    let dest = oplib::modrm_reg16(dest.unwrap_register());
    let src_value = oplib::modrm_value_w(cs, &src);
    cs.setreg_w(&dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    let src_value = oplib::modrm_value_b(cs, &src);
    oplib::modrm_set_b(cs, &dest, src_value);
}

pub fn w_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    let src_value = oplib::modrm_value_w(cs, &src);
    oplib::modrm_set_w(cs, &dest, src_value);
}

pub fn b_xchg_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_b(cs, &left);
    let right_value = oplib::modrm_value_b(cs, &right);
    oplib::modrm_set_b(cs, &left, right_value);
    oplib::modrm_set_b(cs, &right, left_value);
}

pub fn w_or_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_or(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_and_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_and(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_xor_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_xor(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_add_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);

    let result = oplib::w_add(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn w_sbb_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
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
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);
    let result = oplib::w_sub(cs, left_value, right_value);
    oplib::modrm_set_w(cs, &left, result);
}

pub fn b_cmp_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_b(cs, &left);
    let right_value = oplib::modrm_value_b(cs, &right);
    oplib::b_sub(cs, left_value, right_value);
}

pub fn w_cmp_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    let left_value = oplib::modrm_value_w(cs, &left);
    let right_value = oplib::modrm_value_w(cs, &right);
    oplib::w_sub(cs, left_value, right_value);
}

pub fn w_adc_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    let effective_value = oplib::modrm_value_w(cs, &effective);
    let carry_value = match cs.carry() {
        true => 1,
        false => 0,
    };

    let result = oplib::w_add(cs, effective_value, immediate + carry_value);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn w_add_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    let effective_value = oplib::modrm_value_w(cs, &effective);
    let result = oplib::w_add(cs, effective_value, immediate);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn b_cmp_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    let effective = oplib::modrm_value_b(cs, &effective);
    oplib::b_sub(cs, effective, immediate);
}

pub fn b_or_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    let effective_value = oplib::modrm_value_b(cs, &effective);
    let result = oplib::b_or(cs, effective_value, immediate);
    oplib::modrm_set_b(cs, &effective, result);
}

pub fn w_sub_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    let effective_value = oplib::modrm_value_w(cs, &effective);
    let result = oplib::w_sub(cs, effective_value, immediate);
    oplib::modrm_set_w(cs, &effective, result);
}

pub fn w_cmp_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    let effective = oplib::modrm_value_w(cs, &effective);
    oplib::w_sub(cs, effective, immediate);
}

pub fn b_mov_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Byte) {
    oplib::modrm_set_b(cs, &effective, immediate);
}

pub fn w_mov_ei(cs: &mut CpuState, effective: ModrmResult, immediate: Word) {
    oplib::modrm_set_w(cs, &effective, immediate);
}

pub fn b_inc_e(cs: &mut CpuState, effective: ModrmResult) {
    let cur_val = oplib::modrm_value_b(cs, &effective);
    let new_val = oplib::b_add(cs, cur_val, 1);
    oplib::modrm_set_b(cs, &effective, new_val);
}

pub fn b_dec_e(cs: &mut CpuState, effective: ModrmResult) {
    let cur_val = oplib::modrm_value_b(cs, &effective);
    let new_val = oplib::b_sub(cs, cur_val, 1);
    oplib::modrm_set_b(cs, &effective, new_val);
}
