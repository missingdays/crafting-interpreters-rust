use super::opcode;
use std::cell::Cell;

pub struct VM<'a> {
    chunk: &'a opcode::Chunk,
    ip: Cell<usize>
}

impl <'a> VM<'a> {
    pub fn new(chunk: &'a opcode::Chunk) -> VM {
        VM { chunk: chunk, ip: Cell::new(0) }
    }

    pub fn interpret(&self) -> InterpretResult {
        loop {
            let byte = self.next_byte();
            let opcode = opcode::OpCode::from_byte(byte);
            match opcode {
                opcode::OpCode::Return => return InterpretResult::Ok,
                opcode::OpCode::Constant => {
                    let constant_index = self.read_constant();
                    println!("{}", self.chunk.get_constant(constant_index));
                }
            }
        }
    }

    fn next_byte(&self) -> u8 {
        let ip = self.ip.get();
        let byte = self.chunk.get_byte(ip);
        self.ip.set(ip + 1);
        byte
    }

    fn read_constant(&self) -> u8 {
        let byte = self.next_byte();
        byte
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}