use std::{
    fs::File,
    io::{LineWriter, Write},
};

use crate::{
    parser::{
        arithmetic_instruction::ArithmeticInstruction, memory_instruction::MemoryInstruction,
    },
    utils::write_output_file,
};

struct CodeWriter {
    line_writer: LineWriter<File>,
}

impl CodeWriter {
    pub fn new(filename: &str) -> CodeWriter {
        CodeWriter {
            line_writer: write_output_file(filename),
        }
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.line_writer
            .flush()
            .expect("Unable to write previous file");
        self.line_writer = write_output_file(filename);
    }

    pub fn write_arithmetic(&mut self, command: ArithmeticInstruction) {
        let asm_code = match command {
            ArithmeticInstruction::Add => [
                "@SP", "AM=M-1", "D=M", "M=0", "@SP", "AM=M-1", "M=D+M", "@SP", "M=M+1",
            ],
            _ => unreachable!(),
        };

        let asm_code = asm_code
            .iter()
            .map(|curr| format!("{}\n", curr))
            .collect::<Vec<String>>()
            .join("");

        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn write_push_pop(&mut self, command: MemoryInstruction) {
        let asm_code = match command {
            _ => "blah",
        };
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn close(&mut self) {
        self.line_writer.flush().expect("Unable to close file");
    }
}
