use cstate::*;
use cstate::Register::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::*;
use modrm::ModrmValue::*;


pub fn inc(cs: &mut CpuState, reg: Register) {
    println!("(op) inc");
    let cur_val = cs.getreg(&reg);
    let new_val = cs.w_add(cur_val, 1);
    cs.setreg(&reg, new_val);
}

pub fn push(cs: &mut CpuState, reg: Register) {
    println!("(op) push");
    let cur_val = cs.getreg(&reg);
    cs.push(cur_val);
}

pub fn pop(cs: &mut CpuState, reg: Register) {
    println!("(op) pop");
    let popped_val = cs.pop();
    cs.setreg(&reg, popped_val);
}

pub fn b_add(cs: &mut CpuState, reg: Register) {
    println!("(op) b_add");
    let byte = cs.read_b();
    let cur_val = cs.getreg(&reg);
    let new_val = cs.b_add(cur_val, byte);
    cs.setreg(&reg, new_val);
}

pub fn w_add(cs: &mut CpuState, reg: Register) {
    println!("(op) w_add");
    let word = cs.read_w();
    let cur_val = cs.getreg(&reg);
    let new_val = cs.w_add(cur_val, word);
    cs.setreg(&reg, new_val);
}

pub fn b_cmp_ri(cs: &mut CpuState, reg: Register) {
    println!("(op) b_cmp_ri");
    let reg_val = cs.getreg(&reg);
    let byte = cs.read_b();
    cs.b_sub(reg_val, byte);
}

pub fn w_cmp_ri(cs: &mut CpuState, reg: Register) {
    println!("(op) w_cmp_ri");
    let reg_val = cs.getreg(&reg);
    let word = cs.read_w();
    cs.w_sub(reg_val, word);
}

pub fn b_mov_ir(cs: &mut CpuState, reg: Register) {
    println!("(op) b_mov_r");
    let byte = cs.read_b();
    cs.setreg(&reg, byte);
}

pub fn w_mov_ir(cs: &mut CpuState, reg: Register) {
    println!("(op) w_mov_r");
    let word = cs.read_w();
    cs.setreg(&reg, word);
}

pub fn mov_e(cs: &mut CpuState) {
    println!("(op) mov_e");
    let (dest, _) = get_modrm(cs, true);

    let src: Byte = cs.read_b();
    match dest {
        ModrmMemoryAddr(x) => cs.setmem(x, src),
        ModrmRegister(x) => cs.setreg(&x, src),
        ModrmNone => panic!("ModrmNone"),
    }
}

pub fn b_mov_ge(cs: &mut CpuState) {
    println!("(op) b_mov_ge");

    let (src, dest) = get_modrm(cs, true);
    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmRegister(x) => cs.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };
    cs.setreg(&dest, src_value);
}

pub fn w_mov_ge(cs: &mut CpuState) {
    println!("(op) w_mov_ge");

    let (src, dest) = get_modrm(cs, false);
    let src_value = match src {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmRegister(x) => cs.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };
    cs.setreg(&dest, src_value);
}

pub fn b_mov_eg(cs: &mut CpuState) {
    println!("(op) b_mov_eg");
    let (dest, src) = get_modrm(cs, true);

    let src_value = cs.getreg(&src);
    match dest {
        ModrmMemoryAddr(x) => cs.setmem(x, src_value),
        ModrmRegister(x) => cs.setreg(&x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn w_mov_eg(cs: &mut CpuState) {
    println!("(op) w_mov_eg");
    let (dest, src) = get_modrm(cs, false);

    let src_value = cs.getreg(&src);
    match dest {
        ModrmMemoryAddr(x) => {
            // I'm pretty sure this doesn't work this way...
            cs.setmem(x, byteutils::high8(src_value));
            cs.setmem(x + 1, byteutils::low8(src_value));
        },
        ModrmRegister(x) => cs.setreg(&x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn b_cmp_eg(cs: &mut CpuState) {
    println!("(op) b_cmp_eg");
    let (left, right) = get_modrm(cs, true);

    let right_value = cs.getreg(&right);
    let left_value = match left {
        ModrmMemoryAddr(x) => cs.getmem(x),
        ModrmRegister(x) => cs.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };

    cs.b_sub(left_value, right_value);
}

pub fn b_jmp(cs: &mut CpuState) {
    println!("(op) b_jmp");
    let dest: Byte = cs.read_b();
    let ip = cs.getreg(&IP);
    let (dest_val, _, _, _, _) = byteutils::b_add(ip, dest);
    cs.setreg(&IP, dest_val);
}

pub fn w_jmp(cs: &mut CpuState) {
    println!("(op) w_jmp");
    let dest: Word = cs.read_w();
    let ip: Word = cs.getreg(&IP);
    let (dest_val, _, _, _, _) = byteutils::w_add(ip, dest);
    cs.setreg(&IP, dest_val);
}

pub fn jz(cs: &mut CpuState) {
    println!("(op) jz");
    let dest: Byte = cs.read_b();

    if cs.zero() {
        let ip = cs.getreg(&IP);
        let (dest_val, _, _, _, _) = byteutils::b_add(ip, dest);
        cs.setreg(&IP, dest_val);
    }
}

pub fn call(cs: &mut CpuState) {
    println!("(op) call");
    let dest = cs.read_w();
    let ip: Word = cs.getreg(&IP);
    cs.push(ip);
    cs.setreg(&IP, ip+dest);
}

pub fn ret(cs: &mut CpuState) {
    println!("(op) ret");
    let ip: Word = cs.pop();
    cs.setreg(&IP, ip);
}
