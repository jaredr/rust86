use byteutils;
use cstate::{CpuState, Reg8, Reg16};
use cstate::Reg16::{IP, SP};
use datatypes::{Byte, Word};


/**
 * Push the given value onto the stack.
 */
pub fn push(cs: &mut CpuState, val: Word) {
    let low_b = byteutils::low8(val);
    let high_b = byteutils::high8(val);
    let sp = cs.getreg_w(&SP);
    cs.setmem(sp - 1, low_b);
    cs.setmem(sp - 2, high_b);
    cs.setreg_w(&SP, sp - 2);
}

/**
 * Pop and return the top value from the stack.
 */
pub fn pop(cs: &mut CpuState) -> Word {
    let sp = cs.getreg_w(&SP);
    let low_b = cs.getmem(sp);
    let high_b = cs.getmem(sp + 1);
    cs.setmem(sp, 0x0);
    cs.setmem(sp + 1, 0x0);
    cs.setreg_w(&SP, sp + 2);
    byteutils::join8(low_b, high_b)
}

/**
 * Move `ip` by the given twos-complement byte offset.
 */
pub fn jump_b(cs: &mut CpuState, offset: Byte) {
    let ip = cs.getreg_w(&IP);
    let offset = offset.to_u16().unwrap();
    if offset < 127 {
        cs.setreg_w(&IP, ip + offset);
    } else {
        cs.setreg_w(&IP, ip - (256 - offset));
    }
}

/**
 * Move `ip` by the given unsigned word offset.
 */
pub fn jump_w(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg_w(&IP);
    cs.setreg_w(&IP, ip + offset);
}

/**
 * Push `ip`, then jmp `offset`
 */
pub fn call(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg_w(&IP);
    push(cs, ip);
    jump_w(cs, offset);
}

/**
 * Pop the top value from the stack into `ip`.
 */
pub fn ret(cs: &mut CpuState) {
    let ip = pop(cs);
    cs.setreg_w(&IP, ip);
}

/**
 * Wrapper around byteutils::b_add that sets flags on this CpuState.
 */
pub fn b_add(cs: &mut CpuState, left: Byte, right: Byte) -> Byte {
    let (result, cf, of, sf, zf) = byteutils::b_add(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn w_add(cs: &mut CpuState, left: Word, right: Word) -> Word {
    let (result, cf, of, sf, zf) = byteutils::w_add(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn b_sub(cs: &mut CpuState, left: Byte, right: Byte) -> Byte {
    let (result, cf, of, sf, zf) = byteutils::b_sub(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn w_sub(cs: &mut CpuState, left: Word, right: Word) -> Word {
    let (result, cf, of, sf, zf) = byteutils::w_sub(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}
