use cstate::{CpuState, Reg8, Reg16};
use operand::Operand;


pub fn read_modrm(cs: &mut CpuState, byte_registers: bool) -> (u8, Operand, Operand) {
    // Read ModR/M byte
    let byte = cs.read();

    // Extract `mod'
    let modbits = byte & 0b11000000;
    let modbits = modbits / 64;

    // Extract `reg'
    let reg = byte & 0b00111000;
    let reg = reg / 8;

    // Extract `r/m'
    let rm = byte & 0b00000111;

    // Build effective and register operands
    let effective = modrm_effective(cs, modbits, rm, byte_registers);
    let register = modrm_register(reg, byte_registers);
    (reg, effective, register)
}

fn modrm_register(reg: u8, byte: bool) -> Operand {
    if byte {
        return match reg {
            0b000 => Operand::Reg8(Reg8::AL),
            0b001 => Operand::Reg8(Reg8::CL),
            0b010 => Operand::Reg8(Reg8::DL),
            0b011 => Operand::Reg8(Reg8::BL),
            _ => Operand::RawWord(65534), // FIXME
        }
    } else {
        return match reg {
            0b000 => Operand::Reg16(Reg16::AX),
            0b001 => Operand::Reg16(Reg16::CX),
            0b010 => Operand::Reg16(Reg16::DX),
            0b011 => Operand::Reg16(Reg16::BX),
            0b100 => Operand::Reg16(Reg16::SP),
            0b101 => Operand::Reg16(Reg16::BP),
            0b110 => Operand::Reg16(Reg16::SI),
            0b111 => Operand::Reg16(Reg16::DI),
            _ => panic!("Invalid ModR/M byte 1"),
        }
    }
}

fn modrm_effective(cs: &mut CpuState, modbits: u8, rm: u8, byte_registers: bool) -> Operand {
    match modbits {
        0b00 => match rm {
            0b000 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BX) + cs.getreg16(&Reg16::SI)
            ),
            0b001 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BX) + cs.getreg16(&Reg16::DI)
            ),
            0b010 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BP) + cs.getreg16(&Reg16::SI)
            ),
            0b011 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BP) + cs.getreg16(&Reg16::DI)
            ),
            0b100 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::SI)
            ),
            0b101 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::DI)
            ),
            0b111 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BX)
            ),
            0b110 => Operand::MemoryAddress(
                cs.read16()
            ),
            _ => panic!("Invalid ModR/M byte"),
        },
        0b11 => modrm_register(rm, byte_registers),
        0b10 => match rm {
            0b111 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::BX) + cs.read16()
            ),
            0b101 => Operand::MemoryAddress(
                cs.getreg16(&Reg16::DI) + cs.read16()
            ),
            _ => panic!("Not Implemented"),
        },
        _ => panic!("Not Implemented"),
    }
}
