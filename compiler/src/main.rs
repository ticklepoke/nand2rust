use std::env;
use std::fs::read_dir;

use xml_writer::XMLWriter;

use crate::tokenizer::Tokenizer;
use crate::utils::parse_filepath;

mod token;
mod tokenizer;
mod utils;
mod xml_writer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args.iter().map(AsRef::as_ref).collect();
    tokenize(args);
}

fn tokenize(args: Vec<&str>) {
    let file_path = parse_filepath(args).trim();
    let mut files_to_process: Vec<String> = Vec::new();

    if file_path.ends_with(".jack") {
        // single file
        files_to_process.push(file_path.to_string());
    } else {
        // folder of files
        let paths = read_dir(file_path).unwrap();
        for path in paths {
            if let Ok(dir_entry) = path {
                if dir_entry.path().ends_with(".jack") {
                    files_to_process.push(dir_entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    for file in files_to_process {
        let mut tokenizer = Tokenizer::new(file.as_str());
        let mut xml_writer = XMLWriter::new(file.replace(".jack", ".debug.xml").as_str());
        tokenizer.advance();
        while tokenizer.current_token.is_some() {
            if let Some(current_token_type) = tokenizer.get_token_type() {
                let current_token_value = tokenizer.get_value();
                xml_writer.write_line(
                    current_token_type.to_string().to_lowercase().as_str(),
                    current_token_value.as_str(),
                );
            }
            tokenizer.advance();
        }

        xml_writer.close();
    }
}
