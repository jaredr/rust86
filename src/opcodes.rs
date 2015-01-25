use cstate::{CpuState, Reg8, Reg16};
use datatypes::Byte;
use debugger;
use modrm;
use operations;
use operations::{b_op, b_op_dry, tf_b_add, tf_b_sub, tf_b_or, tf_b_noop};
use operand::Operand;


type F = fn(&mut CpuState, u8);

pub fn do_opcode(cs: &mut CpuState, opcode: Byte) {
    // TODO - Don't duplicate opcode definitions here and in their do_* method

    let func: F = match opcode {
        0x04 |
        0x3C |
        0x72 |
        0x74...0x79 |
        0xB0 |
        0xB1 |
        0xB2 |
        0xB3 |
        0xB4 |
        0xB5 |
        0xB6 |
        0xB7 |
        0xEB => b_opcode_i,

        0x05 |
        0x2D |
        0x3D |
        0xB8 |
        0xB9 |
        0xBA...0xBF |
        0xE8 |
        0xE9 => w_opcode_i,

        0x86 |
        0x88 |
        0x8A |
        0x38 => b_opcode_m,

        0x01 |
        0x09 |
        0x19 |
        0x20 |
        0x29 |
        0x31 |
        0x39 |
        0x89 |
        0x8B => w_opcode_m,

        0xC6 => b_opcode_mi,

        0xC7 => w_opcode_mi,

        0x40...0x4C |
        0x50...0x5F |
        0x92 |
        0xC3 |
        0xF9 => opcode_noargs,

        0x80 => b_group_i,
        0x81 => w_group_i,

        0xFE => b_group_noargs,

        0xF4 |
        0x90 => special,

        _ => {
            debugger::dump_state(cs);
            debugger::dump_vram(cs);
            panic!("Unrecognized opcode: 0x{:X}", opcode);
        }
    };

    func(cs, opcode);
}

