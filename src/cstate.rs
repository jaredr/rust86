use std::io::File;

pub enum Register {
    AX, AH, AL,
    BX, BH, BL,
    CX, CH, CL,
    DX, DH, DL,
    SI,
    DI,
    SP,
    BP,
}

pub struct CpuState {
    _state: Vec<u8>,

    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,

    pub si: u16,
    pub di: u16,
    pub sp: u16,
    pub bp: u16,

    pub ip: u16,
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
            sp: 0,
            bp: 0,
            ip: 0,
        }
    }

    pub fn getmem(&self, i: u16) -> u8 {
        let idx = i.to_uint().unwrap();
        self._state[idx]
    }

    pub fn setmem(&mut self, addr: u16, value: u8) {
        let idx = addr.to_uint().unwrap();
        self._state[idx] = value
    }

    pub fn getreg(&self, reg: Register) -> u16 {
        match reg {
            AX => return self.ax,
            BX => return self.bx,
            CX => return self.cx,
            DX => return self.dx,
            AL => return CpuState::low8(self.ax),
            BL => return CpuState::low8(self.bx),
            CL => return CpuState::low8(self.cx),
            DL => return CpuState::low8(self.dx),
            AH => return CpuState::high8(self.ax),
            BH => return CpuState::high8(self.bx),
            CH => return CpuState::high8(self.cx),
            DH => return CpuState::high8(self.dx),
            SI => return 0,
            DI => return 0,
            SP => return 0,
            BP => return 0,
        }
    }
    pub fn setreg(&mut self, reg: Register, new_value: u16) {
        // TODO - Bounds check on 8-bit registers
        match reg {
            AX => self.ax = new_value,
            BX => self.bx = new_value,
            CX => self.cx = new_value,
            DX => self.dx = new_value,
            AL => self.ax = CpuState::join_low8(self.ax, new_value),
            BL => self.bx = CpuState::join_low8(self.bx, new_value),
            CL => self.cx = CpuState::join_low8(self.cx, new_value),
            DL => self.dx = CpuState::join_low8(self.dx, new_value),
            AH => self.ax = CpuState::join_high8(self.ax, new_value),
            BH => self.bx = CpuState::join_high8(self.bx, new_value),
            CH => self.cx = CpuState::join_high8(self.cx, new_value),
            DH => self.dx = CpuState::join_high8(self.dx, new_value),
            SI => {},
            DI => {},
            SP => {},
            BP => {},
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

    fn dump_mem(&self, start: u16) {
        let mut s_hex = String::new();
        let mut s_chr = String::new();
        for i in range(0, 16) {
            let val = self.getmem(start+i);
            s_hex.push_str(format!("{:0>2X} ", val).as_slice());
            s_chr.push_str(format!("{:c}", val as char).as_slice());
        }
        println!("mem    0x{:0>5X} {} {}", start, s_hex, s_chr);
    }

    /**
     * Read from the memory location at `ip` and advance `ip`.
     */
    pub fn read(&mut self) -> u8 {
        let byte: u8 = self.getmem(self.ip);
        self.ip += 1;

        println!("(read) 0x{:X}", byte);
        byte
    }
    
    /**
     * Read a WORD from the memory location at `ip` and advance `ip`
     * past the WORD.
     */
    pub fn read_word(&mut self) -> u16 {
        let left: u8 = self.read();
        let right: u8 = self.read();
        let word = CpuState::word_up(left, right);

        println!("(read_word) 0x{:X}", word);
        word
    }

    fn word_up(left: u8, right: u8) -> u16 {
        let left: u16 = left.to_u16().unwrap();
        let right: u16 = right.to_u16().unwrap();
        CpuState::join8(left, right)
    }

    // TODO - Move these methods to a new module

    fn low8(val: u16) -> u16 {
        (val >> 8)
    }
        
    fn high8(val: u16) -> u16 {
        (val & 0xFF)
    }

    fn join8(low: u16, high: u16) -> u16 {
        let mut word: u16 = high;
        word = word << 8;
        word = word + low;
        word
    }

    fn join_low8(val: u16, low: u16) -> u16 {
        CpuState::join8(low, CpuState::high8(val))
    }

    fn join_high8(val: u16, high: u16) -> u16 {
        CpuState::join8(CpuState::low8(val), high)
    }
}
