use crate::instruction::InstructionType::{
    JumpIfNotZero, JumpIfZero, Left, Minus, Plus, PutChar, ReadChar, Right,
};
use crate::instruction::{Instruction, InstructionType};

pub struct Compiler {
    code: Box<Vec<char>>,
    code_length: usize,
    position: usize,
    pub instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new(code: String) -> Compiler {
        let filtered_code: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
        let code_length = &filtered_code.len();

        Compiler {
            code: Box::new(filtered_code),
            code_length: *code_length,
            position: 0,
            instructions: vec![],
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let mut loopstack: Vec<usize> = vec![];
        let mut last_char: Option<char> = None;
        let valid_chars = "+-><.,[]";

        while self.position < self.code_length {
            let current = self.code[self.position];

            if last_char.is_some() && valid_chars.contains(current) && last_char.unwrap() != ';' {
                return Err(format!("invalid code at position {}", self.position));
            }

            match current {
                '+' => self.compile_foldable_instruction(current, Plus),
                '-' => self.compile_foldable_instruction(current, Minus),
                '>' => self.compile_foldable_instruction(current, Right),
                '<' => self.compile_foldable_instruction(current, Left),
                '.' => self.compile_foldable_instruction(current, PutChar),
                ',' => self.compile_foldable_instruction(current, ReadChar),
                '[' => {
                    let instruction_pos = self.emit_with_arg(JumpIfZero, 0);
                    loopstack.push(instruction_pos);
                }
                ']' => {
                    // pop position of last JumpIfZero ("[") off the stack
                    let open_instruction = match loopstack.pop() {
                        None => panic!("] without opening ["),
                        Some(instruction) => instruction,
                    };

                    // emit the new JumpIfNotZero ("]") instruction,
                    // with correct position argument
                    let close_instruction_pos = self.emit_with_arg(JumpIfNotZero, open_instruction);

                    // Patch the old JumpIfZero with the new position
                    self.instructions[open_instruction].argument = close_instruction_pos;
                }
                _ => {
                    // ignore any other characters
                }
            }

            self.position += 1;
            last_char = Some(current);
        }

        return Ok(());
    }

    fn compile_foldable_instruction(&mut self, the_char: char, instruction_type: InstructionType) {
        let mut count = 1;

        while self.position < self.code_length - 1 && self.code[self.position + 1] == the_char {
            count += 1;
            self.position += 1;
        }

        self.emit_with_arg(instruction_type, count);
    }

    fn emit_with_arg(&mut self, instruction_type: InstructionType, arg: usize) -> usize {
        let instruction = Instruction {
            instruction_type,
            argument: arg,
        };

        self.instructions.push(instruction);

        self.instructions.len() - 1
    }
}
