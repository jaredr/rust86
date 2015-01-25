use cstate::{CpuState, Reg8, Reg16};
use debugger;
use datatypes::Byte;
use modrm;
use operations::{b_op, w_op, b_op_dry, w_op_dry};
use operand::Operand;
use specialops;
use tf;


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
    let immediate_raw = cs.read_b();
    let immediate = Operand::RawByte(immediate_raw);

    match opcode {
        0x04 => b_op(cs, Operand::Reg8(Reg8::AL), immediate, tf::b_add),

        0x3C => b_op_dry(cs, Operand::Reg8(Reg8::AL), immediate, tf::b_sub),

        0x72 => specialops::jmp_flag(cs, CpuState::carry, false, immediate_raw),
        0x74 => specialops::jmp_flag(cs, CpuState::zero, false, immediate_raw),
        0x75 => specialops::jmp_flag(cs, CpuState::zero, true, immediate_raw),
        0x76 => specialops::jmp_flags(cs, CpuState::carry, CpuState::zero, false, immediate_raw),
        0x77 => specialops::jmp_flags(cs, CpuState::carry, CpuState::zero, true, immediate_raw),
        0x79 => specialops::jmp_flag(cs, CpuState::sign, true, immediate_raw),

        0xB0 => b_op(cs, Operand::Reg8(Reg8::AL), immediate, tf::b_noop),
        0xB1 => b_op(cs, Operand::Reg8(Reg8::CL), immediate, tf::b_noop),
        0xB2 => b_op(cs, Operand::Reg8(Reg8::DL), immediate, tf::b_noop),
        0xB3 => b_op(cs, Operand::Reg8(Reg8::BL), immediate, tf::b_noop),
        0xB4 => b_op(cs, Operand::Reg8(Reg8::AH), immediate, tf::b_noop),
        0xB5 => b_op(cs, Operand::Reg8(Reg8::CH), immediate, tf::b_noop),
        0xB6 => b_op(cs, Operand::Reg8(Reg8::DH), immediate, tf::b_noop),
        0xB7 => b_op(cs, Operand::Reg8(Reg8::BH), immediate, tf::b_noop),

        0xEB => specialops::b_jmp(cs, immediate_raw),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_i(cs: &mut CpuState, opcode: Byte) {
    let immediate_raw = cs.read_w();
    let immediate = Operand::RawWord(immediate_raw);

    match opcode {
        0x05 => w_op(cs, Operand::Reg16(Reg16::AX), immediate, tf::w_add),

        0x2D => w_op(cs, Operand::Reg16(Reg16::AX), immediate, tf::w_sub),
        0x3D => w_op_dry(cs, Operand::Reg16(Reg16::AX), immediate, tf::w_sub),

        0xB8 => w_op(cs, Operand::Reg16(Reg16::AX), immediate, tf::w_noop),
        0xB9 => w_op(cs, Operand::Reg16(Reg16::CX), immediate, tf::w_noop),
        0xBA => w_op(cs, Operand::Reg16(Reg16::DX), immediate, tf::w_noop),
        0xBB => w_op(cs, Operand::Reg16(Reg16::BX), immediate, tf::w_noop),
        0xBC => w_op(cs, Operand::Reg16(Reg16::SP), immediate, tf::w_noop),
        0xBE => w_op(cs, Operand::Reg16(Reg16::SI), immediate, tf::w_noop),
        0xBF => w_op(cs, Operand::Reg16(Reg16::DI), immediate, tf::w_noop),

        0xE8 => specialops::call(cs, immediate_raw),
        0xE9 => specialops::w_jmp(cs, immediate_raw),

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
        0x86 => specialops::xchg_modrm(cs, mb.effective(), mb.register()),

        0x88 => b_op(cs, eff, reg, tf::b_noop),
        0x8A => b_op(cs, reg, eff, tf::b_noop),
        0x38 => b_op_dry(cs, eff, reg, tf::b_sub),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_m(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();
    let eff = mb.effective();
    let reg = mb.register();
    let eff = Operand::Modrm(eff);
    let reg = Operand::Modrm(reg);

    match opcode {
        0x01 => w_op(cs, eff, reg, tf::w_add),
        0x09 => w_op(cs, eff, reg, tf::w_or),
        0x19 => w_op(cs, eff, reg, tf::w_sbb),
        0x20 => w_op(cs, eff, reg, tf::w_and),
        0x29 => w_op(cs, eff, reg, tf::w_sub),
        0x31 => w_op(cs, eff, reg, tf::w_xor),
        0x39 => w_op_dry(cs, eff, reg, tf::w_sub),
        0x89 => w_op(cs, eff, reg, tf::w_noop),
        0x8B => w_op(cs, reg, eff, tf::w_noop),

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
        0xC6 => b_op(cs, eff, immediate, tf::b_noop),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_mi(cs: &mut CpuState, opcode: Byte) {
    let mb = cs.read_modrm();
    let eff = mb.effective();
    let eff = Operand::Modrm(eff);

    let immediate_raw = cs.read_w();
    let immediate = Operand::RawWord(immediate_raw);

    match opcode {
        0xC7 => w_op(cs, eff, immediate, tf::w_noop),

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
        0b001 => b_op(cs, eff, immediate, tf::b_or),
        0b111 => b_op_dry(cs, eff, immediate, tf::b_sub),
        _ => panic!("b_group_i: Not Implemented: 0b{:b}", mb.reg),
    }
}

fn w_group_i(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x81 {
        panic!("Invalid opcode");
    }

    let mb = cs.read_modrm();
    let eff = mb.effective();
    let eff = Operand::Modrm(eff);

    let immediate_raw = cs.read_w();
    let immediate = Operand::RawWord(immediate_raw);

    match mb.reg {
        0b111 => w_op_dry(cs, eff, immediate, tf::w_sub),
        0b101 => w_op(cs, eff, immediate, tf::w_sub),
        0b010 => w_op(cs, eff, immediate, tf::w_adc),
        0b000 => w_op(cs, eff, immediate, tf::w_add),
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
        0b000 => b_op(cs, eff, Operand::RawByte(1), tf::b_add),
        0b001 => b_op(cs, eff, Operand::RawByte(1), tf::b_sub),
        _ => panic!("b_group_noargs: Invalid reg value"),
    }
}

fn opcode_noargs(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0x40 => w_op(cs, Operand::Reg16(Reg16::AX), Operand::RawWord(1), tf::w_add),
        0x41 => w_op(cs, Operand::Reg16(Reg16::CX), Operand::RawWord(1), tf::w_add),
        0x42 => w_op(cs, Operand::Reg16(Reg16::DX), Operand::RawWord(1), tf::w_add),
        0x43 => w_op(cs, Operand::Reg16(Reg16::BX), Operand::RawWord(1), tf::w_add),
        0x47 => w_op(cs, Operand::Reg16(Reg16::DI), Operand::RawWord(1), tf::w_add),

        0x48 => w_op(cs, Operand::Reg16(Reg16::AX), Operand::RawWord(1), tf::w_sub),
        0x49 => w_op(cs, Operand::Reg16(Reg16::CX), Operand::RawWord(1), tf::w_sub),
        0x4A => w_op(cs, Operand::Reg16(Reg16::DX), Operand::RawWord(1), tf::w_sub),
        0x4B => w_op(cs, Operand::Reg16(Reg16::BX), Operand::RawWord(1), tf::w_sub),
        0x4C => w_op(cs, Operand::Reg16(Reg16::DI), Operand::RawWord(1), tf::w_sub),

        0x50 => specialops::push(cs, Reg16::AX),
        0x51 => specialops::push(cs, Reg16::CX),
        0x52 => specialops::push(cs, Reg16::DX),
        0x53 => specialops::push(cs, Reg16::BX),
        0x54 => specialops::push(cs, Reg16::SP),
        0x56 => specialops::push(cs, Reg16::SI),
        0x57 => specialops::push(cs, Reg16::DI),

        0x58 => specialops::pop(cs, Reg16::AX),
        0x59 => specialops::pop(cs, Reg16::CX),
        0x5A => specialops::pop(cs, Reg16::DX),
        0x5B => specialops::pop(cs, Reg16::BX),
        0x5C => specialops::pop(cs, Reg16::SP),
        0x5E => specialops::pop(cs, Reg16::SI),
        0x5F => specialops::pop(cs, Reg16::DI),

        0x92 => specialops::xchg_reg(cs, Reg16::AX, Reg16::DX),

        0xC3 => specialops::ret(cs),

        0xF9 => specialops::stc(cs),

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
