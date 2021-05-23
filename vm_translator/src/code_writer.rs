use std::{
    fs::File,
    io::{LineWriter, Write},
    vec,
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
        let asm_code = match command {
            MemoryInstruction::Push => self.get_push_instr(segment, formatted_index.as_str()),
            MemoryInstruction::Pop => self.get_pop_instr(segment, formatted_index.as_str()),
        };

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    fn get_push_instr(&self, segment: MemorySegment, formatted_index: &str) -> Vec<String> {
        let push_instr = match segment {
            MemorySegment::Constant => vec![],
            MemorySegment::Argument => vec!["@ARG", "A=D+M", "D=M"],
            MemorySegment::Local => vec!["@LCL", "A=D+M", "D=M"],
            MemorySegment::Static => vec!["@16", "A=D+A", "D=M"],
            MemorySegment::This => vec!["@THIS", "A=D+M", "D=M"],
            MemorySegment::That => vec!["@THAT", "A=D+M", "D=M"],
            MemorySegment::Pointer => vec!["@3", "A=D+A", "D=M"],
            MemorySegment::Temp => vec!["@5", "A=D+A", "D=M"],
        };

        let mut instr = vec![formatted_index, "D=A"];

        for s in push_instr {
            instr.push(s);
        }
        // instr.push("D=M");
        instr.push("@SP");
        instr.push("A=M");
        instr.push("M=D");
        instr.push("@SP");
        instr.push("M=M+1");

        instr.iter().map(|&s| s.into()).collect()
    }

    fn get_pop_instr(&self, segment: MemorySegment, formatted_index: &str) -> Vec<String> {
        let push_instr = match segment {
            MemorySegment::Argument => vec!["@ARG", "D=D+M"],
            MemorySegment::Local => vec!["@LCL", "D=D+M"],
            MemorySegment::Static => vec!["@16", "D=D+A"],
            MemorySegment::This => vec!["@R3", "D=D+M"],
            MemorySegment::That => vec!["@R4", "D=D+M"],
            MemorySegment::Pointer => vec!["@3", "D=D+A"],
            MemorySegment::Temp => vec!["@5", "D=D+A"],
            _ => unreachable!(),
        };

        let mut instr = vec![formatted_index, "D=A"];

        for s in push_instr {
            instr.push(s);
        }
        instr.push("@R5");
        instr.push("M=D");
        instr.push("@SP");
        instr.push("AM=M-1");
        instr.push("D=M");
        instr.push("@R5");
        instr.push("A=M");
        instr.push("M=D");

        instr.iter().map(|&s| s.into()).collect()
    }

    pub fn write_init(&mut self) {
        let asm_code = vec!["@256", "D=A", "@SP", "M=D"];

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");

        write!(self.line_writer, "{}", asm_code.as_str()).expect("Unable to write line");
    }

    pub fn write_label(&mut self, label: String) {
        let asm_code = format!("({})\n", label);
        write!(self.line_writer, "{}", asm_code.as_str()).expect("Unable to write line");
    }

    pub fn write_goto(&mut self, label: String) {
        let target = format!("@{}\n", label);
        let asm_code = vec![target.as_str(), "0;JMP\n"];

        write!(self.line_writer, "{}", asm_code.join("")).expect("Unable to write line");
    }

    pub fn write_if(&mut self, label: String) {
        let target = format!("@{}\n", label);

        let asm_code = vec!["@SP", "AM=M-1", "D=M", "M=0", target.as_str(), "D;JNE"];
        let mut asm_code = asm_code.join("\n");
        asm_code.push('\n');

        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn write_call(&self, fn_name: String, num_args: usize) {
        unimplemented!()
    }

    pub fn write_return(&mut self) {
        let asm_code = vec![
            "@LCL", "D=M", "@FRAME", "M=D", "@5", "D=D-A", "A=D", "D=M", "@RETURN", "M=D", "@SP",
            "A=M-1", "D=M", "@ARG", "A=M", "M=D", "@ARG", "D=M", "@SP", "M=D+1", "@FRAME",
            "AM=M-1", "D=M", "@THAT", "M=D", "@FRAME", "AM=M-1", "D=M", "@THIS", "M=D", "@FRAME",
            "AM=M-1", "D=M", "@ARG", "M=D", "@FRAME", "AM=M-1", "D=M", "@LCL", "M=D", "@RETURN",
            "A=M", "0;JMP",
        ];

        let mut asm_code = asm_code.join("\n");
        asm_code.push_str("\n");
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn write_function(&mut self, fn_name: String, num_locals: usize) {
        let asm_code = format!("({})\n", fn_name);
        write!(self.line_writer, "{}", asm_code).expect("Unable to write line");
    }

    pub fn close(&mut self) {
        write!(self.line_writer, "(END)\n@END\n0;JMP\n").expect("Unable to terminate file");
        self.line_writer.flush().expect("Unable to close file");
    }
}
