use cstate::{CpuState, Reg8, Reg16};
use datatypes::Byte;
use modrm;
use operation::{op8, op16, op8_dry, op16_dry};
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
        0x90 |
        0x92 |
        0xC3 |
        0xF9 => opcode_noargs,

        0x80 => b_group_i,
        0x81 => w_group_i,

        0xFE => b_group_noargs,

        _ => {
            panic!("Unrecognized opcode: 0x{:X}", opcode);
        }
    };

    func(cs, opcode);
}

fn b_opcode_i(cs: &mut CpuState, opcode: Byte) {
    let immediate_raw = cs.read();
    let immediate = Operand::RawByte(immediate_raw);

    match opcode {
        0x04 => op8(cs, Operand::Reg8(Reg8::AL), immediate, tf::add8),

        0x3C => op8_dry(cs, Operand::Reg8(Reg8::AL), immediate, tf::sub8),

        0x72 => specialops::jmp_flag(cs, CpuState::carry, false, immediate_raw),
        0x74 => specialops::jmp_flag(cs, CpuState::zero, false, immediate_raw),
        0x75 => specialops::jmp_flag(cs, CpuState::zero, true, immediate_raw),
        0x76 => specialops::jmp_flags(cs,
                                      CpuState::carry,
                                      CpuState::zero,
                                      false,
                                      immediate_raw),
        0x77 => specialops::jmp_flags(cs,
                                      CpuState::carry,
                                      CpuState::zero,
                                      true,
                                      immediate_raw),
        0x79 => specialops::jmp_flag(cs, CpuState::sign, true, immediate_raw),

        0xB0 => op8(cs, Operand::Reg8(Reg8::AL), immediate, tf::noop8),
        0xB1 => op8(cs, Operand::Reg8(Reg8::CL), immediate, tf::noop8),
        0xB2 => op8(cs, Operand::Reg8(Reg8::DL), immediate, tf::noop8),
        0xB3 => op8(cs, Operand::Reg8(Reg8::BL), immediate, tf::noop8),
        0xB4 => op8(cs, Operand::Reg8(Reg8::AH), immediate, tf::noop8),
        0xB5 => op8(cs, Operand::Reg8(Reg8::CH), immediate, tf::noop8),
        0xB6 => op8(cs, Operand::Reg8(Reg8::DH), immediate, tf::noop8),
        0xB7 => op8(cs, Operand::Reg8(Reg8::BH), immediate, tf::noop8),

        0xEB => specialops::jmp8(cs, immediate_raw),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_i(cs: &mut CpuState, opcode: Byte) {
    let immediate_raw = cs.read16();
    let immediate = Operand::RawWord(immediate_raw);

    match opcode {
        0x05 => op16(cs, Operand::Reg16(Reg16::AX), immediate, tf::add16),

        0x2D => op16(cs, Operand::Reg16(Reg16::AX), immediate, tf::sub16),
        0x3D => op16_dry(cs, Operand::Reg16(Reg16::AX), immediate, tf::sub16),

        0xB8 => op16(cs, Operand::Reg16(Reg16::AX), immediate, tf::noop16),
        0xB9 => op16(cs, Operand::Reg16(Reg16::CX), immediate, tf::noop16),
        0xBA => op16(cs, Operand::Reg16(Reg16::DX), immediate, tf::noop16),
        0xBB => op16(cs, Operand::Reg16(Reg16::BX), immediate, tf::noop16),
        0xBC => op16(cs, Operand::Reg16(Reg16::SP), immediate, tf::noop16),
        0xBE => op16(cs, Operand::Reg16(Reg16::SI), immediate, tf::noop16),
        0xBF => op16(cs, Operand::Reg16(Reg16::DI), immediate, tf::noop16),

        0xE8 => specialops::call(cs, immediate_raw),
        0xE9 => specialops::jmp16(cs, immediate_raw),

        _ => panic!("Invalid opcode"),
    };
}

fn b_opcode_m(cs: &mut CpuState, opcode: Byte) {
    let (_, eff, reg) = modrm::read_modrm(cs, true);

    match opcode {
        0x86 => specialops::xchg8(cs, eff, reg),

        0x88 => op8(cs, eff, reg, tf::noop8),
        0x8A => op8(cs, reg, eff, tf::noop8),
        0x38 => op8_dry(cs, eff, reg, tf::sub8),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_m(cs: &mut CpuState, opcode: Byte) {
    let (_, eff, reg) = modrm::read_modrm(cs, false);

    match opcode {
        0x01 => op16(cs, eff, reg, tf::add16),
        0x09 => op16(cs, eff, reg, tf::or16),
        0x19 => op16(cs, eff, reg, tf::sbb16),
        0x20 => op16(cs, eff, reg, tf::and16),
        0x29 => op16(cs, eff, reg, tf::sub16),
        0x31 => op16(cs, eff, reg, tf::xor16),
        0x39 => op16_dry(cs, eff, reg, tf::sub16),
        0x89 => op16(cs, eff, reg, tf::noop16),
        0x8B => op16(cs, reg, eff, tf::noop16),

        _ => panic!("Invalid opcode"),
    };
}

fn b_opcode_mi(cs: &mut CpuState, opcode: Byte) {
    let (_, eff, _) = modrm::read_modrm(cs, true);

    let immediate = cs.read();
    let immediate = Operand::RawByte(immediate);

    match opcode {
        0xC6 => op8(cs, eff, immediate, tf::noop8),

        _ => panic!("Invalid opcode"),
    };
}

fn w_opcode_mi(cs: &mut CpuState, opcode: Byte) {
    let (_, eff, _) = modrm::read_modrm(cs, false);

    let immediate_raw = cs.read16();
    let immediate = Operand::RawWord(immediate_raw);

    match opcode {
        0xC7 => op16(cs, eff, immediate, tf::noop16),

        _ => panic!("Invalid opcode"),
    };
}

fn b_group_i(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x80 {
        panic!("Invalid opcode");
    }

    let (rb, eff, _) = modrm::read_modrm(cs, true);

    let immediate = cs.read();
    let immediate = Operand::RawByte(immediate);

    match rb {
        0b001 => op8(cs, eff, immediate, tf::or8),
        0b111 => op8_dry(cs, eff, immediate, tf::sub8),
        _ => panic!("b_group_i: Not Implemented: 0b{:b}", rb),
    }
}

fn w_group_i(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0x81 {
        panic!("Invalid opcode");
    }

    let (rb, eff, _) = modrm::read_modrm(cs, false);

    let immediate_raw = cs.read16();
    let immediate = Operand::RawWord(immediate_raw);

    match rb {
        0b111 => op16_dry(cs, eff, immediate, tf::sub16),
        0b101 => op16(cs, eff, immediate, tf::sub16),
        0b010 => op16(cs, eff, immediate, tf::adc16),
        0b000 => op16(cs, eff, immediate, tf::add16),
        _ => println!("w_group_i: Not Implemented: 0b{:b}", rb),
    }
}

fn b_group_noargs(cs: &mut CpuState, opcode: Byte) {
    if opcode != 0xFE {
        panic!("Invalid opcode");
    }

    let (rb, eff, _) = modrm::read_modrm(cs, true);

    match rb {
        0b000 => op8(cs, eff, Operand::RawByte(1), tf::add8),
        0b001 => op8(cs, eff, Operand::RawByte(1), tf::sub8),
        _ => panic!("b_group_noargs: Invalid reg value"),
    }
}

fn opcode_noargs(cs: &mut CpuState, opcode: Byte) {
    match opcode {
        0x40 => op16(cs, Operand::Reg16(Reg16::AX), Operand::RawWord(1), tf::add16),
        0x41 => op16(cs, Operand::Reg16(Reg16::CX), Operand::RawWord(1), tf::add16),
        0x42 => op16(cs, Operand::Reg16(Reg16::DX), Operand::RawWord(1), tf::add16),
        0x43 => op16(cs, Operand::Reg16(Reg16::BX), Operand::RawWord(1), tf::add16),
        0x47 => op16(cs, Operand::Reg16(Reg16::DI), Operand::RawWord(1), tf::add16),

        0x48 => op16(cs, Operand::Reg16(Reg16::AX), Operand::RawWord(1), tf::sub16),
        0x49 => op16(cs, Operand::Reg16(Reg16::CX), Operand::RawWord(1), tf::sub16),
        0x4A => op16(cs, Operand::Reg16(Reg16::DX), Operand::RawWord(1), tf::sub16),
        0x4B => op16(cs, Operand::Reg16(Reg16::BX), Operand::RawWord(1), tf::sub16),
        0x4C => op16(cs, Operand::Reg16(Reg16::DI), Operand::RawWord(1), tf::sub16),

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

        0x90 => {},

        0x92 => specialops::xchg16(cs,
                                   Operand::Reg16(Reg16::AX),
                                   Operand::Reg16(Reg16::DX)),

        0xC3 => specialops::ret(cs),

        0xF9 => specialops::stc(cs),

        _ => panic!("Invalid opcode"),
    };
}
