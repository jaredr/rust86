use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use num::ToPrimitive;
use self::Reg8::*;
use self::Reg16::*;
use byteutils::{low8, high8, join8, join_low8, join_high8};
use datatypes::{Byte, Word};
use operand::Flags;


pub enum Reg16 {
    AX, BX, CX, DX,
    SI, DI, BP, SP,
    IP,
}

pub enum Reg8 {
    AH, AL,
    BH, BL,
    CH, CL,
    DH, DL,
}

pub struct CpuState {
    _state: Vec<u8>,

    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,

    si: u16,
    di: u16,
    sp: u16,
    bp: u16,

    ip: u16, // Instruction pointer

    cf: bool, // Carry flag
    of: bool, // Overflow flag
    sf: bool, // Sign flag
    zf: bool, // Zero flag
}

impl CpuState {
    pub fn new() -> CpuState {
        let mut mem = Vec::new();
        mem.resize(65535, 0u8);

        CpuState {
            _state: mem,

            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            si: 0,
            di: 0,
            sp: 0x100,
            bp: 0,
            ip: 0,

            cf: false,
            of: false,
            sf: false,
            zf: false,
        }
    }

    pub fn load_program(&mut self, path: &Path) {
        let mut prog = Vec::new();
        File::open(path).unwrap().read_to_end(&mut prog).unwrap();
        for byte in prog.iter().rev() {
            self._state.pop();
            self._state.insert(0, *byte);
        }
    }

    pub fn getmem(&self, i: Word) -> Byte {
        let idx = i.to_usize().unwrap();
        let val = self._state[idx];
        let val = val.to_u8().unwrap();
        val
    }

    pub fn setmem(&mut self, addr: Word, val: Byte) {
        let idx = addr.to_usize().unwrap();
        let val8 = val.to_u8().unwrap();
        self._state[idx] = val8
    }


    /// Get the current value of the specified 16-bit register.
    pub fn getreg16(&self, reg: &Reg16) -> Word {
        match *reg {
            AX => return self.ax,
            BX => return self.bx,
            CX => return self.cx,
            DX => return self.dx,
            SI => return self.si,
            DI => return self.di,
            SP => return self.sp,
            BP => return self.bp,
            IP => return self.ip,
        }
    }

    /// Get the current value of the specified 8-bit register.
    pub fn getreg8(&self, reg: &Reg8) -> Byte {
        match *reg {
            AL => return high8(self.ax),
            BL => return high8(self.bx),
            CL => return high8(self.cx),
            DL => return high8(self.dx),
            AH => return low8(self.ax),
            BH => return low8(self.bx),
            CH => return low8(self.cx),
            DH => return low8(self.dx),
        }
    }

    /// Set the current value of the specified 16-bit register.
    pub fn setreg16(&mut self, reg: &Reg16, new_val: Word) {
        match *reg {
            AX => self.ax = new_val,
            BX => self.bx = new_val,
            CX => self.cx = new_val,
            DX => self.dx = new_val,
            SI => self.si = new_val,
            DI => self.di = new_val,
            SP => self.sp = new_val,
            BP => self.bp = new_val,
            IP => self.ip = new_val,
        }
    }

    /// Set the current value of the specified 16-bit register.
    pub fn setreg8(&mut self, reg: &Reg8, new_val: Byte) {
        match *reg {
            AL => self.ax = join_high8(self.ax, new_val),
            BL => self.bx = join_high8(self.bx, new_val),
            CL => self.cx = join_high8(self.cx, new_val),
            DL => self.dx = join_high8(self.dx, new_val),
            AH => self.ax = join_low8(self.ax, new_val),
            BH => self.bx = join_low8(self.bx, new_val),
            CH => self.cx = join_low8(self.cx, new_val),
            DH => self.dx = join_low8(self.dx, new_val),
        }
    }

    /// Read a Byte from the memory location at `ip` and advance `ip`.
    pub fn read(&mut self) -> Byte {
        let byte: Byte = self.getmem(self.ip);
        self.ip += 1;

        byte
    }

    /// Read a Word from the memory location at `ip` and advance `ip`.
    pub fn read16(&mut self) -> Word {
        let high_b: Byte = self.read();
        let low_b: Byte = self.read();
        let word: Word = join8(low_b, high_b);

        word
    }

    pub fn push(&mut self, val: Word) {
        let low_b = low8(val);
        let high_b = high8(val);
        let sp = self.sp;
        self.setmem(sp - 2, high_b);
        self.setmem(sp - 1, low_b);
        self.sp = sp - 2;
    }

    pub fn pop(&mut self) -> Word {
        let low_b = self.getmem(self.sp + 1);
        let high_b = self.getmem(self.sp);
        self.sp = self.sp + 2;
        join8(low_b, high_b)
    }

    pub fn set_flags(&mut self, f: Flags) {
        self.cf = f.carry;
        self.of = f.overflow;
        self.sf = f.sign;
        self.zf = f.zero;
    }

    pub fn get_flags(&self) -> Flags {
        Flags {
            carry: self.cf,
            overflow: self.of,
            sign: self.sf,
            zero: self.zf,
        }
    }

    pub fn zero(&self) -> bool {
        self.zf
    }

    pub fn sign(&self) -> bool {
        self.sf
    }

    pub fn carry(&self) -> bool {
        self.cf
    }

    pub fn set_carry(&mut self) {
        self.cf = true;
    }
}
