use std::{
    fs::File,
    io::{LineWriter, Write},
};

use crate::{
    memory_instruction::MemoryInstruction, memory_segment::MemorySegment,
    parser::arithmetic_instruction::ArithmeticInstruction, utils::get_output_handle,
};

pub struct CodeWriter {
    line_writer: LineWriter<File>,
}

impl CodeWriter {
    pub fn new(filename: &str) -> CodeWriter {
        CodeWriter {
            line_writer: get_output_handle(filename),
        }
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.line_writer
            .flush()
            .expect("Unable to write previous file");
        self.line_writer = get_output_handle(filename);
    }

    pub fn write_arithmetic(&mut self, command: ArithmeticInstruction) {
        let asm_code = match command {
            ArithmeticInstruction::Add => [
                "@SP", "AM=M-1", "D=M", "M=0", "@SP", "AM=M-1", "M=D+M", "@SP", "M=M+1",
            ],
            _ => unreachable!(),
        };

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn write_push_pop(
        &mut self,
        command: MemoryInstruction,
        segment: MemorySegment,
        index: usize,
    ) {
        let formatted_index = format!("@{}", index);
        let formatted_memory_segment = segment.to_string();
        let asm_code = match command {
            MemoryInstruction::Push => match segment {
                MemorySegment::Constant => [
                    formatted_index.as_str(),
                    "D=A",
                    "@SP",
                    "A=M",
                    "M=D",
                    "@SP",
                    "M=M+1",
                ],
                _ => unimplemented!(),
            },
            MemoryInstruction::Pop => unimplemented!(),
        };

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn close(&mut self) {
        write!(self.line_writer, "(END)\n@END\n0;JMP\n").expect("Unable to terminate file");
        self.line_writer.flush().expect("Unable to close file");
    }
}
