use cstate::*;
use modrm::*;
use alias::{Byte, Word};

pub fn inc(memory: &mut CpuState, reg: Register) {
    println!("(op) inc_reg");
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + 1);
}

pub fn b_add(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_byte");
    let byte = memory.read_b();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + byte);
}

pub fn w_add(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_word");
    let word = memory.read_w();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + word);
}

pub fn b_mov_r(memory: &mut CpuState, reg: Register) {
    println!("(op) mov_reg_byte");
    let byte = memory.read_b();
    memory.setreg(reg, byte);
}

pub fn w_mov_r(memory: &mut CpuState, reg: Register) {
    println!("(op) mov_reg_word");
    let word = memory.read_w();
    memory.setreg(reg, word);
}

pub fn mov_e(memory: &mut CpuState) {
    println!("(op) mov_modrm_byte");
    let (dest, _) = get_modrm(memory);

    let src: Byte = memory.read_b();
    match dest {
        ModrmMemoryAddr(x) => memory.setmem_b(x, src),
        ModrmRegister(x) => memory.setreg(x, src),
        ModrmNone => panic!("ModrmNone"),
    }
}

pub fn mov_ge(memory: &mut CpuState) {
    println!("(op) mov_mreg_modrm");

    let (src, dest) = get_modrm(memory);
    let src_value = match src {
        ModrmMemoryAddr(x) => memory.getmem_b(x),
        ModrmRegister(x) => memory.getreg(x),
        ModrmNone => panic!("ModrmNone"),
    };
    memory.setreg(dest, src_value);
}

pub fn mov_eg(memory: &mut CpuState) {
    println!("(op) mov_modrm_mreg");
    let (dest, src) = get_modrm(memory);

    let src_value = memory.getreg(src);
    match dest {
        ModrmMemoryAddr(x) => memory.setmem_b(x, src_value),
        ModrmRegister(x) => memory.setreg(x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn b_jmp(memory: &mut CpuState) {
    println!("(op) jmp_byte");
    let dest: Byte = memory.read_b();

    // Cast u16 `ip` down to u8 so that `byte` can wrap at 255
    // I'm pretty sure this isn't how a CPU works, but I don't know
    // enough about CPUs to dispute it.
    let mut ip8 = memory.ip.to_u8().unwrap();
    let dest8 = dest.to_u8().unwrap();
    ip8 += dest8;
    memory.ip = ip8.to_u16().unwrap();
}

pub fn w_jmp(memory: &mut CpuState) {
    println!("(op) jmp_word");
    let dest: Word = memory.read_w();
    memory.ip += dest;
}
