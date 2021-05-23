use std::{env, fs::read_dir};

use code_writer::CodeWriter;
use parser::Parser;
use utils::parse_filepath;

use crate::{c_instruction::CInstruction, memory_instruction::MemoryInstruction};

mod c_instruction;
mod code_writer;
mod memory_instruction;
mod memory_segment;
mod parser;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args.iter().map(AsRef::as_ref).collect();
    vm_assembler(args);
}

fn vm_assembler(args: Vec<&str>) {
    let file_path = parse_filepath(args).trim();

    let mut files_to_process: Vec<String> = Vec::new();

    let mut output_file_name: String;

    if file_path.ends_with(".vm") {
        // single file
        files_to_process.push(file_path.to_string());
        output_file_name = file_path.trim_end_matches(".vm").to_string();
    } else {
        let mut file_split = file_path.clone().split("/").collect::<Vec<&str>>();
        file_split.push(file_split.last().unwrap());
        output_file_name = file_split.join("/");
        let paths = read_dir(file_path).unwrap();
        for path in paths {
            if let Ok(dir_entry) = path {
                let path = dir_entry.path().to_str().unwrap().to_string();
                if path.contains(".vm") {
                    files_to_process.push(path);
                }
            }
        }
    }

    output_file_name.push_str(".asm");

    if files_to_process.len() < 1 {
        panic!();
    }

    let mut files_to_process = files_to_process.iter().peekable();
    let mut parser = Parser::new(files_to_process.next().unwrap());
    let mut code_writer = CodeWriter::new(&output_file_name);

    code_writer.write_init();

    loop {
        while parser.has_more_commands() {
            parser.advance();
            let command = parser.command_type();
            match command {
                CInstruction::CArithmetic(a_instr) => {
                    code_writer.write_arithmetic(a_instr.command_type)
                }
                CInstruction::CPop(p_instr) => code_writer.write_push_pop(
                    MemoryInstruction::Pop,
                    p_instr.segment,
                    p_instr.index,
                ),
                CInstruction::CPush(p_instr) => code_writer.write_push_pop(
                    MemoryInstruction::Push,
                    p_instr.segment,
                    p_instr.index,
                ),
                CInstruction::CLabel(c_instr) => code_writer.write_label(c_instr.label),
                CInstruction::CGoto(g_instr) => code_writer.write_goto(g_instr.label),
                CInstruction::CIf(if_instr) => code_writer.write_if(if_instr.label),
                CInstruction::CCall(c_instr) => {
                    code_writer.write_call(c_instr.function_name, c_instr.n_locals)
                }
                CInstruction::CReturn(_) => code_writer.write_return(),
                CInstruction::CFunction(f_instr) => {
                    code_writer.write_function(f_instr.function_name, f_instr.n_locals)
                }
                _ => unimplemented!(),
            };
        }

        if files_to_process.peek().is_none() {
            break;
        }
        parser.set_new_file(files_to_process.next().unwrap())
    }
    code_writer.close();
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use crate::vm_assembler;

    #[test]
    fn run_emulator() {
        let files = vec![
            "./data/SimpleAdd.vm",
            "./data/StackTest.vm",
            "./data/BasicTest.vm",
            "./data/PointerTest.vm",
            "./data/StaticTest.vm",
            "./data/BasicLoop.vm",
            "./data/FibonacciSeries.vm",
            "./data/SimpleFunction.vm",
            "./data/FibonacciElement",
        ];
        for file in files {
            vm_assembler(vec!["", file]);
            let mut file_arg;
            if file.ends_with(".vm") {
                file_arg = file.replace(".vm", ".tst");
            } else {
                let mut file_split = file.clone().split("/").collect::<Vec<&str>>();
                file_split.push(file_split.last().unwrap());
                file_arg = file_split.join("/");
                file_arg.push_str(".tst");
            }
            let mut child = Command::new("./../tools/CPUEmulator.sh")
                .arg(file_arg.as_str())
                .spawn()
                .unwrap();

            let exit_status = child.wait().unwrap();

            assert!(
                exit_status.success(),
                "Test failed: {:?}",
                file_arg.as_str()
            );
        }
    }
}
