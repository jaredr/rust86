use self::ModrmValue::*;
use cstate::*;
use cstate::Register::*;
use datatypes::{Byte,Word};


pub enum ModrmValue {
    ModrmRegister(Register),
    ModrmMemoryAddr(Word),
    ModrmNone,
}


fn get_modrm_reg(b_reg: u16, bytes: bool) -> Register {
    match b_reg {
        0b000 => if bytes { AL } else { AX }, // ax/al
        0b001 => if bytes { CL } else { CX }, // cx/cl
        0b010 => if bytes { DL } else { DX }, // dx/dl
        0b011 => if bytes { BL } else { BX }, // bx/bl
        0b100 => SP, // sp
        0b101 => BP, // bp
        0b110 => SI, // si
        0b111 => DI, // di
        _ => panic!("Invalid ModRM.reg"),
    }
}

pub fn get_modrm(cs: &mut CpuState, bytes: bool) -> (ModrmValue, Register) {
    let (modbits, reg, rm) = read_modrm(cs);
    println!(
        "(dbg) get_modrm .mod=0b{:0>2b}, .reg=0b{:0>3b}, .rm=0b{:0>3b}",
        modbits,
        reg,
        rm,
    );

    // http://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-2a-manual.html
    // Table 2-1
    let effective: ModrmValue = match modbits {
        0b00 => match rm {
            0b000 => ModrmMemoryAddr(cs.getreg(&BX) + cs.getreg(&SI)), // [bx+si]
            0b001 => ModrmMemoryAddr(cs.getreg(&BX) + cs.getreg(&DI)), // [bx+di]
            0b010 => ModrmMemoryAddr(cs.getreg(&BP) + cs.getreg(&SI)), // [bp+si]
            0b011 => ModrmMemoryAddr(cs.getreg(&BP) + cs.getreg(&DI)), // [bp+di]
            0b100 => ModrmMemoryAddr(cs.getreg(&SI)), // [si]
            0b101 => ModrmMemoryAddr(cs.getreg(&DI)), // [di]
            0b110 => ModrmMemoryAddr(cs.read_w()), // [disp16]
            0b111 => ModrmMemoryAddr(cs.getreg(&BX)), // [bx]
            _ => panic!("Invalid ModRM.rm"),
        },
        0b11 => ModrmRegister(get_modrm_reg(rm, bytes)),
        0b01 => panic!("Not Implemented"),
        0b10 => panic!("Not Implemented"),
        _ => panic!("Invalid ModRM.mod"),
    };

    let register = get_modrm_reg(reg, bytes);

    (effective, register)
}

pub fn read_modrm(cs: &mut CpuState) -> (Byte, Byte, Byte) {
    let byte: Byte = cs.read_b();

    // Extract `mod'
    let modbits = byte & 0b11000000;
    let modbits = modbits / 64;

    // Extract `reg'
    let reg = byte & 0b00111000;
    let reg = reg / 8;

    // Extract `r/m'
    let rm = byte & 0b00000111;

    return (modbits, reg, rm);
}
