use std::io::{self, Write};

use crate::{assembler::{lexer::Lexer, parser::Parser, program::Program}, utils::hex_to_byte_arr, vm::VM};

pub struct REPL {
    vm: VM,
    command_buffer: Vec<String>,
}

impl REPL {
    pub fn new(vm: VM) -> Self {
        Self {
            vm,
            command_buffer: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        println!("[INFO] Entering SPECTRUM");
        loop {
            let mut is_hex_input:bool = false;
            let mut lexer: Lexer = Lexer::new("", "".len());
            let mut parser: Parser = Parser::default();
            let mut program: Program = Program::default();

            let mut buffer: String = String::new();
            print!("[REPL]>> ");
            io::stdout()
                .flush()
                .expect("[REPL]>> [FATAL] Couldn't flush stdout");
            io::stdin()
                .read_line(&mut buffer)
                .expect("[REPL]>> [FATAL] Failed to read user input");
            let buffer: &str = buffer.trim();
            self.command_buffer.push(buffer.into());
            match buffer {
                ".quit" => {
                    println!("[INFO] Shutting down SPECTRUM");
                    std::process::exit(0);
                },
                ".help" => {
                    println!("[REPL]>> .help : list all commands");
                    println!("[REPL]>> .quit : exit current process");
                    println!("[REPL]>> .program : display vm's current bytecode");
                    println!("[REPL]>> .registers : display vm's registers state");
                    println!("[REPL]>> .input_mode : switch input method (between INSTRUCTION and HEX)");
                },
                ".program" => {
                    println!("[REPL]>> {:#?}", self.vm.bytecode)
                },
                ".registers" => {
                    println!("[REPL]>> {:#?}", self.vm.registers)
                },
                ".input_mode" => {
                    if !is_hex_input {
                        is_hex_input = true;
                        println!("[REPL]>> [INFO] Switching input method from INSTRUCTION to HEX");
                    } else {
                        is_hex_input = false;
                        println!("[REPL]>> [INFO] Switching input method from HEX to INSTRUCTION");
                    }
                }
                _ => {
                    if !is_hex_input {
                        lexer.set_content(buffer);
                        lexer.tokens = Vec::new();
                        lexer.tokenize();
                        parser.set_tokens(lexer.tokens);
                        let parsing_result: Vec<_> = parser.parse();
                        program.set_instructions(parsing_result);
                        let program_as_bytes: Vec<u8> = program.as_bytes();
                        for byte in program_as_bytes {
                            self.vm.bytecode.push(byte);
                        }
                        self.vm.run();
                    } else {
                        let parsed_instruction: Result<[u8; 4], _> = hex_to_byte_arr(buffer);
                        match parsed_instruction {
                            Ok(bytes) => {
                                for byte in bytes.iter() {
                                    self.vm.bytecode.push(*byte);
                                }
                                self.vm.run();
                            }
                            Err(_) => {
                                println!("[REPL]>> [WARNING] Failed to parse instruction");
                                println!("[REPL]>> [INFO] Correct format is \'00 00 00 00\'");
                            }
                        }
                    }
                }
            }
        }
    }
}
