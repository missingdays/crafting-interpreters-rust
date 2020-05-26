mod opcode;
mod debug;

fn main() {
    let mut chunk = opcode::Chunk::new();
    chunk.add_opcode(opcode::OpCode::Return);
    debug::disassemble_chunk(&chunk, "Hello world");
}
