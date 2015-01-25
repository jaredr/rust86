use oplib;
use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::ModrmResult;
use operand::{
    Operand,
    Flags,
    b_operand_value,
    b_operand_set,
    w_operand_value,
    w_operand_set,
};


pub type transform8 = fn(left: Byte, right: Byte, flags: Flags) -> (Byte, Flags);
pub type transform16 = fn(left: Word, right: Word, flags: Flags) -> (Word, Flags);
 

fn operation_byte(cs: &mut CpuState,
                  dest: Operand,
                  src: Operand,
                  tf: transform8,
                  dry: bool) {
    // Boil src and dest down to actual Byte values
    let dest_val = b_operand_value(cs, &dest);
    let src_val = b_operand_value(cs, &src);

    // Run the transform to get the new value for dest
    let flags_in = cs.get_flags();
    let (result_val, flags) = tf(dest_val, src_val, flags_in);

    // Now assign that value to dest, and set flags
    cs.set_flags(flags.carry, flags.overflow, flags.sign, flags.zero);
    if !dry {
        b_operand_set(cs, &dest, result_val);
    }
}

fn operation_word(cs: &mut CpuState,
                  dest: Operand,
                  src: Operand,
                  tf: transform16,
                  dry: bool) {
    // Boil src and dest down to actual Word values
    let dest_val = w_operand_value(cs, &dest);
    let src_val = w_operand_value(cs, &src);

    // Run the transform to get the new value for dest
    let flags_in = cs.get_flags();
    let (result_val, flags) = tf(dest_val, src_val, flags_in);

    // Now assign that value to dest, and set flags
    cs.set_flags(flags.carry, flags.overflow, flags.sign, flags.zero);
    if !dry {
        w_operand_set(cs, &dest, result_val);
    }
}

pub fn b_op(cs: &mut CpuState, dest: Operand, src: Operand, tf: transform8) {
    operation_byte(cs, dest, src, tf, false);
}

pub fn b_op_dry(cs: &mut CpuState, dest: Operand, src: Operand, tf: transform8) {
    operation_byte(cs, dest, src, tf, true);
}

pub fn w_op(cs: &mut CpuState, dest: Operand, src: Operand, tf: transform16) {
    operation_word(cs, dest, src, tf, false);
}

pub fn w_op_dry(cs: &mut CpuState, dest: Operand, src: Operand, tf: transform16) {
    operation_word(cs, dest, src, tf, true);
}
