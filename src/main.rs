#![feature(globs)]
use self::cstate::*;
mod cstate;


fn inc_reg(memory: &mut CpuState, reg: Register) {
    println!("(op) inc_reg");
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + 1);
}

fn add_reg_word(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_word");
    let word = memory.read_word();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + word);
}

fn add_reg_byte(memory: &mut CpuState, reg: Register) {
    println!("(op) add_reg_byte");
    let byte = memory.read().to_u16().unwrap();
    let cur_val = memory.getreg(reg);
    memory.setreg(reg, cur_val + byte);
}

fn jmp_byte(memory: &mut CpuState) {
    let byte = memory.read();

    // Cast u16 `ip` down to u8 so that `byte` can wrap at 255
    // I'm pretty sure this isn't how a CPU works, but I don't know
    // enough about CPUs to dispute it.
    let mut ip8 = memory.ip.to_u8().unwrap();
    ip8 += byte;
    memory.ip = ip8.to_u16().unwrap();
    println!("(op) jmp_byte: 0x{:X}", byte);
}

fn jmp_word(memory: &mut CpuState) {
    let word = memory.read_word();
    memory.ip += word;

    println!("(op) jmp_word: 0x{:X}", word);
}

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
        0x40 => inc_reg(memory, AX),
        0x41 => inc_reg(memory, CX),
        0x42 => inc_reg(memory, BX),
        0x43 => inc_reg(memory, DX),

        0xE9 => jmp_word(memory),
        0xEB => jmp_byte(memory),

        0x88 => debug_modrm(memory, false),
        0x89 => debug_modrm(memory, true),

        0x04 => add_reg_byte(memory, AL),
        0x05 => add_reg_word(memory, AX),

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
