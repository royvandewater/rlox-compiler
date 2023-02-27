use crate::op_code::OpCode;

pub struct Chunk {
    code: Vec<u8>,

    constants: Vec<f64>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub(crate) fn add_constant(&mut self, value: f64) -> u8 {
        self.constants.push(value);
        return self.constants.len() as u8 - 1;
    }

    pub(crate) fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub(crate) fn dissassemble(&self, name: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("== {} ==\n", name));

        let mut i = 0;
        while i < self.code.len() {
            let (offset, instruction) = self.disassemble_instruction(i);
            i += offset;
            output.push_str(&format!("{}\n", instruction));
        }

        return output;
    }

    fn disassemble_instruction(&self, offset: usize) -> (usize, String) {
        let opcode_byte = self.code[offset];
        let opcode = opcode_byte.into();

        let line_display = if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            format!("   | ") // show '|' if same line as previous to make it clearer
        } else {
            format!("{:04} ", self.lines[offset])
        };

        let prefix = format!("{:04} {:04}", offset, line_display);

        match opcode {
            OpCode::Constant => {
                let constant = self.code[offset + 1];
                (
                    2,
                    format!(
                        "{} {} {:4} '{}'",
                        prefix, opcode, constant, self.constants[constant as usize]
                    ),
                )
            }
            OpCode::Return => (1, format!("{} {:16}", prefix, opcode)),
            OpCode::Unknown => (1, format!("{} Unknown opcode {}", prefix, opcode)),
        }
    }
}
