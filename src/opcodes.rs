use operations;
use debugger;
use cstate::{CpuState, Reg8, Reg16};
use datatypes::Byte;


pub fn do_opcode(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        // Opcodes with immediate byte arguments
        0x04 | 0xB0 | 0xB1 | 0xB2 | 0xB3 | 0xB4 | 0xB5 | 0xB6 | 0xB7 | 0x3C | 0xEB | 0x74 => do_opcode_ib(cs, opcode),

        // Opcodes with immediate word arguments
        0x05 | 0xB8 | 0xB9 | 0xBA | 0xBB | 0xBC | 0xBD | 0xBE | 0xBF | 0x3D | 0xE8 | 0xE9 => do_opcode_iw(cs, opcode),

        // TODO - Opcodes with ModR/M arguments (operate on bytes)
        0x88 => operations::b_mov_eg(cs),
        0x8A => operations::b_mov_ge(cs),
        0x38 => operations::b_cmp_eg(cs),

        // TODO - Opcodes with ModR/M arguments (operate on words)
        0x89 => operations::w_mov_eg(cs),
        0x8B => operations::w_mov_ge(cs),
        0xC6 => operations::mov_e(cs),

        // Opcodes with no arguments
        0x40 => operations::inc(cs, Reg16::AX),
        0x41 => operations::inc(cs, Reg16::CX),
        0x42 => operations::inc(cs, Reg16::DX),
        0x43 => operations::inc(cs, Reg16::BX),
        0x47 => operations::inc(cs, Reg16::DI),

        0x50 => operations::push(cs, Reg16::AX),
        0x51 => operations::push(cs, Reg16::CX),
        0x52 => operations::push(cs, Reg16::DX),
        0x53 => operations::push(cs, Reg16::BX),
        0x54 => operations::push(cs, Reg16::SP),
        0x56 => operations::push(cs, Reg16::SI),
        0x57 => operations::push(cs, Reg16::DI),

        0x58 => operations::pop(cs, Reg16::AX),
        0x59 => operations::pop(cs, Reg16::CX),
        0x5A => operations::pop(cs, Reg16::DX),
        0x5B => operations::pop(cs, Reg16::BX),
        0x5C => operations::pop(cs, Reg16::SP),
        0x5E => operations::pop(cs, Reg16::SI),
        0x5F => operations::pop(cs, Reg16::DI),

        0xC3 => operations::ret(cs),

        // Special opcodes
        0xF4 => {
            debugger::dump_state(cs);
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Unrecognized opcode: 0x{:X}", opcode),
    };
}

/**
 * Handle operations with immediate byte arguments
 */
fn do_opcode_ib(cs: &mut CpuState, opcode: Byte) {
    let immediate = cs.read_b();

    match opcode {
        0x04 => operations::b_add(cs, Reg8::AL, immediate),

        0x74 => operations::jz(cs, immediate),

        0x3C => operations::b_cmp_ri(cs, Reg8::AL, immediate),

        0xB0 => operations::b_mov_ir(cs, Reg8::AL, immediate),
        0xB1 => operations::b_mov_ir(cs, Reg8::CL, immediate),
        0xB2 => operations::b_mov_ir(cs, Reg8::DL, immediate),
        0xB3 => operations::b_mov_ir(cs, Reg8::BL, immediate),
        0xB4 => operations::b_mov_ir(cs, Reg8::AH, immediate),
        0xB5 => operations::b_mov_ir(cs, Reg8::CH, immediate),
        0xB6 => operations::b_mov_ir(cs, Reg8::DH, immediate),
        0xB7 => operations::b_mov_ir(cs, Reg8::BH, immediate),

        0xEB => operations::b_jmp(cs, immediate),

        _ => panic!("Invalid opcode for do_opcode_ib: 0x{:X}", opcode),
    };
}

/**
 * Handle operations with immediate word arguments
 */
fn do_opcode_iw(cs: &mut CpuState, opcode: Byte) {
    let immediate = cs.read_w();

    match opcode {
        0x05 => operations::w_add(cs, Reg16::AX, immediate),

        0x3D => operations::w_cmp_ri(cs, Reg16::AX, immediate),

        0xB8 => operations::w_mov_ir(cs, Reg16::AX, immediate),
        0xB9 => operations::w_mov_ir(cs, Reg16::CX, immediate),
        0xBA => operations::w_mov_ir(cs, Reg16::DX, immediate),
        0xBB => operations::w_mov_ir(cs, Reg16::BX, immediate),
        0xBC => operations::w_mov_ir(cs, Reg16::SP, immediate),
        0xBE => operations::w_mov_ir(cs, Reg16::SI, immediate),
        0xBF => operations::w_mov_ir(cs, Reg16::DI, immediate),

        0xE8 => operations::call(cs, immediate),
        0xE9 => operations::w_jmp(cs, immediate),

        _ => panic!("Invalid opcode for do_opcode_iw: 0x{:X}", opcode),
    };
}
