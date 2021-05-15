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
    jump_count: usize,
}

impl CodeWriter {
    pub fn new(filename: &str) -> CodeWriter {
        CodeWriter {
            line_writer: get_output_handle(filename),
            jump_count: 0,
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
            ArithmeticInstruction::Add => {
                let res: Vec<String> = vec![
                    "@SP", "AM=M-1", "D=M", "M=0", "@SP", "AM=M-1", "M=D+M", "@SP", "M=M+1",
                ]
                .iter()
                .map(|&s| s.into())
                .collect();
                res
            }
            ArithmeticInstruction::Sub => {
                let res: Vec<String> = vec![
                    "@SP", "AM=M-1", "D=M", "M=0", "@R5", "M=D", "@SP", "AM=M-1", "D=M", "@R5",
                    "D=D-M", "@SP", "A=M", "M=D", "@SP", "M=M+1", "@R5", "M=0",
                ]
                .iter()
                .map(|&s| s.into())
                .collect();
                res
            }
            ArithmeticInstruction::Neg => {
                let res = vec!["@32767", "D=A", "@SP", "A=M-1", "M=D-M", "M=M+1"]
                    .iter()
                    .map(|&s| s.into())
                    .collect();
                res
            }
            ArithmeticInstruction::Eq | ArithmeticInstruction::Gt | ArithmeticInstruction::Lt => {
                self.get_comparison_instr(command)
            }
            ArithmeticInstruction::And | ArithmeticInstruction::Or => {
                self.get_boolean_instr(command)
            }
            ArithmeticInstruction::Not => vec!["@SP", "A=M-1", "M=!M"]
                .iter()
                .map(|&s| s.into())
                .collect(),
        };

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    fn get_comparison_instr(&mut self, command: ArithmeticInstruction) -> Vec<String> {
        let jump_instr = match command {
            ArithmeticInstruction::Eq => "D;JEQ",
            ArithmeticInstruction::Gt => "D;JLT",
            ArithmeticInstruction::Lt => "D;JGT",
            _ => unreachable!(),
        };

        let replacement_label = self.jump_count.to_string();

        self.jump_count += 1;

        vec![
            "@RUN_J", "0;JMP", "(TRUE_J)", "@SP", "A=M", "M=-1", "@EQ_J", "0;JMP", "(RUN_J)",
            "@SP", "AM=M-1", "D=M", "M=0", "@SP", " AM=M-1", "D=D-M", "M=0", "@TRUE_J", jump_instr,
            "@SP", "A=M", "M=0", "(EQ_J)", "@SP", "M=M+1",
        ]
        .iter()
        .map(|&s| s.replace("_J", &replacement_label))
        .collect()
    }

    fn get_boolean_instr(&self, command: ArithmeticInstruction) -> Vec<String> {
        let bool_instr = match command {
            ArithmeticInstruction::And => "M=D&M",
            ArithmeticInstruction::Or => "M=D|M",
            _ => unreachable!(),
        };

        vec![
            "@SP", "AM=M-1", "D=M", "M=0", "@SP", "AM=M-1", bool_instr, "@SP", "M=M+1",
        ]
        .iter()
        .map(|&s| s.into())
        .collect()
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
                MemorySegment::Constant => vec![
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
        // write!(self.line_writer, "(END)\n@END\n0;JMP\n").expect("Unable to terminate file");
        self.line_writer.flush().expect("Unable to close file");
    }
}
