#![feature(globs)]
use self::cstate::*;
use self::alias::{Byte, Word};
mod alias;
mod cstate;
mod inst;
mod modrm;


fn execute(memory: &mut CpuState) {
    let opcode: Byte = memory.read_b();

    match opcode {
        0x40 => inst::inc(memory, AX),
        0x41 => inst::inc(memory, CX),
        0x42 => inst::inc(memory, DX),
        0x43 => inst::inc(memory, BX),

        0xE9 => inst::w_jmp(memory),
        0xEB => inst::b_jmp(memory),

        0x88 => inst::mov_eg(memory),
        0x8B => inst::mov_ge(memory),
        0xC6 => inst::mov_e(memory),

        0xB0 => inst::b_mov_r(memory, AL),
        0xB1 => inst::b_mov_r(memory, CL),
        0xB2 => inst::b_mov_r(memory, DL),
        0xB3 => inst::b_mov_r(memory, BL),
        0xB4 => inst::b_mov_r(memory, AH),
        0xB5 => inst::b_mov_r(memory, CH),
        0xB6 => inst::b_mov_r(memory, DH),
        0xB7 => inst::b_mov_r(memory, BH),

        0xB8 => inst::w_mov_r(memory, AX),
        0xB9 => inst::w_mov_r(memory, CX),
        0xBA => inst::w_mov_r(memory, DX),
        0xBB => inst::w_mov_r(memory, BX),
        // SP, BP, SI, DI omitted
        0xBF => inst::w_mov_r(memory, DI),

        0x04 => inst::b_add(memory, AL),
        0x05 => inst::w_add(memory, AX),

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
