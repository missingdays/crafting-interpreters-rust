use std::fmt;

pub struct Chunk {
    code: std::vec::Vec<OpCode>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
    }

    pub fn add_opcode(&mut self, opcode: OpCode) -> &Self {
        self.code.push(opcode);
        self
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

pub enum OpCode {
    Return
}

impl OpCode {
    pub fn size(&self) -> usize {
        match self {
            OpCode::Return => 1
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Return => write!(f, "{}", "OP_RETURN")
        }
    }
}

impl std::ops::Index<usize> for Chunk {
    type Output = OpCode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}