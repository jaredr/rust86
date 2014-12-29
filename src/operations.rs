use cpulib;
use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::*;
use modrm::ModrmValue::*;


pub fn ret(cs: &mut CpuState) {
    println!("(op) ret");
    cpulib::ret(cs);
}

pub fn inc(cs: &mut CpuState, reg: Reg16) {
    println!("(op) inc");
    let cur_val = cs.getreg_w(&reg);
    let new_val = cpulib::w_add(cs, cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn push(cs: &mut CpuState, reg: Reg16) {
    println!("(op) push");
    let cur_val = cs.getreg_w(&reg);
    cpulib::push(cs, cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    println!("(op) pop");
    let popped_val = cpulib::pop(cs);
    cs.setreg_w(&reg, popped_val);
}

pub fn b_jmp(cs: &mut CpuState, immediate: Byte) {
    println!("(op) b_jmp");
    cpulib::jump_b(cs, immediate);
}

pub fn w_jmp(cs: &mut CpuState, immediate: Word) {
    println!("(op) w_jmp");
    cpulib::jump_w(cs, immediate);
}

pub fn jz(cs: &mut CpuState, immediate: Byte) {
    println!("(op) jz");
    if !cs.zero() {
        return;
    }

    cpulib::jump_b(cs, immediate);
}

pub fn call(cs: &mut CpuState, immediate: Word) {
    println!("(op) call");
    cpulib::call(cs, immediate);
}

pub fn b_add(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_add");
    let cur_val = cs.getreg_b(&reg);
    let new_val = cpulib::b_add(cs, cur_val, immediate);
    cs.setreg_b(&reg, new_val);
}

pub fn w_add(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_add");
    let cur_val = cs.getreg_w(&reg);
    let new_val = cpulib::w_add(cs, cur_val, immediate);
    cs.setreg_w(&reg, new_val);
}

pub fn b_cmp_ri(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_cmp_ri");
    let reg_val = cs.getreg_b(&reg);
    cpulib::b_sub(cs, reg_val, immediate);
}

pub fn w_cmp_ri(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_cmp_ri");
    let reg_val = cs.getreg_w(&reg);
    cpulib::w_sub(cs, reg_val, immediate);
}

pub fn b_mov_ir(cs: &mut CpuState, reg: Reg8, immediate: Byte) {
    println!("(op) b_mov_ir");
    cs.setreg_b(&reg, immediate);
}

pub fn w_mov_ir(cs: &mut CpuState, reg: Reg16, immediate: Word) {
    println!("(op) w_mov_ir");
    cs.setreg_w(&reg, immediate);
}

pub fn mov_e(cs: &mut CpuState, effective: ModrmValue, reg: ModrmValue) {
    println!("(op) mov_e");

    // TODO - Accept as method argument; should not call cs.read_* from here
    let src: Byte = cs.read_b();

    match effective {
        ModrmMemoryAddr(x) => cs.setmem(x, src),
        ModrmReg8(x) => cs.setreg_b(&x, src),
        _ => panic!("ModrmNone"),
    }
}

pub fn b_mov_ge(cs: &mut CpuState, src: ModrmValue, dest: ModrmValue) {
    println!("(op) b_mov_ge");
    let dest = dest.unwrap_reg8();

    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmReg8(x) => cs.getreg_b(&x),
        _ => panic!("ModrmNone"),
    };
    cs.setreg_b(dest, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState, src: ModrmValue, dest: ModrmValue) {
    println!("(op) w_mov_ge");
    let dest = dest.unwrap_reg16();

    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x).to_u16().unwrap(),
        ModrmReg16(x) => cs.getreg_w(&x),
        _ => panic!("ModrmNone"),
    };
    cs.setreg_w(dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState, dest: ModrmValue, src: ModrmValue) {
    println!("(op) b_mov_eg");
    let src = src.unwrap_reg8();
    let src_value = cs.getreg_b(src);

    match dest {
        ModrmMemoryAddr(x) => cs.setmem(x, src_value),
        ModrmReg8(x) => cs.setreg_b(&x, src_value),
        _ => panic!("ModrmNone"),
    };
}

pub fn w_mov_eg(cs: &mut CpuState, dest: ModrmValue, src: ModrmValue) {
    println!("(op) w_mov_eg");
    let src = src.unwrap_reg16();
    let src_value = cs.getreg_w(src);

    match dest {
        ModrmMemoryAddr(x) => {
            // I'm pretty sure this doesn't work this way...
            cs.setmem(x, byteutils::high8(src_value));
            cs.setmem(x + 1, byteutils::low8(src_value));
        },
        ModrmReg16(x) => cs.setreg_w(&x, src_value),
        _ => panic!("ModrmNone"),
    };
}

pub fn b_cmp_eg(cs: &mut CpuState, left: ModrmValue, right: ModrmValue) {
    println!("(op) b_cmp_eg");
    let right = right.unwrap_reg8();
    let right_value = cs.getreg_b(right);

    let left_value = match left {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmReg8(x) => cs.getreg_b(&x),
        _ => panic!("ModrmNone"),
    };

    cpulib::b_sub(cs, left_value, right_value);
}
