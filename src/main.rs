mod chunk;
mod op_code;

use chunk::Chunk;
use op_code::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant, 123);

    chunk.write(OpCode::Return.into(), 123);

    println!("{}", chunk.dissassemble("test chunk"));
}
