extern crate num;

use std::env;
use std::path::Path;
mod byteutils;
mod cstate;
mod datatypes;
mod debugger;
mod opcodes;
mod operation;
mod operand;
mod modrm;
mod specialops;
mod tf;


fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 2 {
        println!("Usage: {} <filename>", argv[0]);
        return;
    }
    let path = Path::new(&argv[1]);

    let mut cs = cstate::CpuState::new();
    cs.load_program(&path);

    loop {
        let opcode = cs.read();
        if opcode == 0xF4 {
            debugger::dump_state(&cs);
            debugger::dump_vram(&cs);
            return;
        }
        opcodes::do_opcode(&mut cs, opcode);
    }
}
