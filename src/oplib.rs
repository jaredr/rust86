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
    cs.setmem(sp - 2, high_b);
    cs.setmem(sp - 1, low_b);
    cs.setreg_w(&SP, sp - 2);
}

/**
 * Pop and return the top value from the stack.
 */
pub fn pop(cs: &mut CpuState) -> Word {
    let sp = cs.getreg_w(&SP);
    let low_b = cs.getmem(sp + 1);
    let high_b = cs.getmem(sp);
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
 * Wrapper functions for byteutils::arithmetic! that set the resulting
 * flags on the provided CpuState.
 */
macro_rules! arithmetic (
    (
        $name:ident,
        $size:ident,
        $arithmetic_fn:expr
    ) => {
        pub fn $name(cs: &mut CpuState, left: $size, right: $size) -> $size {
            let (result, cf, of, sf, zf) = $arithmetic_fn(left, right);
            cs.set_flags(cf, of, sf, zf);
            result
        }
    }
);

arithmetic!(b_add, Byte, byteutils::b_add);
arithmetic!(w_add, Word, byteutils::w_add);
arithmetic!(b_sub, Byte, byteutils::b_sub);
arithmetic!(w_sub, Word, byteutils::w_sub);
arithmetic!(b_or, Byte, byteutils::b_or);
arithmetic!(w_or, Word, byteutils::w_or);
arithmetic!(b_xor, Byte, byteutils::b_xor);
arithmetic!(w_xor, Word, byteutils::w_xor);
arithmetic!(b_and, Byte, byteutils::b_and);
arithmetic!(w_and, Word, byteutils::w_and);


/**
 * Returns the current value of the memory address specified by this MemoryAddr
 */
pub fn modrm_addr(cs: &mut CpuState, result_addr: &modrm::MemoryAddr) -> Word {
    match *result_addr {
        modrm::MemoryAddr::BX_SI => cs.getreg_w(&BX) + cs.getreg_w(&SI),
        modrm::MemoryAddr::BX_DI => cs.getreg_w(&BX) + cs.getreg_w(&DI),
        modrm::MemoryAddr::BP_SI => cs.getreg_w(&BP) + cs.getreg_w(&SI),
        modrm::MemoryAddr::BP_DI => cs.getreg_w(&BP) + cs.getreg_w(&DI),
        modrm::MemoryAddr::SI => cs.getreg_w(&SI),
        modrm::MemoryAddr::DI => cs.getreg_w(&DI),
        modrm::MemoryAddr::BX => cs.getreg_w(&BX),
    }
}

/**
 * Returns the Reg16 specified by this ModR/M byte
 */
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

/**
 * Returns the Reg8 specified by this ModR/M byte
 */
pub fn modrm_reg8(result_reg: &modrm::Register) -> Reg8 {
    match *result_reg {
        modrm::Register::AX => Reg8::AL,
        modrm::Register::CX => Reg8::CL,
        modrm::Register::DX => Reg8::DL,
        modrm::Register::BX => Reg8::BL,
        _ => panic!("Invalid ModRM.reg for modrm_reg8"),
    }
}

/**
 * Resolves and returns the current 8-bit value specified by
 * this ModrmResult, whether it may be a memory address or register.
 */
pub fn modrm_value_b(cs: &mut CpuState, effective: &ModrmResult) -> Byte {
    match *effective {
        ModrmResult::MemoryAddr(ref x) => {
            let addr = modrm_addr(cs, x);
            cs.getmem(addr)
        },
        ModrmResult::MemoryDisp16(ref addr) => {
            cs.getmem(*addr)
        },
        ModrmResult::Register(ref x) => {
            let reg = modrm_reg8(x);
            cs.getreg_b(&reg)
        },
    }
}

/**
 * Resolves and returns the current 16-bit value specified by
 * this ModrmResult, whether it may be a memory address or register.
 */
pub fn modrm_value_w(cs: &mut CpuState, effective: &ModrmResult) -> Word {
    match *effective {
        ModrmResult::MemoryAddr(ref x) => {
            let addr = modrm_addr(cs, x);
            byteutils::join8(cs.getmem(addr + 1), cs.getmem(addr))
        },
        ModrmResult::MemoryDisp16(ref addr) => {
            byteutils::join8(cs.getmem(*addr + 1), cs.getmem(*addr))
        },
        ModrmResult::Register(ref x) => {
            let reg = modrm_reg16(x);
            cs.getreg_w(&reg)
        },
    }
}

/**
 * TODO - Document
 */
pub fn modrm_set_b(cs: &mut CpuState, result: &ModrmResult, value: Byte) {
    match *result {
        ModrmResult::MemoryAddr(ref x) => {
            let addr = modrm_addr(cs, x);
            cs.setmem(addr, value);
        },
        ModrmResult::MemoryDisp16(ref addr) => {
            cs.setmem(*addr, value);
        },
        ModrmResult::Register(ref x) => {
            let reg = modrm_reg8(x);
            cs.setreg_b(&reg, value);
        },
    }
}

/**
 * TODO - Document
 */
pub fn modrm_set_w(cs: &mut CpuState, result: &ModrmResult, value: Word) {
    match *result {
        ModrmResult::MemoryAddr(ref x) => {
            let addr = modrm_addr(cs, x);
            cs.setmem(addr, byteutils::high8(value));
            cs.setmem(addr + 1, byteutils::low8(value));
        },
        ModrmResult::MemoryDisp16(ref addr) => {
            cs.setmem(*addr, byteutils::high8(value));
            cs.setmem(*addr + 1, byteutils::low8(value));
        },
        ModrmResult::Register(ref x) => {
            let reg = modrm_reg16(x);
            cs.setreg_w(&reg, value);
        },
    }
}
