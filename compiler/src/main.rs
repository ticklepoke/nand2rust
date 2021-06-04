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
        let mut xml_writer = XMLWriter::new(file.replace(".jack", ".tokenized.xml").as_str());
        tokenizer.advance();
        while tokenizer.current_token.is_some() {
            if let Some(current_token_type) = tokenizer.get_token_type() {
                let current_token_value = tokenizer.get_value();
                let formatted_type = match current_token_type {
                    token::TokenType::StringConst => String::from("stringConstant"),
                    token::TokenType::IntConst => String::from("integerConstant"),
                    _ => current_token_type.to_string().to_lowercase(),
                };
                xml_writer.write_line(formatted_type.as_str(), current_token_value.as_str());
            }
            tokenizer.advance();
        }

        xml_writer.close();
    }
}

#[cfg(test)]
mod tests {

    use crate::tokenize;
    use std::process::Command;

    #[test]
    fn run_tokenizer() {
        let files = vec![
            "./data/ArrayTest/Main.jack",
            "./data/ExpressionLessSquare/Main.jack",
            "./data/ExpressionLessSquare/Square.jack",
            "./data/ExpressionLessSquare/SquareGame.jack",
            "./data/Square/Main.jack",
            "./data/Square/Square.jack",
            "./data/Square/SquareGame.jack",
        ];
        for file in files {
            tokenize(vec!["", file]);

            let mut child = Command::new("./../tools/TextComparer.sh")
                .arg(file.replace(".jack", ".tokenized.xml"))
                .arg(file.replace(".jack", "T.xml"))
                .spawn()
                .unwrap();

            let exit_status = child.wait().unwrap();

            assert!(exit_status.success(), "Test failed: {:?}", file);
        }
    }
}
