#![feature(macro_rules)]
#![feature(globs)]
use self::cstate::*;
use self::datatypes::{Byte, Word};
mod byteutils;
mod cstate;
mod datatypes;
mod inst;
mod modrm;


fn execute(memory: &mut CpuState) {
    let opcode: Byte = memory.read_b();

    match opcode {
        0x40 => inst::inc(memory, AX),
        0x41 => inst::inc(memory, CX),
        0x42 => inst::inc(memory, DX),
        0x43 => inst::inc(memory, BX),
        0x47 => inst::inc(memory, DI),

        0x50 => inst::push(memory, AX),
        0x51 => inst::push(memory, CX),
        0x52 => inst::push(memory, DX),
        0x53 => inst::push(memory, BX),
        0x54 => inst::push(memory, SP),
        0x56 => inst::push(memory, SI),
        0x57 => inst::push(memory, DI),

        0x58 => inst::pop(memory, AX),
        0x59 => inst::pop(memory, CX),
        0x5A => inst::pop(memory, DX),
        0x5B => inst::pop(memory, BX),
        0x5C => inst::pop(memory, SP),
        0x5E => inst::pop(memory, SI),
        0x5F => inst::pop(memory, DI),

        0xE9 => inst::w_jmp(memory),
        0xEB => inst::b_jmp(memory),

        0xE8 => inst::call(memory),
        0xC3 => inst::ret(memory),

        0x74 => inst::jz(memory),

        0x88 => inst::b_mov_eg(memory),
        0x89 => inst::w_mov_eg(memory),
        0x8A => inst::b_mov_ge(memory),
        0x8B => inst::w_mov_ge(memory),
        0xC6 => inst::mov_e(memory),

        0xB0 => inst::b_mov_ir(memory, AL),
        0xB1 => inst::b_mov_ir(memory, CL),
        0xB2 => inst::b_mov_ir(memory, DL),
        0xB3 => inst::b_mov_ir(memory, BL),
        0xB4 => inst::b_mov_ir(memory, AH),
        0xB5 => inst::b_mov_ir(memory, CH),
        0xB6 => inst::b_mov_ir(memory, DH),
        0xB7 => inst::b_mov_ir(memory, BH),

        0xB8 => inst::w_mov_ir(memory, AX),
        0xB9 => inst::w_mov_ir(memory, CX),
        0xBA => inst::w_mov_ir(memory, DX),
        0xBB => inst::w_mov_ir(memory, BX),
        0xBC => inst::w_mov_ir(memory, SP),
        0xBE => inst::w_mov_ir(memory, SI),
        0xBF => inst::w_mov_ir(memory, DI),

        0x04 => inst::b_add(memory, AL),
        0x05 => inst::w_add(memory, AX),

        0x3C => inst::b_cmp_ri(memory, AL),
        0x3D => inst::w_cmp_ri(memory, AX),

        0xF4 => {
            memory.dump_state();
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Unrecognized opcode: 0x{:X}", opcode),
    }
}

fn main() {
    let mut memory = CpuState::read_from_file();
    loop {
        execute(&mut memory);
    }
}
