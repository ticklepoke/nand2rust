use std::{
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
};

use crate::instructions::{get_comp_instr, get_dest_instr, get_jump_instr};

pub fn assemble_l(full_file_path: &String) {
    let split_path = full_file_path
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();

    let filename_without_extension = split_path
        .split(".")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string();

    let input_file = File::open(full_file_path).expect("File not found!");
    let reader = BufReader::new(input_file);

    let output_file = File::create(format!("./data/{}.hack", filename_without_extension))
        .expect("Unable to create output file");
    let mut output_file = LineWriter::new(output_file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line!").trim().to_string();

        if line.len() == 0 {
            continue;
        }

        if &line[0..1] == "@" {
            // A instruction
            let a_value = line.split("//").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();

            let mut a_value = a_value.chars();
            a_value.next();
            let a_value = a_value.as_str().parse::<u32>().unwrap();

            output_file
                .write_fmt(format_args!("0{:0>15b}\n", a_value))
                .expect("Unable to write line")
        } else if &line[0..2] == "//" {
            // comment
            continue;
        } else {
            // c instruction
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

    output_file.flush().expect("Unable to complete file write");
    println!("Assembly completed");
}
