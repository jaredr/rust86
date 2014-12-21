#![feature(macro_rules)]
#![feature(globs)]
use std::os;
use self::cstate::*;
use self::datatypes::{Byte, Word};
mod byteutils;
mod cstate;
mod datatypes;
mod debugger;
mod inst;
mod modrm;


fn execute(cs: &mut CpuState) {
    let opcode: Byte = cs.read_b();

    match opcode {
        0x40 => inst::inc(cs, Reg16::AX),
        0x41 => inst::inc(cs, Reg16::CX),
        0x42 => inst::inc(cs, Reg16::DX),
        0x43 => inst::inc(cs, Reg16::BX),
        0x47 => inst::inc(cs, Reg16::DI),

        0x50 => inst::push(cs, Reg16::AX),
        0x51 => inst::push(cs, Reg16::CX),
        0x52 => inst::push(cs, Reg16::DX),
        0x53 => inst::push(cs, Reg16::BX),
        0x54 => inst::push(cs, Reg16::SP),
        0x56 => inst::push(cs, Reg16::SI),
        0x57 => inst::push(cs, Reg16::DI),

        0x58 => inst::pop(cs, Reg16::AX),
        0x59 => inst::pop(cs, Reg16::CX),
        0x5A => inst::pop(cs, Reg16::DX),
        0x5B => inst::pop(cs, Reg16::BX),
        0x5C => inst::pop(cs, Reg16::SP),
        0x5E => inst::pop(cs, Reg16::SI),
        0x5F => inst::pop(cs, Reg16::DI),

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

        0xB0 => inst::b_mov_ir(cs, Reg8::AL),
        0xB1 => inst::b_mov_ir(cs, Reg8::CL),
        0xB2 => inst::b_mov_ir(cs, Reg8::DL),
        0xB3 => inst::b_mov_ir(cs, Reg8::BL),
        0xB4 => inst::b_mov_ir(cs, Reg8::AH),
        0xB5 => inst::b_mov_ir(cs, Reg8::CH),
        0xB6 => inst::b_mov_ir(cs, Reg8::DH),
        0xB7 => inst::b_mov_ir(cs, Reg8::BH),

        0xB8 => inst::w_mov_ir(cs, Reg16::AX),
        0xB9 => inst::w_mov_ir(cs, Reg16::CX),
        0xBA => inst::w_mov_ir(cs, Reg16::DX),
        0xBB => inst::w_mov_ir(cs, Reg16::BX),
        0xBC => inst::w_mov_ir(cs, Reg16::SP),
        0xBE => inst::w_mov_ir(cs, Reg16::SI),
        0xBF => inst::w_mov_ir(cs, Reg16::DI),

        0x04 => inst::b_add(cs, Reg8::AL),
        0x05 => inst::w_add(cs, Reg16::AX),

        0x38 => inst::b_cmp_eg(cs),
        0x3C => inst::b_cmp_ri(cs, Reg8::AL),
        0x3D => inst::w_cmp_ri(cs, Reg16::AX),

        0xF4 => {
            debugger::dump_state(cs);
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
