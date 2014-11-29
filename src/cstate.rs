use std::io::File;
use byteutils::{low8, high8, join8, join_low8, join_high8};
use datatypes::{Byte, Word};


pub enum Register {
    AX, AH, AL,
    BX, BH, BL,
    CX, CH, CL,
    DX, DH, DL,
    SI,
    DI,
    SP,
    BP,
    IP,
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

    ip: u16,
}

impl CpuState {
    pub fn read_from_file() -> CpuState {
        // Initialize memory as a 65535-byte vector with the input program
        // starting at address zero.
        let mut mem = File::open(&Path::new("test.bin")).read_to_end().unwrap();
        let mem_len = mem.len();
        mem.grow(65535 - mem_len, 0u8);

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
        }
    }

    pub fn getmem_b(&self, i: Word) -> Byte {
        let idx = i.to_uint().unwrap();
        let value = self._state[idx];
        let value16 = value.to_u16().unwrap();
        value16
    }

    pub fn setmem_b(&mut self, addr: Word, value: Byte) {
        let idx = addr.to_uint().unwrap();
        let value8 = value.to_u8().unwrap();
        self._state[idx] = value8
    }

    /**
     * Get the current value of the specified register.
     * Returns either a Byte or a Word, depending on the register.
     */
    pub fn getreg(&self, reg: Register) -> u16 {
        match reg {
            AX => return self.ax,
            BX => return self.bx,
            CX => return self.cx,
            DX => return self.dx,
            AL => return low8(self.ax),
            BL => return low8(self.bx),
            CL => return low8(self.cx),
            DL => return low8(self.dx),
            AH => return high8(self.ax),
            BH => return high8(self.bx),
            CH => return high8(self.cx),
            DH => return high8(self.dx),
            SI => return self.si,
            DI => return self.di,
            SP => return self.sp,
            BP => return 0,
            IP => return self.ip,
        }
    }

    /**
     * Set the current value of the specified register.
     * `new_value' should be Byte for 8-bit registers and Word for 16-bit
     * registers.
     */
    pub fn setreg(&mut self, reg: Register, new_value: u16) {
        // TODO - Bounds check on 8-bit registers
        match reg {
            AX => self.ax = new_value,
            BX => self.bx = new_value,
            CX => self.cx = new_value,
            DX => self.dx = new_value,
            AL => self.ax = join_low8(self.ax, new_value),
            BL => self.bx = join_low8(self.bx, new_value),
            CL => self.cx = join_low8(self.cx, new_value),
            DL => self.dx = join_low8(self.dx, new_value),
            AH => self.ax = join_high8(self.ax, new_value),
            BH => self.bx = join_high8(self.bx, new_value),
            CH => self.cx = join_high8(self.cx, new_value),
            DH => self.dx = join_high8(self.dx, new_value),
            SI => self.si = new_value,
            DI => self.di = new_value,
            SP => {},
            BP => {},
            IP => self.ip = new_value,
        }
    }

    pub fn dump_state(&self) {
        println!("\n*** Begin Processor State Dump ***");
        self.dump_register("ax", AX, AL, AH);
        self.dump_register("bx", BX, BL, BH);
        self.dump_register("cx", CX, CL, CH);
        self.dump_register("dx", DX, DL, DH);
        self.dump_mem(0x8000);
        self.dump_mem(0x8010);
        self.dump_mem(0x8020);
        self.dump_mem(0x8030);
        self.dump_mem(0x8040);
        self.dump_mem(0x8050);
        println!("*** End Processor State Dump ***");
    }

    fn dump_register(&self, name: &str, x: Register, l: Register, h: Register) {
        println!(
            "{}     0x{: <5X} (0x{:X} 0x{:X})",
            name,
            self.getreg(x),
            self.getreg(l),
            self.getreg(h),
        );
    }

    fn dump_mem(&self, start: Word) {
        let mut s_hex = String::new();
        let mut s_chr = String::new();
        for i in range(0, 16) {
            let val: Byte = self.getmem_b(start+i);
            let val_u8: u8 = val.to_u8().unwrap();
            s_hex.push_str(format!("{:0>2X} ", val).as_slice());
            s_chr.push_str(format!("{:c}", val_u8 as char).as_slice());
        }
        println!("mem    0x{:0>5X} {} {}", start, s_hex, s_chr);
    }

    /**
     * Read a Byte from the memory location at `ip` and advance `ip`.
     */
    pub fn read_b(&mut self) -> Byte {
        let byte: Byte = self.getmem_b(self.ip);
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

    pub fn push(&mut self, value: Word) {
        let low_b = low8(value);
        let high_b = high8(value);
        let sp = self.sp;
        self.setmem_b(sp - 1, low_b);
        self.setmem_b(sp - 2, high_b);
        self.sp = sp - 2;
    }

    pub fn pop(&mut self) -> Word {
        let sp = self.sp;
        let low_b = self.getmem_b(sp);
        let high_b = self.getmem_b(sp + 1);
        self.setmem_b(sp, 0x0);
        self.setmem_b(sp + 1, 0x0);
        self.sp = sp + 2;
        join8(low_b, high_b)
    }
}
