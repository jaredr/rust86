use byteutils;
use cstate::{CpuState, Reg8, Reg16};
use cstate::Reg16::{IP, BX, SP, BP, SI, DI};
use datatypes::{Byte, Word};
use modrm;
use modrm::ModrmResult;


/**
 * Push the given value onto the stack.
 */
pub fn push(cs: &mut CpuState, val: Word) {
    let low_b = byteutils::low8(val);
    let high_b = byteutils::high8(val);
    let sp = cs.getreg_w(&SP);
    cs.setmem(sp - 1, low_b);
    cs.setmem(sp - 2, high_b);
    cs.setreg_w(&SP, sp - 2);
}

/**
 * Pop and return the top value from the stack.
 */
pub fn pop(cs: &mut CpuState) -> Word {
    let sp = cs.getreg_w(&SP);
    let low_b = cs.getmem(sp);
    let high_b = cs.getmem(sp + 1);
    cs.setmem(sp, 0x0);
    cs.setmem(sp + 1, 0x0);
    cs.setreg_w(&SP, sp + 2);
    byteutils::join8(low_b, high_b)
}

/**
 * Move `ip` by the given twos-complement byte offset.
 */
pub fn jump_b(cs: &mut CpuState, offset: Byte) {
    let ip = cs.getreg_w(&IP);
    let offset = offset.to_u16().unwrap();
    if offset < 127 {
        cs.setreg_w(&IP, ip + offset);
    } else {
        cs.setreg_w(&IP, ip - (256 - offset));
    }
}

/**
 * Move `ip` by the given unsigned word offset.
 */
pub fn jump_w(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg_w(&IP);
    cs.setreg_w(&IP, ip + offset);
}

/**
 * Push `ip`, then jmp `offset`
 */
pub fn call(cs: &mut CpuState, offset: Word) {
    let ip = cs.getreg_w(&IP);
    push(cs, ip);
    jump_w(cs, offset);
}

/**
 * Pop the top value from the stack into `ip`.
 */
pub fn ret(cs: &mut CpuState) {
    let ip = pop(cs);
    cs.setreg_w(&IP, ip);
}

/**
 * Wrapper around byteutils::b_add that sets resulting flags on the CpuState.
 */
pub fn b_add(cs: &mut CpuState, left: Byte, right: Byte) -> Byte {
    let (result, cf, of, sf, zf) = byteutils::b_add(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn w_add(cs: &mut CpuState, left: Word, right: Word) -> Word {
    let (result, cf, of, sf, zf) = byteutils::w_add(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn b_sub(cs: &mut CpuState, left: Byte, right: Byte) -> Byte {
    let (result, cf, of, sf, zf) = byteutils::b_sub(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}

pub fn w_sub(cs: &mut CpuState, left: Word, right: Word) -> Word {
    let (result, cf, of, sf, zf) = byteutils::w_sub(left, right);
    cs.set_flags(cf, of, sf, zf);
    result
}


/**
 * Helper functions to get or set CpuState properties based on ModR/M bytes
 */
pub fn modrm_addr(cs: &mut CpuState, result_addr: modrm::MemoryAddr) -> Word {
    match result_addr {
        modrm::MemoryAddr::BX_SI => cs.getreg_w(&BX) + cs.getreg_w(&SI),
        modrm::MemoryAddr::BX_DI => cs.getreg_w(&BX) + cs.getreg_w(&DI),
        modrm::MemoryAddr::BP_SI => cs.getreg_w(&BP) + cs.getreg_w(&SI),
        modrm::MemoryAddr::BP_DI => cs.getreg_w(&BP) + cs.getreg_w(&DI),
        modrm::MemoryAddr::SI => cs.getreg_w(&SI),
        modrm::MemoryAddr::DI => cs.getreg_w(&DI),
        modrm::MemoryAddr::BX => cs.getreg_w(&BX),
        modrm::MemoryAddr::DISP16 => cs.read_w(),
    }
}

pub fn modrm_reg16(result_reg: &modrm::Register) -> Reg16 {
    match *result_reg {
        modrm::Register::AX => Reg16::AX,
        modrm::Register::CX => Reg16::CX,
        modrm::Register::DX => Reg16::DX,
        modrm::Register::BX => Reg16::BX,
        modrm::Register::SP => Reg16::SP,
        modrm::Register::BP => Reg16::BP,
        modrm::Register::SI => Reg16::SI,
        modrm::Register::DI => Reg16::DI,
    }
}

pub fn modrm_reg8(result_reg: &modrm::Register) -> Reg8 {
    match *result_reg {
        modrm::Register::AX => Reg8::AL,
        modrm::Register::CX => Reg8::CL,
        modrm::Register::DX => Reg8::DL,
        modrm::Register::BX => Reg8::BL,
        _ => panic!("Invalid ModRM.reg for modrm_reg8"),
    }
}

pub fn modrm_value_b(cs: &mut CpuState, effective: ModrmResult) -> Byte {
    match effective {
        ModrmResult::MemoryAddr(x) => {
            let addr = modrm_addr(cs, x);
            cs.getmem(addr)
        },
        ModrmResult::Register(x) => {
            let reg = modrm_reg8(&x);
            cs.getreg_b(&reg)
        },
    }
}

pub fn modrm_value_w(cs: &mut CpuState, effective: ModrmResult) -> Word {
    match effective {
        ModrmResult::MemoryAddr(x) => {
            let addr = modrm_addr(cs, x);
            cs.getmem(addr).to_u16().unwrap()
        },
        ModrmResult::Register(x) => {
            let reg = modrm_reg16(&x);
            cs.getreg_w(&reg)
        },
    }
}
