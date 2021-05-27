use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Split},
    iter::Peekable,
};

use crate::utils::get_input_file;

pub struct Tokenizer {
    lines: Peekable<Lines<BufReader<File>>>,
    curr_line: Option<Vec<String>>,
    token: Option<String>,
}

impl Tokenizer {
    pub fn new(file_path: &str) -> Tokenizer {
        Tokenizer {
            lines: get_input_file(file_path).lines().peekable(),
            curr_line: None,
            token: None,
        }
    }

    fn has_more_lines(&mut self) -> bool {
        self.lines.peek().is_some()
    }

    // Not at last line, and not at last token
    pub fn has_more_tokens(&self) -> bool {
        true
    }

    pub fn advance(&mut self) {
        if self.curr_line.is_none() && self.has_more_lines() {
            let string_vec = self
                .lines
                .next()
                .unwrap()
                .unwrap()
                .split(" ")
                .map(String::from)
                .collect();

            self.curr_line = Some(string_vec);
        }
    }
}
