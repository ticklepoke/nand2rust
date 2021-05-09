use std::{fs::File, io::{BufReader, LineWriter}};

pub fn split_filename(full_file_path: &str) -> String {
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

    filename_without_extension
}

pub fn get_input_file(full_file_path: &str) -> BufReader<File> {
    let input_file = File::open(full_file_path).expect("File not found!");
    let reader = BufReader::new(input_file);
    reader
}

pub fn write_output_file(filename_without_extension: &str) -> LineWriter<File> {
    let output_file = File::create(format!("./data/{}.hack", filename_without_extension))
        .expect("Unable to create output file");
    let output_file = LineWriter::new(output_file);
    output_file
}
