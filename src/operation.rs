use cstate::*;
use datatypes::{Byte, Word};
use operand::{
    Operand,
    Flags,
    operand_value8,
    operand_set8,
    operand_value16,
    operand_set16,
};


pub type Transform8 = fn(left: Byte, right: Byte, flags: Flags) -> (Byte, Flags);
pub type Transform16 = fn(left: Word, right: Word, flags: Flags) -> (Word, Flags);
 

fn operation_byte(cs: &mut CpuState,
                  dest: Operand,
                  src: Operand,
                  tf: Transform8,
                  dry: bool) {
    // Boil src and dest down to actual Byte values
    let dest_val = operand_value8(cs, &dest);
    let src_val = operand_value8(cs, &src);

    // Run the transform to get the new value for dest
    let flags_in = cs.get_flags();
    let (result_val, flags) = tf(dest_val, src_val, flags_in);

    // Now assign that value to dest, and set flags
    cs.set_flags(flags.carry, flags.overflow, flags.sign, flags.zero);
    if !dry {
        operand_set8(cs, &dest, result_val);
    }
}

fn operation_word(cs: &mut CpuState,
                  dest: Operand,
                  src: Operand,
                  tf: Transform16,
                  dry: bool) {
    // Boil src and dest down to actual Word values
    let dest_val = operand_value16(cs, &dest);
    let src_val = operand_value16(cs, &src);

    // Run the transform to get the new value for dest
    let flags_in = cs.get_flags();
    let (result_val, flags) = tf(dest_val, src_val, flags_in);

    // Now assign that value to dest, and set flags
    cs.set_flags(flags.carry, flags.overflow, flags.sign, flags.zero);
    if !dry {
        operand_set16(cs, &dest, result_val);
    }
}

pub fn op8(cs: &mut CpuState, dest: Operand, src: Operand, tf: Transform8) {
    operation_byte(cs, dest, src, tf, false);
}

pub fn op8_dry(cs: &mut CpuState, dest: Operand, src: Operand, tf: Transform8) {
    operation_byte(cs, dest, src, tf, true);
}

pub fn op16(cs: &mut CpuState, dest: Operand, src: Operand, tf: Transform16) {
    operation_word(cs, dest, src, tf, false);
}

pub fn op16_dry(cs: &mut CpuState, dest: Operand, src: Operand, tf: Transform16) {
    operation_word(cs, dest, src, tf, true);
}
