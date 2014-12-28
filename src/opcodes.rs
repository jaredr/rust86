use operations;
use debugger;
use cstate::{CpuState, Reg8, Reg16};
use datatypes::Byte;


pub fn do_opcode(cs: &mut CpuState, opcode: Byte) {
    match opcode {
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

        0xE9 => operations::w_jmp(cs),
        0xEB => operations::b_jmp(cs),

        0xE8 => operations::call(cs),
        0xC3 => operations::ret(cs),

        0x74 => operations::jz(cs),

        0x88 => operations::b_mov_eg(cs),
        0x89 => operations::w_mov_eg(cs),
        0x8A => operations::b_mov_ge(cs),
        0x8B => operations::w_mov_ge(cs),
        0xC6 => operations::mov_e(cs),

        0xB0 => operations::b_mov_ir(cs, Reg8::AL),
        0xB1 => operations::b_mov_ir(cs, Reg8::CL),
        0xB2 => operations::b_mov_ir(cs, Reg8::DL),
        0xB3 => operations::b_mov_ir(cs, Reg8::BL),
        0xB4 => operations::b_mov_ir(cs, Reg8::AH),
        0xB5 => operations::b_mov_ir(cs, Reg8::CH),
        0xB6 => operations::b_mov_ir(cs, Reg8::DH),
        0xB7 => operations::b_mov_ir(cs, Reg8::BH),

        0xB8 => operations::w_mov_ir(cs, Reg16::AX),
        0xB9 => operations::w_mov_ir(cs, Reg16::CX),
        0xBA => operations::w_mov_ir(cs, Reg16::DX),
        0xBB => operations::w_mov_ir(cs, Reg16::BX),
        0xBC => operations::w_mov_ir(cs, Reg16::SP),
        0xBE => operations::w_mov_ir(cs, Reg16::SI),
        0xBF => operations::w_mov_ir(cs, Reg16::DI),

        0x04 => operations::b_add(cs, Reg8::AL),
        0x05 => operations::w_add(cs, Reg16::AX),

        0x38 => operations::b_cmp_eg(cs),
        0x3C => operations::b_cmp_ri(cs, Reg8::AL),
        0x3D => operations::w_cmp_ri(cs, Reg16::AX),

        0xF4 => {
            debugger::dump_state(cs);
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Unrecognized opcode: 0x{:X}", opcode),
    };
}
