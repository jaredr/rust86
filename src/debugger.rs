use cstate::{CpuState, Reg16, Reg8};
use datatypes::{Byte, Word};


pub fn dump_state(cs: &CpuState) {
    dump_gr(cs, "ax", Reg16::AX, Reg8::AL, Reg8::AH);
    dump_gr(cs, "bx", Reg16::BX, Reg8::BL, Reg8::BH);
    dump_gr(cs, "cx", Reg16::CX, Reg8::CL, Reg8::CH);
    dump_gr(cs, "dx", Reg16::DX, Reg8::DL, Reg8::DH);
    dump_mem(cs, 0x8000);
    dump_mem(cs, 0x8010);
    dump_mem(cs, 0x8020);
    dump_mem(cs, 0x8030);
    dump_mem(cs, 0x8040);
    dump_mem(cs, 0x8050);
}

fn dump_gr(cs: &CpuState, name: &str, x: Reg16, l: Reg8, h: Reg8) {
    println!(
        "{}     0x{: <5X} (0x{:X} 0x{:X})",
        name,
        cs.getreg_w(&x),
        cs.getreg_b(&l),
        cs.getreg_b(&h),
    );
}

fn dump_mem(cs: &CpuState, start: Word) {
    let mut s_hex = String::new();
    let mut s_chr = String::new();
    for i in range(0, 16) {
        let val: Byte = cs.getmem(start+i);
        s_hex.push_str(format!("{:0>2X} ", val).as_slice());
        s_chr.push_str(format!("{:}", val as char).as_slice());
    }
    println!("mem    0x{:0>5X} {} {}", start, s_hex, s_chr);
}

pub fn dump_vram(cs: &CpuState) {
    let start = 0x8000;
    let rowcnt = 25;
    let rowlen = 80;

    for row in range(0, rowcnt) {
        for col in range(0, rowlen) {
            let offset = (row * rowlen) + col;
            let val = cs.getmem(start + offset);
            let val = match val {
                0 => ' ',
                _ => val as char,
            };
            print!("{}", val);
        }

        println!("");
    }
}
