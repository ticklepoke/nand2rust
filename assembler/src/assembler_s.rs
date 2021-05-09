use std::io::{BufRead, Write};

use crate::{
    instructions::{get_comp_instr, get_dest_instr, get_jump_instr},
    symbols::SymbolTable,
    utils::{get_input_file, split_filename, write_output_file},
};

/**
* Assembler module with support for symbols
*/
pub fn assemble_s(full_file_path: &str) {
    let filename_without_extension = split_filename(full_file_path);

    let reader = get_input_file(full_file_path);

    let mut output_file = write_output_file(filename_without_extension.as_str());

    let mut symbol_table = SymbolTable::new();

    let mut line_count = 0;

    let mut variable_count = 16;

    // First pass, scan out jump labels and @variables
    for line in reader.lines() {
        let line = line.expect("Unable to read line!").trim().to_string();

        if line.len() == 0 || line.starts_with("//") {
            continue;
        } else if line.starts_with("(") {
            let label = line.split(")").collect::<Vec<&str>>()[0];

            let mut label = label.chars();
            label.next();

            if !symbol_table.contains(label.as_str()) {
                symbol_table
                    .add_entity(label.as_str(), line_count)
                    .expect("Unable to insert symbol")
            }
            continue;
        }
        line_count += 1;
    }

    let reader = get_input_file(full_file_path);

    for line in reader.lines() {
        let line = line.expect("Unable to read line!").trim().to_string();

        if line.len() == 0 {
            continue;
        }

        if line.starts_with("@") {
            // insert into symbol table
            let a_value = line.split("//").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();

            let mut a_value = a_value.chars();
            a_value.next();
            if let Ok(_) = a_value.as_str().parse::<u32>() {
                // noop
            } else {
                if !symbol_table.contains(a_value.as_str()) {
                    symbol_table
                        .add_entity(a_value.as_str(), variable_count)
                        .expect("Unable to insert symbol");

                    variable_count += 1;
                }
            }

            // A instruction
            let a_value = line.split("//").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();

            let mut a_value = a_value.chars();
            a_value.next();
            if let Ok(a_value) = a_value.as_str().parse::<u32>() {
                output_file
                    .write_fmt(format_args!("0{:0>15b}\n", a_value))
                    .expect("Unable to write line")
            } else {
                let address = symbol_table.get_address(a_value.as_str()).unwrap();
                output_file
                    .write_fmt(format_args!("0{:0>15b}\n", address))
                    .expect("Unable to write line")
            }
        } else if line.starts_with("//") || line.starts_with("(") {
            // comment
            continue;
        } else {
            // c instruction
            let line = line
                .split("//")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .trim()
                .to_string();

            if line.contains("=") && line.contains(";") {
                // 3 part
                let args: Vec<&str> = line.split("=").collect();
                let dest_bin = get_dest_instr(args[0]).expect("Dest instruction missing");
                let args: Vec<&str> = args[1].split(";").collect();
                let comp_bin = get_comp_instr(args[0]).expect("Comp instruction missing");
                let jump_bin = get_jump_instr(args[1]).expect("Jump instruction missing");

                output_file
                    .write_fmt(format_args!("111{}{}{}\n", comp_bin, dest_bin, jump_bin))
                    .expect("Unable to write line")
            } else if line.contains("=") {
                // dest and comp
                let args: Vec<&str> = line.split("=").collect();
                let dest_bin = get_dest_instr(args[0]).expect("Dest instruction missing");
                let comp_bin = get_comp_instr(args[1]).expect("Comp instruction missing");

                output_file
                    .write_fmt(format_args!("111{}{}000\n", comp_bin, dest_bin))
                    .expect("Unable to write line")
            } else if line.contains(";") {
                // comp and jump
                let args: Vec<&str> = line.split(";").collect();
                let comp_bin = get_comp_instr(args[0]).expect("Comp instruction missing");
                let jump_bin = get_jump_instr(args[1]).expect("Jump instruction missing");

                output_file
                    .write_fmt(format_args!("111{}000{}\n", comp_bin, jump_bin))
                    .expect("Unable to write line")
            } else {
                // comp only
                let comp_bin = get_comp_instr(line.as_str()).expect("Comp instruction missing");

                output_file
                    .write_fmt(format_args!("111{}000000\n", comp_bin))
                    .expect("Unable to write line");
            }
        }
    }
}
