use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pub bytecode: Vec<u8>,
    pub stack: [u8; 1024],
    pub heap: Vec<u8>,
    pub program_counter: usize,
    pub div_remainder: u32,
    pub eq_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            bytecode: Vec::new(),
            stack: [0; 1024],
            heap: Vec::new(),
            program_counter: 0,
            div_remainder: 0,
            eq_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_running = true;
        while is_running {
            is_running = self.execute_bytecode()
        }
    }

    fn execute_bytecode(&mut self) -> bool {
        if self.program_counter >= self.bytecode.len() {
            return false;
        }

        match self.get_instruction_from_bytecode() {
            Opcode::LOAD => {
                let register: usize = self.get_next_8_bits() as usize;
                let value: usize = self.get_next_16_bits() as usize;
                self.registers[register as usize] = value as i32;
            }
            Opcode::ADD => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                let register: usize = self.get_next_8_bits() as usize;
                self.registers[register] = operand_1 + operand_2;
            }
            Opcode::SUB => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                let register: usize = self.get_next_8_bits() as usize;
                self.registers[register] = operand_1 - operand_2;
            }
            Opcode::MUL => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                let register: usize = self.get_next_8_bits() as usize;
                self.registers[register] = operand_1 * operand_2;
            }
            Opcode::DIV => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                let register: usize = self.get_next_8_bits() as usize;
                match operand_2 {
                    0 => return false,
                    _ => {
                        self.registers[register] = operand_1 / operand_2;
                        self.div_remainder = (operand_1 % operand_2) as u32;
                    }
                }
            }
            Opcode::JMP => {
                self.program_counter = self.registers[self.get_next_8_bits() as usize] as usize;
            }
            Opcode::JMPF => {
                self.program_counter += self.registers[self.get_next_8_bits() as usize] as usize;
            }
            Opcode::JMPB => {
                self.program_counter -= self.registers[self.get_next_8_bits() as usize] as usize;
            }
            Opcode::JEQ => {
                if let true = self.eq_flag {
                    self.program_counter = self.registers[self.get_next_8_bits() as usize] as usize;
                }
            }
            Opcode::JNEQ => {
                if let false = self.eq_flag {
                    self.program_counter = self.registers[self.get_next_8_bits() as usize] as usize;
                }
            }
            Opcode::EQ => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 == operand_2;
                self.skip_next_8_bits();
            }
            Opcode::NEQ => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 != operand_2;
                self.skip_next_8_bits();
            }
            Opcode::GT => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 > operand_2;
                self.skip_next_8_bits();
            }
            Opcode::GEQ => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 >= operand_2;
                self.skip_next_8_bits();
            }
            Opcode::LE => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 < operand_2;
                self.skip_next_8_bits();
            }
            Opcode::LEQ => {
                let operand_1: i32 = self.registers[self.get_next_8_bits() as usize];
                let operand_2: i32 = self.registers[self.get_next_8_bits() as usize];
                self.eq_flag = operand_1 <= operand_2;
                self.skip_next_8_bits();
            }
            Opcode::INC => {
                self.registers[self.get_next_8_bits() as usize] += 1;
                self.skip_next_16_bits();
            }
            Opcode::DEC => {
                self.registers[self.get_next_8_bits() as usize] -= 1;
                self.skip_next_16_bits();
            }
            Opcode::ALOC => {
                let value: usize = self.registers[self.get_next_8_bits() as usize] as usize;
                self.heap.resize(value, 0);
            }
            Opcode::HLT => return false,
            Opcode::NOP => return false,
            _ => return false,
        }
        // TODO program_counter % 16 == 0 because opcodes are every 4 bytes
        // so pc must be a multiple of four to land on an opcode
        // bytecode[0] 01 00 00 00 02 00 00 00 03 00 00 00
        //                    ^^ second opcode is at bytecode[4] (then bytecode[8] ...)
        true
    }

    fn get_instruction_from_bytecode(&mut self) -> Opcode {
        let instruction = Opcode::from(self.bytecode[self.program_counter]);
        self.program_counter += 1;
        instruction
    }

    fn get_next_8_bits(&mut self) -> u8 {
        let result = self.bytecode[self.program_counter];
        self.program_counter += 1;
        result
    }

    fn get_next_16_bits(&mut self) -> u16 {
        let result: u16 = ((self.bytecode[self.program_counter] as u16) << 8)
            | self.bytecode[self.program_counter + 1] as u16;
        self.program_counter += 2;
        result
    }

    fn skip_next_8_bits(&mut self) {
        self.program_counter += 1;
    }

    fn skip_next_16_bits(&mut self) {
        self.program_counter += 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load() {
        let mut vm = VM::new();
        vm.bytecode = vec![1, 1, 1, 244];
        vm.run();
        assert_eq!(vm.registers[1], 500)
    }

    #[test]
    fn add() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 6;
        vm.bytecode = vec![2, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 12)
    }

    #[test]
    fn sub() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.registers[1] = 4;
        vm.bytecode = vec![3, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 1)
    }

    #[test]
    fn mul() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.registers[1] = 2;
        vm.bytecode = vec![4, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 10)
    }

    #[test]
    fn div() {
        let mut vm = VM::new();
        vm.registers[0] = 10;
        vm.registers[1] = 3;
        vm.bytecode = vec![5, 0, 1, 2];
        vm.run();
        assert_eq!(vm.div_remainder, 1);
        assert_eq!(vm.registers[2], 3)
    }

    #[test]
    fn div_by_0() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 0;
        vm.bytecode = vec![5, 0, 1, 2];
        vm.run();
        assert_eq!(vm.eq_flag, false)
    }

    #[test]
    fn jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.bytecode = vec![14, 0, 0, 0];
        vm.run();
        assert_eq!(vm.program_counter, 5)
    }

    #[test]
    fn jmpf() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.bytecode = vec![15, 0, 0, 0];
        vm.run();
        assert_eq!(vm.program_counter, 7)
    }

    #[test]
    fn jeq() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.eq_flag = true;
        vm.bytecode = vec![12, 0, 0, 0];
        vm.run();
        assert_eq!(vm.program_counter, 5)
    }
}
