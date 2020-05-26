use super::opcode;

pub fn disassemble_chunk(chunk: &opcode::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.len() {
        offset = disassemble_intruction(chunk, offset)
    }
}

fn disassemble_intruction(chunk: &opcode::Chunk, offset: usize) -> usize {
    let opcode = &chunk[offset];
    println!("{:04} {}", offset, opcode);
    offset + opcode.size()
}