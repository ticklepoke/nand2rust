use std::{fs::File, io::BufReader};

pub fn get_input_file(file_path: &str) -> BufReader<File> {
    let input_file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(input_file);
    reader
}
