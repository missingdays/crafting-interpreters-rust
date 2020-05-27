use super::opcode;

pub struct VM<'a> {
    chunk: &'a opcode::Chunk,
    ip: usize,
    stack: Vec<f64>
}

impl <'a> VM<'a> {
    pub fn new(chunk: &'a opcode::Chunk) -> VM {
        VM { chunk: chunk, ip: 0, stack: Vec::new() }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            let byte = self.next_byte();
            let opcode = opcode::OpCode::from_byte(byte);
            match opcode {
                opcode::OpCode::Return => {
                    let value = self.pop();
                    println!("{}", value);
                    return InterpretResult::Ok
                },

                opcode::OpCode::Constant => {
                    let constant_index = self.read_constant();
                    let constant = self.chunk.get_constant(constant_index);
                    self.push(constant);
                }
            }
        }
    }

    fn next_byte(&mut self) -> u8 {
        let ip = self.ip;
        let byte = self.chunk.get_byte(ip);
        self.ip = ip + 1;
        byte
    }

    fn read_constant(&mut self) -> u8 {
        let byte = self.next_byte();
        byte
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}