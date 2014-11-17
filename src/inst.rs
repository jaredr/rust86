use cstate::*;
use alias::{Byte, Word};

pub fn inc_reg(memory: &mut CpuState, reg: Register) {
    println!("(op) inc_reg");
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + 1);
}

pub fn add_reg_word(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_word");
    let word = memory.read_w();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + word);
}

pub fn add_reg_byte(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_byte");
    let byte = memory.read_b();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + byte);
}

pub fn mov_reg_word(memory: &mut CpuState, reg: Register) {
    println!("(op) mov_reg_word");
    let word = memory.read_w();
    memory.setreg(reg, word);
}

pub fn jmp_byte(memory: &mut CpuState) {
    let dest: Byte = memory.read_b();

    // Cast u16 `ip` down to u8 so that `byte` can wrap at 255
    // I'm pretty sure this isn't how a CPU works, but I don't know
    // enough about CPUs to dispute it.
    let mut ip8 = memory.ip.to_u8().unwrap();
    let dest8 = dest.to_u8().unwrap();
    ip8 += dest8;
    memory.ip = ip8.to_u16().unwrap();

    println!("(op) jmp_byte: 0x{:X}", dest);
}

pub fn jmp_word(memory: &mut CpuState) {
    let dest: Word = memory.read_w();
    memory.ip += dest;

    println!("(op) jmp_word: 0x{:X}", dest);
}
