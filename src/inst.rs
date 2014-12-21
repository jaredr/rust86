use cstate::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::*;
use modrm::ModrmValue::*;


pub fn inc(cs: &mut CpuState, reg: Reg16) {
    println!("(op) inc");
    let cur_val = cs.getreg_w(&reg);
    let new_val = cs.w_add(cur_val, 1);
    cs.setreg_w(&reg, new_val);
}

pub fn push(cs: &mut CpuState, reg: Reg16) {
    println!("(op) push");
    let cur_val = cs.getreg_w(&reg);
    cs.push(cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Reg16) {
    println!("(op) pop");
    let popped_val = cs.pop();
    cs.setreg_w(&reg, popped_val);
}

pub fn b_add(cs: &mut CpuState, reg: Reg8) {
    println!("(op) b_add");
    let byte = cs.read_b();
    let cur_val = cs.getreg_b(&reg);
    let new_val = cs.b_add(cur_val, byte);
    cs.setreg_b(&reg, new_val);
}

pub fn w_add(cs: &mut CpuState, reg: Reg16) {
    println!("(op) w_add");
    let word = cs.read_w();
    let cur_val = cs.getreg_w(&reg);
    let new_val = cs.w_add(cur_val, word);
    cs.setreg_w(&reg, new_val);
}

pub fn b_cmp_ri(cs: &mut CpuState, reg: Reg8) {
    println!("(op) b_cmp_ri");
    let reg_val = cs.getreg_b(&reg);
    let byte = cs.read_b();
    cs.b_sub(reg_val, byte);
}

pub fn w_cmp_ri(cs: &mut CpuState, reg: Reg16) {
    println!("(op) w_cmp_ri");
    let reg_val = cs.getreg_w(&reg);
    let word = cs.read_w();
    cs.w_sub(reg_val, word);
}

pub fn b_mov_ir(cs: &mut CpuState, reg: Reg8) {
    println!("(op) b_mov_r");
    let byte = cs.read_b();
    cs.setreg_b(&reg, byte);
}

pub fn w_mov_ir(cs: &mut CpuState, reg: Reg16) {
    println!("(op) w_mov_r");
    let word = cs.read_w();
    cs.setreg_w(&reg, word);
}

pub fn mov_e(cs: &mut CpuState) {
    println!("(op) mov_e");
    let (dest, _) = get_modrm(cs, true);

    let src: Byte = cs.read_b();
    match dest {
        ModrmMemoryAddr(x) => cs.setmem(x, src),
        ModrmReg8(x) => cs.setreg_b(&x, src),
        _ => panic!("ModrmNone"),
    }
}

pub fn b_mov_ge(cs: &mut CpuState) {
    println!("(op) b_mov_ge");

    let (src, dest) = get_modrm(cs, true);
    let dest = dest.unwrap_reg8();

    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmReg8(x) => cs.getreg_b(&x),
        _ => panic!("ModrmNone"),
    };
    cs.setreg_b(dest, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState) {
    println!("(op) w_mov_ge");

    let (src, dest) = get_modrm(cs, false);
    let dest = dest.unwrap_reg16();

    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x).to_u16().unwrap(),
        ModrmReg16(x) => cs.getreg_w(&x),
        _ => panic!("ModrmNone"),
    };
    cs.setreg_w(dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState) {
    println!("(op) b_mov_eg");
    let (dest, src) = get_modrm(cs, true);

    let src = src.unwrap_reg8();
    let src_value = cs.getreg_b(src);

    match dest {
        ModrmMemoryAddr(x) => cs.setmem(x, src_value),
        ModrmReg8(x) => cs.setreg_b(&x, src_value),
        _ => panic!("ModrmNone"),
    };
}

pub fn w_mov_eg(cs: &mut CpuState) {
    println!("(op) w_mov_eg");
    let (dest, src) = get_modrm(cs, false);
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

pub fn b_cmp_eg(cs: &mut CpuState) {
    println!("(op) b_cmp_eg");
    let (left, right) = get_modrm(cs, true);

    let right = right.unwrap_reg8();
    let right_value = cs.getreg_b(right);

    let left_value = match left {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmReg8(x) => cs.getreg_b(&x),
        _ => panic!("ModrmNone"),
    };

    cs.b_sub(left_value, right_value);
}

pub fn b_jmp(cs: &mut CpuState) {
    println!("(op) b_jmp");
    let offset: Byte = cs.read_b();
    cs.jump_b(offset);
}

pub fn w_jmp(cs: &mut CpuState) {
    println!("(op) w_jmp");
    let offset: Word = cs.read_w();
    cs.jump_w(offset);
}

pub fn jz(cs: &mut CpuState) {
    println!("(op) jz");
    let offset: Byte = cs.read_b();

    if !cs.zero() {
        return;
    }

    cs.jump_b(offset);
}

pub fn call(cs: &mut CpuState) {
    println!("(op) call");
    let offset = cs.read_w();
    cs.call(offset);
}

pub fn ret(cs: &mut CpuState) {
    println!("(op) ret");
    cs.ret();
}