fn b_opcode_i(cs: &mut CpuState, opcode: Byte) {
    let immediate_byte = cs.read_b();
    let immediate = Operand::RawByte(immediate_byte);

    match opcode {
        0x04 => b_op(cs, Operand::Reg8(Reg8::AL), immediate, tf_b_add),

        0x3C => b_op_dry(cs, Operand::Reg8(Reg8::AL), immediate, tf_b_sub),

        0x72 => operations::b_jmp_flag(cs, CpuState::carry, false, immediate_byte),
        0x74 => operations::b_jmp_flag(cs, CpuState::zero, false, immediate_byte),
        0x75 => operations::b_jmp_flag(cs, CpuState::zero, true, immediate_byte),
        0x76 => operations::b_jmp_flags(cs, CpuState::carry, CpuState::zero, false, immediate_byte),
        0x77 => operations::b_jmp_flags(cs, CpuState::carry, CpuState::zero, true, immediate_byte),
        0x79 => operations::b_jmp_flag(cs, CpuState::sign, true, immediate_byte),

        0xB0 => b_op(cs, Operand::Reg8(Reg8::AL), immediate, tf_b_noop),
        0xB1 => b_op(cs, Operand::Reg8(Reg8::CL), immediate, tf_b_noop),
        0xB2 => b_op(cs, Operand::Reg8(Reg8::DL), immediate, tf_b_noop),
        0xB3 => b_op(cs, Operand::Reg8(Reg8::BL), immediate, tf_b_noop),
        0xB4 => b_op(cs, Operand::Reg8(Reg8::AH), immediate, tf_b_noop),
        0xB5 => b_op(cs, Operand::Reg8(Reg8::CH), immediate, tf_b_noop),
        0xB6 => b_op(cs, Operand::Reg8(Reg8::DH), immediate, tf_b_noop),
        0xB7 => b_op(cs, Operand::Reg8(Reg8::BH), immediate, tf_b_noop),

        0xEB => operations::b_jmp(cs, immediate_byte),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_i(cs: &mut CpuState, opcode: Byte) {
    let immediate = cs.read_w();

    match opcode {
        0x05 => operations::w_add(cs, Reg16::AX, immediate),

        0x2D => operations::w_sub_ri(cs, Reg16::AX, immediate),
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

        _ => panic!("Invalid opcode"),
    };
}

fn b_opcode_m(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();

    let eff = mb.effective();
    let reg = mb.register();
    let eff = Operand::Modrm(eff);
    let reg = Operand::Modrm(reg);

    match opcode {
        0x86 => operations::b_xchg_eg(cs, mb.effective(), mb.register()),

        0x88 => b_op(cs, eff, reg, tf_b_noop),
        0x8A => b_op(cs, reg, eff, tf_b_noop),
        0x38 => b_op_dry(cs, eff, reg, tf_b_sub),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_m(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();
    let effective = mb.effective();
    let register = mb.register();

    // let dest = Operand::Modrm(effective)
    // let src = Operand::Modrm(effective)

    match opcode {
        // 0x01 => w_op(cs, dest, src, tf::w_add)
        0x01 => operations::w_add_eg(cs, effective, register),
        0x09 => operations::w_or_eg(cs, effective, register),
        0x19 => operations::w_sbb_eg(cs, effective, register),
        0x20 => operations::w_and_eg(cs, effective, register),
        0x29 => operations::w_sub_eg(cs, effective, register),
        0x31 => operations::w_xor_eg(cs, effective, register),
        // 0x39 => w_op_dry(cs, dest, src, tf::w_sub)
        0x39 => operations::w_cmp_eg(cs, effective, register),
        0x89 => operations::w_mov_eg(cs, effective, register),
        0x8B => operations::w_mov_ge(cs, effective, register),

        _ => panic!("Invalid opcode"),
    };
}

fn b_opcode_mi(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();
    let eff = mb.effective();
    let eff = Operand::Modrm(eff);

    let immediate = cs.read_b();
    let immediate = Operand::RawByte(immediate);

    match opcode {
        0xC6 => b_op(cs, eff, immediate, tf_b_noop),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_mi(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();
    let effective = mb.effective();
    let immediate = cs.read_w();

    match opcode {
        0xC7 => operations::w_mov_ei(cs, effective, immediate),

        _ => panic!("Invalid opcode"),
    };
}

fn b_group_i(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x80 {
        panic!("Invalid opcode");
    }

    let mb = cs.read_modrm();
    let eff = mb.effective();
    let eff = Operand::Modrm(eff);

    let immediate = cs.read_b();
    let immediate = Operand::RawByte(immediate);

    match mb.reg {
        0b001 => b_op(cs, eff, immediate, tf_b_or),
        0b111 => b_op_dry(cs, eff, immediate, tf_b_sub),
        _ => panic!("b_group_i: Not Implemented: 0b{:b}", mb.reg),
    }
}

fn w_group_i(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x81 {
        panic!("Invalid opcode");
    }

    let mb = cs.read_modrm();
    let effective = mb.effective(); 
    let immediate = cs.read_w();

    match mb.reg {
        0b111 => operations::w_cmp_ei(cs, effective, immediate),
        0b101 => operations::w_sub_ei(cs, effective, immediate),
        0b010 => operations::w_adc_ei(cs, effective, immediate),
        0b000 => operations::w_add_ei(cs, effective, immediate),
        _ => println!("w_group_i: Not Implemented: 0b{:b}", mb.reg),
    }
}

fn b_group_noargs(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0xFE {
        panic!("Invalid opcode");
    }

    let mb = cs.read_modrm();
    let eff = mb.effective();
    let eff = Operand::Modrm(eff);

    match mb.reg {
        0b000 => b_op(cs, eff, Operand::RawByte(1), tf_b_add),
        0b001 => b_op(cs, eff, Operand::RawByte(1), tf_b_sub),
        _ => panic!("b_group_noargs: Invalid reg value"),
    }
}

fn opcode_noargs(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0x40 => operations::inc(cs, Reg16::AX),
        0x41 => operations::inc(cs, Reg16::CX),
        0x42 => operations::inc(cs, Reg16::DX),
        0x43 => operations::inc(cs, Reg16::BX),
        0x47 => operations::inc(cs, Reg16::DI),

        0x48 => operations::dec(cs, Reg16::AX),
        0x49 => operations::dec(cs, Reg16::CX),
        0x4A => operations::dec(cs, Reg16::DX),
        0x4B => operations::dec(cs, Reg16::BX),
        0x4C => operations::dec(cs, Reg16::DI),

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

        0x92 => operations::xchg(cs, Reg16::AX, Reg16::DX),

        0xC3 => operations::ret(cs),

        0xF9 => operations::stc(cs),

        _ => panic!("Invalid opcode"),
    };
}

fn special(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0xF4 => {
            debugger::dump_state(cs);
            debugger::dump_vram(cs);
            panic!("0xF4");
        },
        0x90 => {},

        _ => panic!("Invalid opcode"),
    };
}
