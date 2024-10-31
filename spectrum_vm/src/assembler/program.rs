use super::parser::AssemblyInstruction;

pub struct Program {
    instructions: Vec<AssemblyInstruction>
}

impl Program {
    pub fn default() -> Self {
        Self { instructions: Vec::new() }
    }

    pub fn set_instructions(&mut self, new_instructions: Vec<AssemblyInstruction>) {
        self.instructions = new_instructions;
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut byte_instructions: Vec<u8> = Vec::new();
        for instruction in &self.instructions {
            byte_instructions.append(&mut instruction.as_bytes());
        }
        byte_instructions
    }
}

#[cfg(test)]
mod test {
    use crate::{assembler::{lexer::Lexer, parser::Parser}, vm::VM};

    use super::*;

    #[test]
    fn program_length() {
        let content: &str = "LOAD $1 #500\nLOAD $1 #500\n";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        let mut parser: Parser = Parser::new(lexer.tokens);
        let parsing_result: Vec<_> = parser.parse();
        let program: Program = Program { instructions: parsing_result };
        let program_as_bytes: Vec<u8> = program.as_bytes();
        assert_eq!(program.instructions.len(), 2);
        assert_eq!(program_as_bytes.len(), 8)
    }

    #[test]
    fn endianess() {
        let content: &str = "LOAD $1 #500";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        let mut parser: Parser = Parser::new(lexer.tokens);
        let parsing_result: Vec<_> = parser.parse();
        let program: Program = Program { instructions: parsing_result };
        let program_as_bytes: Vec<u8> = program.as_bytes();
        let mut vm: VM = VM::new();
        vm.bytecode = program_as_bytes;
        vm.run();
        println!("[DEBUG] Vm registers state {:#?}", vm.registers);
        assert_eq!(vm.registers[1], 500)
    }
}
