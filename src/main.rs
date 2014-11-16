#![feature(globs)]
use self::cstate::*;
mod cstate;
mod inst;

fn debug_modrm(memory: &mut CpuState, w: bool) {
    let (b_mod, b_reg, b_rm) = read_modrm(memory);
    println!(
        "(dbg) .mod=0b{:0>2t}, .reg=0b{:0>3t}, .rm=0b{:0>3t}",
        b_mod,
        b_reg,
        b_rm,
    );

    // http://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-2a-manual.html
    // Table 2-1
    let effective_addr = match b_mod {
        0b00 => match b_rm {
            0b000 => "[bx+si]",
            0b001 => "[bx+di]",
            0b010 => "[bp+si]",
            0b011 => "[bp+di]",
            0b100 => "[si]",
            0b101 => "[di]",
            0b110 => "Not Implemented",
            0b111 => "[bx]",
            _ => panic!("Invalid ModRM.rm"),
        },
        0b11 => match b_rm {
            0b000 => "ax/al",
            0b001 => "cx/cl",
            0b010 => "dx/dl",
            0b011 => "bx/bl",
            0b100 => "sp",
            0b101 => "bp",
            0b110 => "si",
            0b111 => "di",
            _ => panic!("Invalid ModRM.rm"),
        },
        0b01 => panic!("Not Implemented"),
        0b10 => panic!("Not Implemented"),
        _ => panic!("Invalid ModRM.mod"),
    };
    let register = match b_reg {
        0b000 => "ax/al",
        0b001 => "cx/cl",
        0b010 => "dx/dl",
        0b011 => "bx/bl",
        0b100 => "ah/sp",
        0b101 => "ch",
        0b110 => "dh/si",
        0b111 => "bh/di",
        _ => panic!("Invalid ModRM.reg"),
    };
    println!("(dbg) Effective Address: {}", effective_addr);
    println!("(dbg) Effective Register: {}", register);

    if w {
        let word = memory.read_word();
        println!("Word argument: 0x{:X}", word);
    } else {
        let byte = memory.read();
        println!("Byte argument: 0x{:X}", byte);
    }
}

fn read_modrm(memory: &mut CpuState) -> (u8, u8, u8) {
    let byte = memory.read();

    // Extract `mod'
    let b_mod: u8 = byte & 0b11000000;
    let b_mod = b_mod / 64;

    // Extract `reg'
    let b_reg = byte & 0b00111000;
    let b_reg = b_reg / 8;

    // Extract `r/m'
    let b_rm = byte & 0b00000111;

    return (b_mod, b_reg, b_rm);
}

fn execute(memory: &mut CpuState) {
    let byte = memory.read();

    if byte == 0xFF {
        memory.dump_state();
        panic!("We're done here.");
    }

    match byte {
        0x40 => inst::inc_reg(memory, AX),
        0x41 => inst::inc_reg(memory, CX),
        0x42 => inst::inc_reg(memory, BX),
        0x43 => inst::inc_reg(memory, DX),

        0xE9 => inst::jmp_word(memory),
        0xEB => inst::jmp_byte(memory),

        0xC6 => debug_modrm(memory, false),

        0xBB => inst::mov_reg_word(memory, BX),

        0x04 => inst::add_reg_byte(memory, AL),
        0x05 => inst::add_reg_word(memory, AX),

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
