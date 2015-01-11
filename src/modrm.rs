/**
 * ModrmResult enum
 *
 * Represents some information inferred from a ModR/M byte, such as an
 * effective address spec or register.
 */
#[allow(non_camel_case_types)]
pub enum MemoryAddr {
    BX_SI,
    BX_DI,
    BP_SI,
    BP_DI,
    SI,
    DI,
    BX,
}
pub enum Register {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

pub enum ModrmResult {
    // Result is a memory address like [BX+SI] or [DI]
    MemoryAddr(MemoryAddr),

    // Result is a [DISP16] memory address
    MemoryDisp16(u16),

    // Result is a general register, e.g. AX or DI
    Register(Register),
}

impl ModrmResult {
    pub fn unwrap_addr(&self) -> &MemoryAddr {
        match *self {
            ModrmResult::MemoryAddr(ref x) => x,
            _ => panic!("ModrmResult::unwrap_addr: not MemoryAddr"),
        }
    }

    pub fn unwrap_disp16(&self) -> &u16 {
        match *self {
            ModrmResult::MemoryDisp16(ref x) => x,
            _ => panic!("ModrmResult::unwrap_addr: not MemoryDisp16"),
        }
    }

    pub fn unwrap_register(&self) -> &Register {
        match *self {
            ModrmResult::Register(ref x) => x,
            _ => panic!("ModrmResult::unwrap_register: not Register"),
        }
    }
}

/**
 * ModrmByte struct
 *
 * Represents the literal ModR/M byte and defines facilities to extract
 * ModrmResults from the byte.
 */
pub struct ModrmByte {
    pub m: u8,
    pub reg: u8,
    pub rm: u8,

    pub peek: u16,
}

impl ModrmByte {
    pub fn read(byte: u8) -> ModrmByte {
        // Extract `mod'
        let modbits = byte & 0b11000000;
        let modbits = modbits / 64;

        // Extract `reg'
        let reg = byte & 0b00111000;
        let reg = reg / 8;

        // Extract `r/m'
        let rm = byte & 0b00000111;

        ModrmByte {
            m: modbits,
            reg: reg,
            rm: rm,
            peek: 0,
        }
    }

    fn parse_register(&self, reg: u8) -> Register {
        match reg {
            0b000 => Register::AX,
            0b001 => Register::CX,
            0b010 => Register::DX,
            0b011 => Register::BX,
            0b100 => Register::SP,
            0b101 => Register::BP,
            0b110 => Register::SI,
            0b111 => Register::DI,
            _ => panic!("Invalid ModRM byte"),
        }
    }

    pub fn effective(&self) -> ModrmResult {
        match self.m {
            0b00 => match self.rm {
                0b000 => ModrmResult::MemoryAddr(MemoryAddr::BX_SI),
                0b001 => ModrmResult::MemoryAddr(MemoryAddr::BX_DI),
                0b010 => ModrmResult::MemoryAddr(MemoryAddr::BP_SI),
                0b011 => ModrmResult::MemoryAddr(MemoryAddr::BP_DI),
                0b100 => ModrmResult::MemoryAddr(MemoryAddr::SI),
                0b101 => ModrmResult::MemoryAddr(MemoryAddr::DI),
                0b110 => ModrmResult::MemoryDisp16(self.peek),
                0b111 => ModrmResult::MemoryAddr(MemoryAddr::BX),
                _ => panic!("Invalid ModRM byte"),
            },
            0b11 => ModrmResult::Register(self.parse_register(self.rm)),
            _ => panic!("Not Implemented"),
        }
    }

    pub fn register(&self) -> ModrmResult {
        ModrmResult::Register(self.parse_register(self.reg))
    }

    /**
     * Returns true if a peek WORD is required. I.e. if either contained
     * ModrmValue is a [disp16].
     */
    pub fn need_peek(&self) -> bool {
        let eff = match self.effective() {
            ModrmResult::MemoryDisp16(_) => true,
            _ => false,
        };
        let reg = match self.register() {
            ModrmResult::MemoryDisp16(_) => true,
            _ => false,
        };
        eff || reg
    }

    pub fn set_peek(&mut self, peek: u16) {
        self.peek = peek;
    }
}
