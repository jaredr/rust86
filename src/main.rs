#![feature(macro_rules)]
#![feature(globs)]
use std::os;
use self::cstate::*;
use self::cstate::Register::*;
use self::datatypes::{Byte, Word};
mod byteutils;
mod cstate;
mod datatypes;
mod inst;
mod modrm;


fn execute(cs: &mut CpuState) {
    let opcode: Byte = cs.read_b();

    match opcode {
        0x40 => inst::inc(cs, AX),
        0x41 => inst::inc(cs, CX),
        0x42 => inst::inc(cs, DX),
        0x43 => inst::inc(cs, BX),
        0x47 => inst::inc(cs, DI),

        0x50 => inst::push(cs, AX),
        0x51 => inst::push(cs, CX),
        0x52 => inst::push(cs, DX),
        0x53 => inst::push(cs, BX),
        0x54 => inst::push(cs, SP),
        0x56 => inst::push(cs, SI),
        0x57 => inst::push(cs, DI),

        0x58 => inst::pop(cs, AX),
        0x59 => inst::pop(cs, CX),
        0x5A => inst::pop(cs, DX),
        0x5B => inst::pop(cs, BX),
        0x5C => inst::pop(cs, SP),
        0x5E => inst::pop(cs, SI),
        0x5F => inst::pop(cs, DI),

        0xE9 => inst::w_jmp(cs),
        0xEB => inst::b_jmp(cs),

        0xE8 => inst::call(cs),
        0xC3 => inst::ret(cs),

        0x74 => inst::jz(cs),

        0x88 => inst::b_mov_eg(cs),
        0x89 => inst::w_mov_eg(cs),
        0x8A => inst::b_mov_ge(cs),
        0x8B => inst::w_mov_ge(cs),
        0xC6 => inst::mov_e(cs),

        0xB0 => inst::b_mov_ir(cs, AL),
        0xB1 => inst::b_mov_ir(cs, CL),
        0xB2 => inst::b_mov_ir(cs, DL),
        0xB3 => inst::b_mov_ir(cs, BL),
        0xB4 => inst::b_mov_ir(cs, AH),
        0xB5 => inst::b_mov_ir(cs, CH),
        0xB6 => inst::b_mov_ir(cs, DH),
        0xB7 => inst::b_mov_ir(cs, BH),

        0xB8 => inst::w_mov_ir(cs, AX),
        0xB9 => inst::w_mov_ir(cs, CX),
        0xBA => inst::w_mov_ir(cs, DX),
        0xBB => inst::w_mov_ir(cs, BX),
        0xBC => inst::w_mov_ir(cs, SP),
        0xBE => inst::w_mov_ir(cs, SI),
        0xBF => inst::w_mov_ir(cs, DI),

        0x04 => inst::b_add(cs, AL),
        0x05 => inst::w_add(cs, AX),

        0x38 => inst::b_cmp_eg(cs),
        0x3C => inst::b_cmp_ri(cs, AL),
        0x3D => inst::w_cmp_ri(cs, AX),

        0xF4 => {
            cs.dump_state();
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Unrecognized opcode: 0x{:X}", opcode),
    }
}

fn main() {
    let argv = os::args();
    if argv.len() < 2 {
        println!("Usage: {} <filename>", argv[0]);
        return;
    }
    let path = Path::new(&argv[1]);

    let mut cs = CpuState::new();
    cs.load_program(&path);

    loop {
        execute(&mut cs);
    }
}
