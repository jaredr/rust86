use std::io::File;

pub enum Register {
    AX, AH, AL,
    BX, BH, BL,
    CX, CH, CL,
    DX, DH, DL,
}

pub struct CpuState {
    _state: Vec<u8>,

    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,

    pub ip: u16,
}

impl CpuState {
    pub fn read_from_file() -> CpuState {
        CpuState {
            _state: File::open(&Path::new("test.bin")).read_to_end().unwrap(),
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            ip: 0,
        }
    }

    pub fn addr(&self, i: u16) -> u8 {
        let idx = i.to_uint().unwrap();

        if idx >= self._state.len() {
            return 0xFF
        }
        self._state[idx]
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
        }
    }

    pub fn dump_state(&self) {
        println!("\n*** Begin Processor State Dump ***");
        self.dump_register("ax", AX, AL, AH);
        self.dump_register("bx", BX, BL, BH);
        self.dump_register("cx", CX, CL, CH);
        self.dump_register("dx", DX, DL, DH);
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

    /**
     * Read from the memory location at `ip` and advance `ip`.
     */
    pub fn read(&mut self) -> u8 {
        let byte: u8 = self.addr(self.ip);
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
