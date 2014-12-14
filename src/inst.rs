use cstate::*;
use cstate::Register::*;
use byteutils;
use datatypes::{Byte, Word};
use modrm::*;
use modrm::ModrmValue::*;


pub fn inc(memory: &mut CpuState, reg: Register) {
    println!("(op) inc");
    let cur_val = memory.getreg(&reg);
    let new_val = memory.w_add(cur_val, 1);
    memory.setreg(&reg, new_val);
}

pub fn push(memory: &mut CpuState, reg: Register) {
    println!("(op) push");
    let cur_val = memory.getreg(&reg);
    memory.push(cur_val);
}

pub fn pop(memory: &mut CpuState, reg: Register) {
    println!("(op) pop");
    let popped_val = memory.pop();
    memory.setreg(&reg, popped_val);
}

pub fn b_add(memory: &mut CpuState, reg: Register) {
    println!("(op) b_add");
    let byte = memory.read_b();
    let cur_val = memory.getreg(&reg);
    let new_val = memory.b_add(cur_val, byte);
    memory.setreg(&reg, new_val);
}

pub fn w_add(memory: &mut CpuState, reg: Register) {
    println!("(op) w_add");
    let word = memory.read_w();
    let cur_val = memory.getreg(&reg);
    let new_val = memory.w_add(cur_val, word);
    memory.setreg(&reg, new_val);
}

pub fn b_cmp_ri(memory: &mut CpuState, reg: Register) {
    println!("(op) b_cmp_ri");
    let reg_val = memory.getreg(&reg);
    let byte = memory.read_b();
    memory.b_sub(reg_val, byte);
}

pub fn w_cmp_ri(memory: &mut CpuState, reg: Register) {
    println!("(op) w_cmp_ri");
    let reg_val = memory.getreg(&reg);
    let word = memory.read_w();
    memory.w_sub(reg_val, word);
}

pub fn b_mov_ir(memory: &mut CpuState, reg: Register) {
    println!("(op) b_mov_r");
    let byte = memory.read_b();
    memory.setreg(&reg, byte);
}

pub fn w_mov_ir(memory: &mut CpuState, reg: Register) {
    println!("(op) w_mov_r");
    let word = memory.read_w();
    memory.setreg(&reg, word);
}

pub fn mov_e(memory: &mut CpuState) {
    println!("(op) mov_e");
    let (dest, _) = get_modrm(memory, true);

    let src: Byte = memory.read_b();
    match dest {
        ModrmMemoryAddr(x) => memory.setmem(x, src),
        ModrmRegister(x) => memory.setreg(&x, src),
        ModrmNone => panic!("ModrmNone"),
    }
}

pub fn b_mov_ge(memory: &mut CpuState) {
    println!("(op) b_mov_ge");

    let (src, dest) = get_modrm(memory, true);
    let src_value = match src {
        ModrmMemoryAddr(x) => memory.getmem(x),
        ModrmRegister(x) => memory.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };
    memory.setreg(&dest, src_value);
}

pub fn w_mov_ge(memory: &mut CpuState) {
    println!("(op) w_mov_ge");

    let (src, dest) = get_modrm(memory, false);
    let src_value = match src {
        ModrmMemoryAddr(x) => memory.getmem(x),
        ModrmRegister(x) => memory.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };
    memory.setreg(&dest, src_value);
}

pub fn b_mov_eg(memory: &mut CpuState) {
    println!("(op) b_mov_eg");
    let (dest, src) = get_modrm(memory, true);

    let src_value = memory.getreg(&src);
    match dest {
        ModrmMemoryAddr(x) => memory.setmem(x, src_value),
        ModrmRegister(x) => memory.setreg(&x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn w_mov_eg(memory: &mut CpuState) {
    println!("(op) w_mov_eg");
    let (dest, src) = get_modrm(memory, false);

    let src_value = memory.getreg(&src);
    match dest {
        ModrmMemoryAddr(x) => {
            // As with b_jmp, I'm pretty sure this doesn't work this way...
            memory.setmem(x, byteutils::high8(src_value));
            memory.setmem(x + 1, byteutils::low8(src_value));
        },
        ModrmRegister(x) => memory.setreg(&x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn b_cmp_eg(memory: &mut CpuState) {
    println!("(op) b_cmp_eg");
    let (left, right) = get_modrm(memory, true);

    let right_value = memory.getreg(&right);
    let left_value = match left {
        ModrmMemoryAddr(x) => memory.getmem(x),
        ModrmRegister(x) => memory.getreg(&x),
        ModrmNone => panic!("ModrmNone"),
    };

    memory.b_sub(left_value, right_value);
}

pub fn b_jmp(memory: &mut CpuState) {
    println!("(op) b_jmp");
    let dest: Byte = memory.read_b();
    let ip = memory.getreg(&IP);
    let (dest_val, _, _, _, _) = byteutils::b_add(ip, dest);
    memory.setreg(&IP, dest_val);
}

pub fn w_jmp(memory: &mut CpuState) {
    println!("(op) w_jmp");
    let dest: Word = memory.read_w();
    let ip: Word = memory.getreg(&IP);
    let (dest_val, _, _, _, _) = byteutils::w_add(ip, dest);
    memory.setreg(&IP, dest_val);
}

pub fn jz(memory: &mut CpuState) {
    println!("(op) jz");
    let dest: Byte = memory.read_b();

    if memory.zero() {
        let ip = memory.getreg(&IP);
        let (dest_val, _, _, _, _) = byteutils::b_add(ip, dest);
        memory.setreg(&IP, dest_val);
    }
}

pub fn call(memory: &mut CpuState) {
    println!("(op) call");
    let dest = memory.read_w();
    let ip: Word = memory.getreg(&IP);
    memory.push(ip);
    memory.setreg(&IP, ip+dest);
}

pub fn ret(memory: &mut CpuState) {
    println!("(op) ret");
    let ip: Word = memory.pop();
    memory.setreg(&IP, ip);
}
