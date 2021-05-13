use std::{
    fs::File,
    io::{BufReader, LineWriter},
};

pub fn get_input_file(file_path: &str) -> BufReader<File> {
    let input_file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(input_file);
    reader
}

pub fn parse_filepath(args: Vec<&str>) -> &str {
    if args.len() != 2 {
        panic!("Usage: cargo run <.vm file or filepath>");
    }
    args[1]
}

pub fn write_output_file(filename: &str) -> LineWriter<File> {
    let output_file = File::create(filename).expect("Unable to create file");
    let output_file = LineWriter::new(output_file);
    output_file
}
