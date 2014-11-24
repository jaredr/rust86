use cstate::*;
use datatypes::{Byte, Word};
use modrm::*;

pub fn inc(memory: &mut CpuState, reg: Register) {
    println!("(op) inc");
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + 1);
}

pub fn push(memory: &mut CpuState, reg: Register) {
    println!("(op) push");
    let cur_val = memory.getreg(reg);
    memory.push(cur_val);
}

pub fn pop(memory: &mut CpuState, reg: Register) {
    println!("(op) pop");
    let popped_val = memory.pop();
    memory.setreg(reg, popped_val);
}

pub fn b_add(memory: &mut CpuState, reg: Register) {
    println!("(op) b_add");
    let byte = memory.read_b();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + byte);
}

pub fn w_add(memory: &mut CpuState, reg: Register) {
    println!("(op) w_add");
    let word = memory.read_w();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + word);
}

pub fn b_mov_r(memory: &mut CpuState, reg: Register) {
    println!("(op) b_mov_r");
    let byte = memory.read_b();
    memory.setreg(reg, byte);
}

pub fn w_mov_r(memory: &mut CpuState, reg: Register) {
    println!("(op) w_mov_r");
    let word = memory.read_w();
    memory.setreg(reg, word);
}

pub fn mov_e(memory: &mut CpuState) {
    println!("(op) mov_e");
    let (dest, _) = get_modrm(memory, true);

    let src: Byte = memory.read_b();
    match dest {
        ModrmMemoryAddr(x) => memory.setmem_b(x, src),
        ModrmRegister(x) => memory.setreg(x, src),
        ModrmNone => panic!("ModrmNone"),
    }
}

pub fn b_mov_ge(memory: &mut CpuState) {
    println!("(op) b_mov_ge");

    let (src, dest) = get_modrm(memory, true);
    let src_value = match src {
        ModrmMemoryAddr(x) => memory.getmem_b(x),
        ModrmRegister(x) => memory.getreg(x),
        ModrmNone => panic!("ModrmNone"),
    };
    memory.setreg(dest, src_value);
}

pub fn w_mov_ge(memory: &mut CpuState) {
    println!("(op) w_mov_ge");

    let (src, dest) = get_modrm(memory, false);
    let src_value = match src {
        ModrmMemoryAddr(x) => memory.getmem_b(x),
        ModrmRegister(x) => memory.getreg(x),
        ModrmNone => panic!("ModrmNone"),
    };
    memory.setreg(dest, src_value);
}

pub fn b_mov_eg(memory: &mut CpuState) {
    println!("(op) b_mov_eg");
    let (dest, src) = get_modrm(memory, true);

    let src_value = memory.getreg(src);
    match dest {
        ModrmMemoryAddr(x) => memory.setmem_b(x, src_value),
        ModrmRegister(x) => memory.setreg(x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn w_mov_eg(memory: &mut CpuState) {
    println!("(op) w_mov_eg");
    let (dest, src) = get_modrm(memory, false);

    let src_value = memory.getreg(src);
    match dest {
        ModrmMemoryAddr(x) => {
            // As with b_jmp, I'm pretty sure this doesn't work this way...
            memory.setmem_b(x, CpuState::high8(src_value));
            memory.setmem_b(x + 1, CpuState::low8(src_value));
        },
        ModrmRegister(x) => memory.setreg(x, src_value),
        ModrmNone => panic!("ModrmNone"),
    };
}

pub fn b_jmp(memory: &mut CpuState) {
    println!("(op) b_jmp");
    let dest: Byte = memory.read_b();

    // Cast u16 `ip` down to u8 so that `byte` can wrap at 255
    // I'm pretty sure this isn't how a CPU works, but I don't know
    // enough about CPUs to dispute it.
    let mut ip8 = memory.getreg(IP).to_u8().unwrap();
    let dest8 = dest.to_u8().unwrap();
    ip8 += dest8;
    memory.setreg(IP, ip8.to_u16().unwrap());
}

pub fn w_jmp(memory: &mut CpuState) {
    println!("(op) w_jmp");
    let dest: Word = memory.read_w();
    let ip: Word = memory.getreg(IP);
    memory.setreg(IP, ip+dest);
}

pub fn call(memory: &mut CpuState) {
    println!("(op) call");
    let dest = memory.read_w();
    let ip: Word = memory.getreg(IP);
    memory.push(ip);
    memory.setreg(IP, ip+dest);
}

pub fn ret(memory: &mut CpuState) {
    println!("(op) ret");
    let ip: Word = memory.pop();
    memory.setreg(IP, ip);
}
