use super::opcode;

pub fn disassemble_chunk(chunk: &opcode::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.len() {
        offset = disassemble_intruction(chunk, offset)
    }
}

fn disassemble_intruction(chunk: &opcode::Chunk, offset: usize) -> usize {
    let byte = &chunk[offset];
    let opcode = opcode::OpCode::from_byte(*byte);

    print!("{:04} ", offset);

    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("  | ");
    } else {
        print!("{:03} ", chunk.get_line(offset));
    }

    match opcode {
        opcode::OpCode::Constant => {
            let constant_index = chunk[offset + 1];
            let constant_value = chunk.get_constant(constant_index);
            print!("{} {}", opcode, constant_value);
        },

        _ => {
            print!("{}", opcode);
        }
    }
    println!("");

    offset + opcode.size()
}
