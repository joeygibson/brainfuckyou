#[derive(Debug)]
pub enum InstructionType {
    Plus,
    Minus,
    Right,
    Left,
    PutChar,
    ReadChar,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub argument: usize,
}
