use std::vec::Vec;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<f64>,
    lines: Vec<i32>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn add_byte(&mut self, byte: u8, line: i32) -> &Self {
        self.code.push(byte);
        self.lines.push(line);
        self
    }

    pub fn add_constant(&mut self, constant: f64) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }

    pub fn get_constant(&self, index: u8) -> f64 {
        self.constants[index as usize]
    }

    pub fn get_line(&self, offset: usize) -> i32 {
        self.lines[offset]
    }

    pub fn get_byte(&self, offset: usize) -> u8 {
        self.code[offset]
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

impl std::ops::Index<usize> for Chunk {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}


pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Substract,
    Multiply,
    Divide
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Return => write!(f, "OP_RETURN"),
            OpCode::Constant => write!(f, "OP_CONSTANT"),
            OpCode::Negate => write!(f, "OP_NEGATE"),
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Substract => write!(f, "OP_SUBSTRACT"),
            OpCode::Multiply => write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE")
        }
    }
}

impl OpCode {
    pub fn from_byte(byte: u8) -> OpCode {
        match byte {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Substract,
            5 => OpCode::Multiply,
            6 => OpCode::Divide,
            _ => panic!("Can't create opcode from byte {}", byte)
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            OpCode::Return => 0,
            OpCode::Constant => 1,
            OpCode::Negate => 2,
            OpCode::Add => 3,
            OpCode::Substract => 4,
            OpCode::Multiply => 5,
            OpCode::Divide => 6
        }
    }

    pub fn size(&self) -> usize {
        match self {
            OpCode::Return => 1,
            OpCode::Constant => 2,
            OpCode::Negate => 1,
            OpCode::Add => 3,
            OpCode::Substract => 3,
            OpCode::Multiply => 3,
            OpCode::Divide => 3
        }
    }
}