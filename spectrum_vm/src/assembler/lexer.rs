use std::str::Chars;

use crate::instruction::Opcode;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Operation { code: Opcode },
    Register { reg_index: usize },
    IntegerOperand { value: i32 },
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
    start: usize,
    end: usize,
    length: usize,
}

impl Token {
    pub fn new(token_kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            token_kind,
            start,
            end,
            length: end - start,
        }
    }
}

pub struct Lexer<'a> {
    content: &'a str,
    content_len: usize,
    iterator: Chars<'a>,
    line: usize,
    start_of_line: usize,
    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str, content_len: usize) -> Self {
        Self {
            content,
            content_len,
            iterator: content.chars(),
            line: 0,
            start_of_line: 0,
            tokens: Vec::new(),
        }
    }

    pub fn set_content(&mut self, new_content: &'a str) {
        self.content_len = new_content.len();
        self.content = new_content;
        self.iterator = new_content.chars();
    }

    pub fn tokenize(&mut self) {
        loop {
            let token: Token = self.next_token();
            match token.token_kind {
                TokenKind::Eof => {
                    self.tokens.push(token);
                    break;
                }
                _ => self.tokens.push(token),
            }
        }
    }

    fn match_kind(&mut self) -> TokenKind {
        while let Some(c) = self.iterator.next() {
            match c {
                '#' => {
                    // NEEDS WORK
                    let start: usize = self.offset();
                    loop {
                        let char_buffer = self.peek();
                        match char_buffer {
                            Some(cc) => {
                                if cc == ' ' || cc == '\n' || self.is_at_end() {
                                    let value: &str = &self.content[start..self.offset()];
                                    let value: Result<i32, _> = i32::from_str_radix(value, 10);
                                    match value {
                                        Ok(val) => return TokenKind::IntegerOperand { value: val },
                                        Err(_err) => {
                                            self.handle_lexical_error(
                                                "failed to tokenize integer operand",
                                                self.offset() - self.start_of_line,
                                            );
                                            return TokenKind::Eof;
                                        }
                                    }
                                } else {
                                    self.iterator.next();
                                }
                            }
                            None => {
                                let value: &str = &self.content[start..self.offset()];
                                let value: i32 = i32::from_str_radix(value, 10).unwrap();
                                return TokenKind::IntegerOperand { value };
                            }
                        }
                    }
                    // NEEDS WORK
                }
                '$' => {
                    // NEEDS WORK
                    let start: usize = self.offset();
                    loop {
                        let char_buffer = self.peek();
                        match char_buffer {
                            Some(cc) => {
                                if cc == ' ' || cc == '\n' || self.is_at_end() {
                                    let value: &str = &self.content[start..self.offset()];
                                    let value: Result<usize, _> = usize::from_str_radix(value, 10);
                                    return match value {
                                        Ok(val) => TokenKind::Register { reg_index: val },
                                        Err(_err) => {
                                            self.handle_lexical_error(
                                                "failed to tokenize register index",
                                                self.offset() - self.start_of_line,
                                            );
                                            TokenKind::Eof
                                        }
                                    }
                                } else {
                                    self.iterator.next();
                                }
                            }
                            None => {
                                let value: &str = &self.content[start..self.offset()];
                                let value: usize = usize::from_str_radix(value, 10).unwrap();
                                return TokenKind::Register { reg_index: value };
                            }
                        }
                    }
                    // NEEDS WORK
                }
                ' ' => {}
                '\n' => {
                    self.line += 1;
                    self.start_of_line = self.offset();
                }
                _ => {
                    // NEEDS WORK
                    let start: usize = self.offset();
                    loop {
                        let char_buffer = self.peek();
                        match char_buffer {
                            Some(cc) => {
                                if cc == ' ' || cc == '\n' || self.is_at_end() {
                                    let value: &str = &self.content[start - 1..self.offset()];
                                    let value: Opcode = Opcode::from(value);
                                    match value {
                                        Opcode::NOP => {
                                            self.handle_lexical_error(
                                                "failed to tokenize opcode literal",
                                                self.offset() - self.start_of_line,
                                            );
                                            return TokenKind::Eof;
                                        }
                                        _ => return TokenKind::Operation { code: value },
                                    }
                                } else {
                                    self.iterator.next();
                                }
                            }
                            None => {
                                let value: &str = &self.content[start - 1..self.offset()];
                                let value: Opcode = Opcode::from(value);
                                return TokenKind::Operation { code: value };
                            }
                        }
                    }
                    // NEEDS WORK
                }
            }
        }
        TokenKind::Eof
    }

    fn next_token(&mut self) -> Token {
        let start: usize = self.offset();
        let token_kind: TokenKind = self.match_kind();
        let end: usize = self.offset();
        Token::new(token_kind, start, end)
    }

    /// does not return a ASCII encoded value (0-255) but an utf8 one (0 - 0x10FFFF)
    /// clone on the iterator only copies tracking and boundary index
    fn peek(&mut self) -> Option<char> {
        self.iterator.clone().next()
    }

    /// the offset is the difference between the total nb of chars and the remaining number of char
    fn offset(&self) -> usize {
        self.content.len() - self.iterator.as_str().len()
    }

    fn is_at_end(&self) -> bool {
        self.offset() >= self.content.len()
    }

    fn handle_lexical_error(&self, msg_buffer: &str, start_of_line: usize) {
        let buffer: String = format!(
            "[ERROR] Lexical error : {} at {}:{}",
            msg_buffer, self.line, start_of_line
        );
        println!("{}", buffer.as_str());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn opcode_token() {
        let content: &str = "LOAD $1 #500";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        assert_eq!(
            lexer.tokens.get(0).unwrap().token_kind,
            TokenKind::Operation { code: Opcode::LOAD }
        )
    }

    #[test]
    fn register_token() {
        let content: &str = "LOAD $1 #500";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        assert_eq!(
            lexer.tokens.get(1).unwrap().token_kind,
            TokenKind::Register { reg_index: 1 }
        )
    }

    #[test]
    fn integer_operand_token() {
        let content: &str = "LOAD $1 #500";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        assert_eq!(
            lexer.tokens.get(2).unwrap().token_kind,
            TokenKind::IntegerOperand { value: 500 }
        )
    }

    #[test]
    fn multiple_lines() {
        let content: &str = "LOAD $0 #500\nADD $0 $1 $2\nDIV $0 $1 $2";
        let mut lexer: Lexer = Lexer::new(content, content.len());
        lexer.tokenize();
        assert_eq!(
            lexer.tokens.get(5).unwrap().token_kind,
            TokenKind::Register { reg_index: 1 }
        );
        assert_eq!(
            lexer.tokens.get(3).unwrap().token_kind,
            TokenKind::Operation { code: Opcode::ADD }
        );
        assert_eq!(
            lexer.tokens.get(7).unwrap().token_kind,
            TokenKind::Operation { code: Opcode::DIV }
        );
    }
}
