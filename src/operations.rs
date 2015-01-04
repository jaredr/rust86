use oplib;
use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm;
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

pub fn jz(cs: &mut CpuState, immediate: Byte) {
    println!("(op) jz");
    if !cs.zero() {
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

pub fn mov_e(cs: &mut CpuState, effective: ModrmResult, reg: ModrmResult) {
    println!("(op) mov_e");

    // TODO - Accept as method argument; should not call cs.read_* from here
    let src: Byte = cs.read_b();

    match effective {
        ModrmResult::MemoryAddr(x) => {
            let addr = oplib::modrm_addr(cs, x);
            cs.setmem(addr, src);
        },
        ModrmResult::Register(x) => {
            let reg = oplib::modrm_reg8(&x);
            cs.setreg_b(&reg, src);
        },
    }
}

pub fn b_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    println!("(op) b_mov_ge");

    let dest = oplib::modrm_reg8(dest.unwrap_register());
    let src_value = oplib::modrm_value_b(cs, src);
    cs.setreg_b(&dest, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState, src: ModrmResult, dest: ModrmResult) {
    println!("(op) w_mov_ge");

    let dest = oplib::modrm_reg16(dest.unwrap_register());
    let src_value = oplib::modrm_value_w(cs, src);
    cs.setreg_w(&dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    println!("(op) b_mov_eg");

    let src_value = oplib::modrm_value_b(cs, src);

    match dest {
        ModrmResult::MemoryAddr(x) => {
            let addr = oplib::modrm_addr(cs, x);
            cs.setmem(addr, src_value);
        },
        ModrmResult::Register(x) => {
            let reg = oplib::modrm_reg8(&x);
            cs.setreg_b(&reg, src_value);
        },
    };
}

pub fn w_mov_eg(cs: &mut CpuState, dest: ModrmResult, src: ModrmResult) {
    println!("(op) w_mov_eg");

    let src_value = oplib::modrm_value_w(cs, src);

    match dest {
        ModrmResult::MemoryAddr(x) => {
            let addr = oplib::modrm_addr(cs, x);

            // I'm pretty sure this doesn't work this way...
            cs.setmem(addr, byteutils::high8(src_value));
            cs.setmem(addr + 1, byteutils::low8(src_value));
        },
        ModrmResult::Register(x) => {
            let reg = oplib::modrm_reg16(&x);
            cs.setreg_w(&reg, src_value);
        }
    };
}

pub fn b_cmp_eg(cs: &mut CpuState, left: ModrmResult, right: ModrmResult) {
    println!("(op) b_cmp_eg");

    let right_value = oplib::modrm_value_b(cs, right);
    let left_value = oplib::modrm_value_b(cs, left);
    oplib::b_sub(cs, left_value, right_value);
}

pub fn b_cmp_ei(cs: &mut CpuState, effective: ModrmResult) {
    println!("(op) b_cmp_ei");

    let effective = oplib::modrm_value_b(cs, effective);

    // TODO - Accept as method argument; should not call cs.read_* from here
    let immediate = cs.read_b();

    oplib::b_sub(cs, effective, immediate);
}
