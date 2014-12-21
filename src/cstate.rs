use self::Reg8::*;
use self::Reg16::*;
use std::vec::Vec;
use std::io::File;
use byteutils::{b_add, w_add, b_sub, w_sub, low8, high8, join8, join_low8, join_high8};
use datatypes::{Byte, Word};


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
        mem.grow(65535, 0u8);

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
        let prog = File::open(path).read_to_end().unwrap();
        for byte in prog.iter().rev() {
            self._state.pop();
            self._state.insert(0, *byte);
        }
    }

    pub fn getmem(&self, i: Word) -> Byte {
        let idx = i.to_uint().unwrap();
        let val = self._state[idx];
        let val = val.to_u8().unwrap();
        val
    }

    pub fn setmem(&mut self, addr: Word, val: Byte) {
        let idx = addr.to_uint().unwrap();
        let val8 = val.to_u8().unwrap();
        self._state[idx] = val8
    }

    /**
     * Get the current value of the specified register.
     * Returns either a Byte or a Word, depending on the register.
     */
    pub fn getreg_w(&self, reg: &Reg16) -> Word {
        match *reg {
            AX => return self.ax,
            BX => return self.bx,
            CX => return self.cx,
            DX => return self.dx,
            SI => return self.si,
            DI => return self.di,
            SP => return self.sp,
            BP => return 0,
            IP => return self.ip,
        }
    }

    pub fn getreg_b(&self, reg: &Reg8) -> Byte {
        match *reg {
            AL => return low8(self.ax),
            BL => return low8(self.bx),
            CL => return low8(self.cx),
            DL => return low8(self.dx),
            AH => return high8(self.ax),
            BH => return high8(self.bx),
            CH => return high8(self.cx),
            DH => return high8(self.dx),
        }
    }

    /**
     * Set the current value of the specified register.
     * `new_val' should be Byte for 8-bit registers and Word for 16-bit
     * registers.
     */
    pub fn setreg_w(&mut self, reg: &Reg16, new_val: Word) {
        match *reg {
            AX => self.ax = new_val,
            BX => self.bx = new_val,
            CX => self.cx = new_val,
            DX => self.dx = new_val,
            SI => self.si = new_val,
            DI => self.di = new_val,
            SP => {},
            BP => {},
            IP => {},
        }
    }

    pub fn setreg_b(&mut self, reg: &Reg8, new_val: Byte) {
        match *reg {
            AL => self.ax = join_low8(self.ax, new_val),
            BL => self.bx = join_low8(self.bx, new_val),
            CL => self.cx = join_low8(self.cx, new_val),
            DL => self.dx = join_low8(self.dx, new_val),
            AH => self.ax = join_high8(self.ax, new_val),
            BH => self.bx = join_high8(self.bx, new_val),
            CH => self.cx = join_high8(self.cx, new_val),
            DH => self.dx = join_high8(self.dx, new_val),
        }
    }

    pub fn zero(&self) -> bool {
        self.zf
    }

    /**
     * Read a Byte from the memory location at `ip` and advance `ip`.
     */
    pub fn read_b(&mut self) -> Byte {
        let byte: Byte = self.getmem(self.ip);
        self.ip += 1;

        byte
    }
    
    /**
     * Read a Word from the memory location at `ip` and advance `ip`.
     */
    pub fn read_w(&mut self) -> Word {
        let left_b: Byte = self.read_b();
        let right_b: Byte = self.read_b();
        let word: Word = join8(left_b, right_b);

        word
    }

    /**
     * Push the given value onto the stack.
     */
    pub fn push(&mut self, val: Word) {
        let low_b = low8(val);
        let high_b = high8(val);
        let sp = self.sp;
        self.setmem(sp - 1, low_b);
        self.setmem(sp - 2, high_b);
        self.sp = sp - 2;
    }

    /**
     * Pop and return the top value from the stack.
     */
    pub fn pop(&mut self) -> Word {
        let sp = self.sp;
        let low_b = self.getmem(sp);
        let high_b = self.getmem(sp + 1);
        self.setmem(sp, 0x0);
        self.setmem(sp + 1, 0x0);
        self.sp = sp + 2;
        join8(low_b, high_b)
    }

    /**
     * Move `ip` by the given twos-complement byte offset.
     */
    pub fn jump_b(&mut self, offset: Byte) {
        let offset = offset.to_u16().unwrap();
        if offset < 127 {
            self.ip += offset;
        } else {
            self.ip -= (256 - offset);
        }
    }

    /**
     * Move `ip` by the given unsigned word offset.
     */
    pub fn jump_w(&mut self, offset: Word) {
        self.ip += offset;
    }

    /**
     * Push `ip`, then jmp `offset`
     */
    pub fn call(&mut self, offset: Word) {
        let ip = self.ip;
        self.push(ip);
        self.jump_w(offset);
    }

    /**
     * Pop the top value from the stack into `ip`.
     */
    pub fn ret(&mut self) {
        self.ip = self.pop();
    }

    /**
     * Wrapper around byteutils::b_add that sets flags on this CpuState.
     */
    pub fn b_add(&mut self, left: Byte, right: Byte) -> Byte {
        let (result, cf, of, sf, zf) = b_add(left, right);
        self.set_flags(cf, of, sf, zf);
        result
    }

    pub fn w_add(&mut self, left: Word, right: Word) -> Word {
        let (result, cf, of, sf, zf) = w_add(left, right);
        self.set_flags(cf, of, sf, zf);
        result
    }

    pub fn b_sub(&mut self, left: Byte, right: Byte) -> Byte {
        let (result, cf, of, sf, zf) = b_sub(left, right);
        self.set_flags(cf, of, sf, zf);
        result
    }

    pub fn w_sub(&mut self, left: Word, right: Word) -> Word {
        let (result, cf, of, sf, zf) = w_sub(left, right);
        self.set_flags(cf, of, sf, zf);
        result
    }

    fn set_flags(&mut self, cf: bool, of: bool, sf: bool, zf: bool) {
        self.cf = cf;
        self.of = of;
        self.sf = sf;
        self.zf = zf;
    }
}
