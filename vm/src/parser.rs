use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
    str::FromStr,
};

use crate::{
    parser::{
        c_instruction::{CCall, CFunction, CGoto, CIf, CLabel, CPop, CPush, CReturn},
        control_instruction::ControlInstruction,
        function_instruction::FunctionInstruction,
        memory_instruction::MemoryInstruction,
        memory_segment::MemorySegment,
    },
    utils::get_input_file,
};

use self::{
    arithmetic_instruction::ArithmeticInstruction,
    c_instruction::{CArithmetic, CInstruction},
};

pub mod arithmetic_instruction;
mod c_instruction;
mod control_instruction;
mod function_instruction;
pub mod memory_instruction;
mod memory_segment;

struct Parser {
    input_lines: Peekable<Lines<BufReader<File>>>,
    curr_line: Option<Vec<String>>,
}

impl Parser {
    pub fn new(file_path: &str) -> Parser {
        Parser {
            input_lines: get_input_file(file_path).lines().peekable(),
            curr_line: None,
        }
    }

    pub fn has_more_commands(&mut self) -> bool {
        self.input_lines.peek().is_some()
    }

    pub fn advance(&mut self) {
        if self.has_more_commands() {
            let mut next_line: String;
            loop {
                next_line = self.input_lines.next().unwrap().unwrap().trim().to_string();

                if !(next_line[0..2].contains("//") || next_line.len() == 0) {
                    break;
                }
            }
            self.curr_line = Some(
                next_line
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }

        self.curr_line = None;
    }

    pub fn command_type(&self) -> CInstruction {
        if let Some(command) = self.get_nth_word(0) {
            if let Ok(a_instr) = ArithmeticInstruction::from_str(&command) {
                return CInstruction::CArithmetic(CArithmetic {
                    command_type: a_instr,
                });
            }

            if let Ok(m_instr) = MemoryInstruction::from_str(&command) {
                let segment = self.get_nth_word(1).unwrap();
                let segment = MemorySegment::from_str(&segment).unwrap();
                let index = self.get_nth_word(2).unwrap().parse::<usize>().unwrap();
                return match m_instr {
                    MemoryInstruction::Pop => CInstruction::CPop(CPop { segment, index }),
                    MemoryInstruction::Push => CInstruction::CPush(CPush { segment, index }),
                };
            }

            if let Ok(ctrl_instr) = ControlInstruction::from_str(&command) {
                let label = self.get_nth_word(1).unwrap();
                return match ctrl_instr {
                    ControlInstruction::Label => CInstruction::CLabel(CLabel { label }),
                    ControlInstruction::Goto => CInstruction::CGoto(CGoto { label }),
                    ControlInstruction::If => CInstruction::CIf(CIf { label }),
                };
            }

            if let Ok(f_instr) = FunctionInstruction::from_str(&command) {
                let function_name = self.get_nth_word(1).unwrap();
                let n_locals = self.get_nth_word(2).unwrap().parse::<usize>().unwrap();
                return match f_instr {
                    FunctionInstruction::Function => CInstruction::CFunction(CFunction {
                        function_name,
                        n_locals,
                    }),
                    FunctionInstruction::Call => CInstruction::CCall(CCall {
                        function_name,
                        n_locals,
                    }),
                    FunctionInstruction::Return => CInstruction::CReturn(CReturn {}),
                };
            }

            panic!("Unsupported instruction")
        } else {
            panic!()
        }
    }

    pub fn arg_1(&mut self) -> Option<String> {
        match self.command_type() {
            CInstruction::CArithmetic(instr) => Some(instr.command_type.to_string()),
            CInstruction::CReturn(_) => None,
            _ => {
                let arg = self.get_nth_word(1).unwrap();
                return Some(arg);
            }
        }
    }

    pub fn arg_2(&self) -> Option<String> {
        match self.command_type() {
            CInstruction::CPush(_)
            | CInstruction::CPop(_)
            | CInstruction::CFunction(_)
            | CInstruction::CCall(_) => {
                let arg = self.get_nth_word(2).unwrap();
                return Some(arg);
            }
            _ => None,
        }
    }

    pub fn get_nth_word(&self, n: usize) -> Option<String> {
        if let Some(instr) = &self.curr_line {
            let command = instr.get(n).unwrap().to_string();
            return Some(command);
        }
        None
    }
}
