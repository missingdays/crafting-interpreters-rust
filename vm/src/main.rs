mod opcode;
mod debug;
mod vm;

fn main() {
    let mut chunk = opcode::Chunk::new();
    let constant_index = chunk.add_constant(13.37);
    chunk.add_byte(opcode::OpCode::Constant.to_byte(), 0);
    chunk.add_byte(constant_index, 1);
    chunk.add_byte(opcode::OpCode::Return.to_byte(), 1);
    vm::VM::new(&chunk).interpret();
}
