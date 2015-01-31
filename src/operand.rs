use std::num::ToPrimitive;
use cstate;
use byteutils;
use datatypes::{Byte, Word};


pub enum Operand {
    RawByte(Byte),
    RawWord(Word),
    Reg8(cstate::Reg8),
    Reg16(cstate::Reg16),
    MemoryAddress(Word),
}


pub struct Flags {
    pub carry: bool,
    pub overflow: bool,
    pub sign: bool,
    pub zero: bool,
}


pub fn b_operand_value(cs: &mut cstate::CpuState, o: &Operand) -> Byte {
    return match *o {
        Operand::RawByte(ref v) => *v,
        Operand::RawWord(_) => panic!("invalid"),
        Operand::Reg8(ref reg) => cs.getreg8(reg),
        Operand::Reg16(_) => panic!("invalid"),
        Operand::MemoryAddress(ref addr) => cs.getmem(*addr),
    }
}

pub fn w_operand_value(cs: &mut cstate::CpuState, o: &Operand) -> Word {
    return match *o {
        Operand::RawByte(_) => panic!("invalid"),
        Operand::RawWord(ref v) => *v,
        Operand::Reg8(_) => panic!("invalid!"),
        Operand::Reg16(ref reg) => cs.getreg16(reg),
        Operand::MemoryAddress(ref addr) => {
            byteutils::join8(cs.getmem(*addr + 1), cs.getmem(*addr))
        }
    }
}

pub fn b_operand_set(cs: &mut cstate::CpuState, o: &Operand, result: Byte) {
    match *o {
        Operand::RawByte(_) => panic!("invalid"),
        Operand::RawWord(_) => panic!("invalid"),
        Operand::Reg8(ref reg) => cs.setreg8(reg, result),
        Operand::Reg16(ref reg) => cs.setreg16(reg, result.to_u16().unwrap()),
        Operand::MemoryAddress(ref addr) => cs.setmem(*addr, result),
    }
}

pub fn w_operand_set(cs: &mut cstate::CpuState, o: &Operand, result: Word) {
    match *o {
        Operand::RawByte(_) => panic!("invalid"),
        Operand::RawWord(_) => panic!("invalid"),
        Operand::Reg8(_) => panic!("invalid"),
        Operand::Reg16(ref reg) => cs.setreg16(reg, result),
        Operand::MemoryAddress(ref addr) => {
            cs.setmem(*addr, byteutils::high8(result));
            cs.setmem(*addr + 1, byteutils::low8(result));
        }
    }
}
