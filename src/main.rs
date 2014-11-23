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
        0x40 => inst::inc_reg(memory, AX),
        0x41 => inst::inc_reg(memory, CX),
        0x42 => inst::inc_reg(memory, DX),
        0x43 => inst::inc_reg(memory, BX),

        0xE9 => inst::jmp_word(memory),
        0xEB => inst::jmp_byte(memory),

        0x88 => inst::mov_modrm_mreg(memory),
        0x8B => inst::mov_mreg_modrm(memory),
        0xC6 => inst::mov_modrm_byte(memory),

        0xB0 => inst::mov_reg_byte(memory, AL),
        0xB1 => inst::mov_reg_byte(memory, CL),
        0xB2 => inst::mov_reg_byte(memory, DL),
        0xB3 => inst::mov_reg_byte(memory, BL),
        0xB4 => inst::mov_reg_byte(memory, AH),
        0xB5 => inst::mov_reg_byte(memory, CH),
        0xB6 => inst::mov_reg_byte(memory, DH),
        0xB7 => inst::mov_reg_byte(memory, BH),

        0xB8 => inst::mov_reg_word(memory, AX),
        0xB9 => inst::mov_reg_word(memory, CX),
        0xBA => inst::mov_reg_word(memory, DX),
        0xBB => inst::mov_reg_word(memory, BX),
        // SP, BP, SI, DI omitted
        0xBF => inst::mov_reg_word(memory, DI),

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
