#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    NEQ,
    GT,
    GEQ,
    LE,
    LEQ,
    JEQ,
    JNEQ,
    JMP,
    JMPF,
    JMPB,
    INC,
    DEC,
    ALOC,
    RSHT,
    LFST,
    RROR,
    LROR,
    NOP,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::EQ,
            7 => Opcode::NEQ,
            8 => Opcode::GT,
            9 => Opcode::GEQ,
            10 => Opcode::LE,
            11 => Opcode::LEQ,
            12 => Opcode::JEQ,
            13 => Opcode::JNEQ,
            14 => Opcode::JMP,
            15 => Opcode::JMPF,
            16 => Opcode::JMPB,
            17 => Opcode::INC,
            18 => Opcode::DEC,
            19 => Opcode::ALOC,
            _ => Opcode::NOP,
        }
    }
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match value {
            "HLT" => Opcode::HLT,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "GEQ" => Opcode::GEQ,
            "LE" => Opcode::LE,
            "LEQ" => Opcode::LEQ,
            "JEQ" => Opcode::JEQ,
            "JNEQ" => Opcode::JNEQ,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "INC" => Opcode::INC,
            "DEC" => Opcode::DEC,
            "ALOC" => Opcode::ALOC,
            _ => Opcode::NOP,
        }
    }
}
