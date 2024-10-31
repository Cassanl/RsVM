use crate::instruction::Opcode;

use super::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub struct AssemblyInstruction {
    opcode: Token,
    operand_1: Option<Token>,
    operand_2: Option<Token>,
    operand_3: Option<Token>,
}

impl AssemblyInstruction {
    pub fn new(
        opcode: Token,
        operand_1: Option<Token>,
        operand_2: Option<Token>,
        operand_3: Option<Token>,
    ) -> Self {
        Self {
            opcode,
            operand_1,
            operand_2,
            operand_3,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut instruction_as_bytes: Vec<u8> = Vec::new();
        match self.opcode.token_kind {
            TokenKind::Operation { code } => {
                instruction_as_bytes.push(code as u8)
            },
            _ => {
                println!("[FATAL] Cannot put a non-opcode token in opcode position");
                std::process::exit(-1);
            }
        }

        for token in &[&self.operand_1, &self.operand_1, &self.operand_1] {
            match token {
                Some(t) => {
                    // extract Operand
                    match &t.token_kind {
                        TokenKind::IntegerOperand { value } => {
                            // can take up to 16 bits
                            let buffer: u16 =  *value as u16;
                            // 11111111 11111111
                            // byte_1 ^
                            // byte_2 >> 8      ^
                            let byte_1: u16 = buffer;                // 8 upper bits
                            let byte_2: u16 = buffer >> 8;        // 8 lower bits
                            // byte is MSB so we put it first to respect endianness eg.: 0x01 0x02 in bytecode:
                            instruction_as_bytes.push(byte_2 as u8);
                            instruction_as_bytes.push(byte_1 as u8);
                        }
                        TokenKind::Register { reg_index } => {
                            instruction_as_bytes.push(*reg_index as u8);
                        }
                        _ => {
                            println!("[FATAL] Only Register and IntegerOperand token kinds are accepted as operand");
                            std::process::exit(-1);
                        }
                    }
                },
                None => {} // No token => no byte pushed
            }
        }
        instruction_as_bytes
    }
}

pub struct Parser {
    tokens_to_parse: Vec<Token>,
}

impl Parser {
    pub fn new(tokens_to_parse: Vec<Token>) -> Self {
        Self { tokens_to_parse }
    }

    pub fn default() -> Self {
        Self { tokens_to_parse: Vec::new() }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens_to_parse = tokens;
    }

    /// TODO avoid .unwrap().clone() as its not safe nor memory efficient
    pub fn parse(&mut self) -> Vec<AssemblyInstruction> {
        let mut parsed_instructions: Vec<AssemblyInstruction> = Vec::new();
        let mut iterator = self.tokens_to_parse.iter();
        while let Some(t) = iterator.next() {
            match &t.token_kind {
                TokenKind::Operation { code } => {
                    match code {
                        Opcode::LOAD => {
                            parsed_instructions.push(AssemblyInstruction::new(
                                t.clone(),
                                Some(iterator.next().unwrap().clone()),
                                Some(iterator.next().unwrap().clone()),
                                None,
                            ));
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        }
        parsed_instructions
    }

    fn handle_parsing_error(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::assembler::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_load() {
        let content: &str = "LOAD $1 #500\n";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        let mut parser: Parser = Parser::new(lexer.tokens);
        let parsing_result: Vec<_> = parser.parse();
        assert_eq!(
            parsing_result
                .get(0)
                .unwrap()
                .opcode
                .token_kind,
            TokenKind::Operation { code: Opcode::LOAD }
        )
    }
}
