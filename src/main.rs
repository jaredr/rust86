#![allow(unstable)]
use std::os;
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
    let argv = os::args();
    if argv.len() < 2 {
        println!("Usage: {} <filename>", argv[0]);
        return;
    }
    let path = Path::new(&argv[1]);

    let mut cs = cstate::CpuState::new();
    cs.load_program(&path);

    loop {
        let opcode = cs.read_b();
        opcodes::do_opcode(&mut cs, opcode);
    }
}
