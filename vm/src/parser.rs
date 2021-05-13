use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
    iter::Peekable,
    str::FromStr,
};

use crate::parser::memory_segment::MemorySegment;

use self::{
    arithmetic_instruction::ArithmeticInstruction,
    c_instruction::{CArithmetic, CInstruction},
};

mod arithmetic_instruction;
mod c_instruction;
mod memory_segment;

// TODO: refactor into utils
pub fn get_input_file(file_path: &str) -> BufReader<File> {
    let input_file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(input_file);
    reader
}

// TODO: refactor into utils
pub fn parse_filepath(args: Vec<&str>) -> &str {
    if args.len() != 2 {
        panic!("Usage: cargo run <.vm file or filepath>");
    }
    args[1]
}

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
            let next_line = self.input_lines.next().unwrap().unwrap();
            self.curr_line = Some(
                next_line
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }

        self.curr_line = None;
    }

    // TODO: check first word in string
    pub fn command_type(&self) -> CInstruction {
        if let Some(instruction) = &self.curr_line {
            let command = instruction.first().unwrap().to_string();

            if let Ok(a_instr) = ArithmeticInstruction::from_str(&command) {
                return CInstruction::CArithmetic(CArithmetic {
                    command_type: a_instr,
                });
            }

            // Handle other type of instructions
            panic!()
        } else {
            panic!()
        }
    }

    // TODO: check for 2nd word in string
    pub fn arg_1(&mut self) -> Result<String, Error> {
        // TODO: refactor into get_nth_entry()
        if let Some(instruction) = &self.curr_line {
            let command = instruction.iter().nth(2).unwrap().to_string();
        }
        match self.command_type() {
            CInstruction::CArithmetic(instr) => Ok(instr.command_type.to_string()),
            CInstruction::CReturn(_) => unreachable!(),
            _ => unimplemented!(), // returns actual first command, prolly checks the line stream
        }
    }

    // TODO: check for third word in string
    pub fn arg_2(&self) -> Option<&str> {
        match self.command_type() {
            CInstruction::CPush(instr) => Some("x"),
            CInstruction::CPop(instr) => Some("x"),
            CInstruction::CFunction(instr) => Some("x"),
            CInstruction::CCall(instr) => Some("x"),
            _ => unimplemented!(),
        }
    }
}

enum MemoryInstructions {
    Push,
    Pop,
}

enum ControlInstructions {
    Label,
    Goto,
    If,
}

enum FunctionInstruction {
    Function,
    Call,
    Return,
}
