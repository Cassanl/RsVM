use std::env;

use crate::{repl::cli::REPL, vm::VM};

mod assembler;
mod repl;

pub mod instruction;
pub mod utils;
pub mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[INFO] Starting REPL");
        let vm: VM = VM::new();
        let mut cli: REPL = REPL::new(vm);
        cli.run();
    } else {
        println!("[INFO] running file on Spectrum vm")
    }
}
