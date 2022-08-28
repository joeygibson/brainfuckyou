use std::io::{BufReader, BufWriter, Read, Write};

use crate::instruction::{Instruction, InstructionType};

pub struct Machine<'a> {
    code: Vec<Instruction>,
    ip: usize,
    memory: [u32; 30000],
    dp: usize,
    input: &'a mut BufReader<Box<dyn Read>>,
    output: &'a mut BufWriter<Box<dyn Write>>,
    buf: [u8; 1],
}

impl<'a> Machine<'a> {
    pub fn new(
        code: Vec<Instruction>,
        input: &'a mut BufReader<Box<dyn Read>>,
        output: &'a mut BufWriter<Box<dyn Write>>,
    ) -> Self {
        Machine {
            code,
            ip: 0,
            memory: [0; 30000],
            dp: 0,
            input,
            output,
            buf: [0; 1],
        }
    }

    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            let instruction = &self.code[self.ip];

            match instruction.instruction_type {
                InstructionType::Plus => self.memory[self.dp] += instruction.argument as u32,
                InstructionType::Minus => self.memory[self.dp] -= instruction.argument as u32,
                InstructionType::Right => self.dp += instruction.argument,
                InstructionType::Left => self.dp -= instruction.argument,
                InstructionType::PutChar => {
                    for _ in 0..instruction.argument {
                        self.put_char()
                    }
                }
                InstructionType::ReadChar => {
                    for _ in 0..instruction.argument {
                        self.read_char()
                    }
                }
                InstructionType::JumpIfZero => {
                    if self.memory[self.dp] == 0 {
                        self.ip = instruction.argument;
                        continue;
                    }
                }
                InstructionType::JumpIfNotZero => {
                    if self.memory[self.dp] != 0 {
                        self.ip = instruction.argument;
                        continue;
                    }
                }
            }

            self.ip += 1;
        }
    }

    pub fn read_char(&mut self) {
        let bytes_read = match self.input.read(&mut self.buf) {
            Ok(bytes_read) => bytes_read,
            Err(e) => panic!(e),
        };

        if bytes_read != 1 {
            panic!("wrong number of bytes read");
        }

        self.memory[self.dp] = self.buf[0] as u32;
    }

    pub fn put_char(&mut self) {
        self.buf[0] = self.memory[self.dp] as u8;

        let bytes_written = match self.output.write(&self.buf) {
            Ok(bytes_written) => bytes_written,
            Err(e) => panic!(e),
        };

        if bytes_written != 1 {
            panic!("wrong number of bytes written")
        }

        self.output.flush();
    }
}
