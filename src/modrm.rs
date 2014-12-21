use self::ModrmValue::*;
use cstate::*;
use cstate::Reg16::{BX, BP, SI, DI};
use datatypes::{Byte,Word};


pub enum ModrmValue {
    ModrmReg16(Reg16),
    ModrmReg8(Reg8),
    ModrmMemoryAddr(Word),
    ModrmNone,
}

impl ModrmValue {
    pub fn unwrap_reg16(&self) -> &Reg16 {
        match *self {
            ModrmReg16(ref x) => x,
            _ => panic!("unwrap_reg16()"),
        }
    }

    pub fn unwrap_reg8(&self) -> &Reg8 {
        match *self {
            ModrmReg8(ref x) => x,
            _ => panic!("unwrap_reg16()"),
        }
    }
}


fn get_modrm_reg16(b_reg: u16) -> Reg16 {
    match b_reg {
        0b000 => Reg16::AX,
        0b001 => Reg16::CX,
        0b010 => Reg16::DX,
        0b011 => Reg16::BX,
        0b100 => Reg16::SP,
        0b101 => Reg16::BP,
        0b110 => Reg16::SI,
        0b111 => Reg16::DI,
        _ => panic!("Invalid ModRM.reg"),
    }
}

fn get_modrm_reg8(b_reg: u16) -> Reg8 {
    match b_reg {
        0b000 => Reg8::AL,
        0b001 => Reg8::CL,
        0b010 => Reg8::DL,
        0b011 => Reg8::BL,
        _ => panic!("Invalid ModRM.reg"),
    }
}

fn get_modrm_reg(b_reg: u16, bytes: bool) -> ModrmValue {
    if bytes {
        return ModrmReg8(get_modrm_reg8(b_reg));
    } else {
        return ModrmReg16(get_modrm_reg16(b_reg));
    }
}

pub fn get_modrm(cs: &mut CpuState, bytes: bool) -> (ModrmValue, ModrmValue) {
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
            0b000 => ModrmMemoryAddr(cs.getreg_w(&BX) + cs.getreg_w(&SI)), // [bx+si]
            0b001 => ModrmMemoryAddr(cs.getreg_w(&BX) + cs.getreg_w(&DI)), // [bx+di]
            0b010 => ModrmMemoryAddr(cs.getreg_w(&BP) + cs.getreg_w(&SI)), // [bp+si]
            0b011 => ModrmMemoryAddr(cs.getreg_w(&BP) + cs.getreg_w(&DI)), // [bp+di]
            0b100 => ModrmMemoryAddr(cs.getreg_w(&SI)), // [si]
            0b101 => ModrmMemoryAddr(cs.getreg_w(&DI)), // [di]
            0b110 => ModrmMemoryAddr(cs.read_w()), // [disp16]
            0b111 => ModrmMemoryAddr(cs.getreg_w(&BX)), // [bx]
            _ => panic!("Invalid ModRM.rm"),
        },
        0b11 => get_modrm_reg(rm, bytes),
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
