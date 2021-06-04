use std::{
    fs::File,
    io::{BufRead, BufReader, LineWriter},
};

pub fn get_input_file(file_path: &str) -> BufReader<File> {
    let input_file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(input_file);
    reader
}

pub fn strip_comments(buf_reader: BufReader<File>) -> String {
    let mut output_string = String::new();

    let mut is_in_block_comment = false;

    for line in buf_reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("//") {
                continue;
            } else if line.contains("//") {
                let split_code = line
                    .split("//")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_owned();
                output_string.push_str(split_code);
            } else if line.contains("/*") && line.contains("*/") {
                let split_code = line.split("/*").collect::<Vec<&str>>();
                let mut split_code = split_code.iter();
                output_string.push_str(split_code.next().unwrap().to_owned());
                while let Some(&token) = split_code.next() {
                    if token.contains("*/") {
                        continue;
                    }
                    output_string.push_str(token);
                }
            } else if line.contains("/*") {
                is_in_block_comment = true;
            } else if line.contains("*/") {
                is_in_block_comment = false;
            } else if is_in_block_comment {
                continue;
            } else {
                output_string.push_str(line.as_str());
            }
        }
    }
    //output_string.replace("\t", "")
    //output_string.replace("  ", "")
    output_string
}

pub fn parse_filepath(args: Vec<&str>) -> &str {
    if args.len() != 2 {
        panic!("Usage: cargo run <.vm file or folder>");
    }
    args[1]
}

pub fn get_output_handle(filename: &str) -> LineWriter<File> {
    let output_file = File::create(filename).expect("Unable to create file");
    let output_file = LineWriter::new(output_file);
    output_file
}
