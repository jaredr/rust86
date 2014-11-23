#![feature(globs)]
use self::cstate::*;
use self::alias::{Byte, Word};
mod alias;
mod cstate;
mod inst;


fn execute(memory: &mut CpuState) {
    let opcode: Byte = memory.read_b();

    match opcode {
        0x40 => inst::inc_reg(memory, AX),
        0x41 => inst::inc_reg(memory, CX),
        0x42 => inst::inc_reg(memory, DX),
        0x43 => inst::inc_reg(memory, BX),

        0xE9 => inst::jmp_word(memory),
        0xEB => inst::jmp_byte(memory),

        0xBB => inst::mov_reg_word(memory, BX),

        0x04 => inst::add_reg_byte(memory, AL),
        0x05 => inst::add_reg_word(memory, AX),

        0xF4 => {
            memory.dump_state();
            panic!("0xF4");
        },
        0x90 => {},

        _ => println!("Unrecognized instruction"),
    }
}

fn main() {
    let mut memory = CpuState::read_from_file();
    loop {
        execute(&mut memory);
    }
}
