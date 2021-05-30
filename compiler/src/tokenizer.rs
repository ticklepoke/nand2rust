use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
    str::{FromStr, Split},
};

use regex::Regex;

use crate::{
    token::{Keyword, Token, TokenType},
    utils::{get_input_file, strip_comments},
};

pub struct Tokenizer {
    pub tokens: String,
    pub current_token: Option<Token>,
    fast: usize,
    slow: usize,
}

impl Tokenizer {
    pub fn new(file_path: &str) -> Self {
        let buf_reader = get_input_file(file_path);
        let stripped_source = strip_comments(buf_reader);

        let mut idx = 0;
        while stripped_source.chars().nth(idx).unwrap().is_whitespace() {
            idx += 1;
        }

        Tokenizer {
            tokens: stripped_source,
            current_token: None,
            fast: idx,
            slow: idx,
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        let mut found_token = None;
        while self.fast < self.tokens.len() && self.slow < self.tokens.len() {
            self.advance_until_different_token();
            if let Some(token) = self.match_pattern(&self.tokens[self.slow..=self.fast]) {
                found_token = Some(token);
                self.current_token = Some(found_token.clone().unwrap());
            }
            if found_token.is_some() {
                // do some matching, return if found, set current_token
                // TODO: extract value

                self.advance_beyond_whitespace();
                self.slow = self.fast;
                return found_token; // return the token
            }

            // Nothing found, continue looking
            self.fast += 1;
        }
        self.current_token = None;
        None
    }

    pub fn get_token_type(&mut self) -> Option<TokenType> {
        if self.current_token.is_some() {
            let current_token = self.current_token.as_ref();
            return Some(current_token.unwrap().token_type);
        }
        None
    }

    pub fn get_value(&mut self) -> String {
        if let Some(token_type) = self.get_token_type() {
            return match token_type {
                TokenType::IntConst
                | TokenType::Identifier
                | TokenType::StringConst
                | TokenType::Keyword(_) => self.current_token.as_ref().unwrap().value.clone(),
                TokenType::Symbol => {
                    let token = self.current_token.as_ref().unwrap().value.clone();
                    match token.as_str() {
                        "<" => String::from("&lt"),
                        ">" => String::from("&gt"),
                        "&" => String::from("&amp"),
                        _ => token,
                    }
                }
            };
        }
        unreachable!()
    }

    fn match_pattern(&self, pattern: &str) -> Option<Token> {
        if pattern.len() == 1 {
            if let Some(punc) = pattern.chars().next() {
                if punc.is_ascii_punctuation() {
                    return Some(Token::new(TokenType::Symbol, punc.to_string().as_str()));
                }
            }
        }

        // Keyword
        if let Ok(token_type) = Keyword::from_str(pattern) {
            return Some(Token::new(
                TokenType::Keyword(token_type),
                token_type.to_string().to_lowercase().as_str(),
            ));
        }

        // String constant
        if pattern.starts_with('"') {
            if pattern.ends_with('"') {
                return Some(Token::new(
                    TokenType::StringConst,
                    pattern.trim_matches('"'),
                ));
            }
            return None;
        }

        // Numeric
        if let Ok(numeric) = pattern.parse::<f64>() {
            return Some(Token::new(
                TokenType::IntConst,
                numeric.to_string().as_str(),
            ));
        }

        let re_identifier = Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$").unwrap();
        if re_identifier.is_match(pattern) {
            return Some(Token::new(TokenType::Identifier, pattern));
        }

        panic!("Unrecognized Token: {:?}", pattern)
    }

    fn advance_beyond_whitespace(&mut self) {
        loop {
            self.fast += 1;
            if self.fast >= self.tokens.len() - 1
                || !self.tokens.chars().nth(self.fast).unwrap().is_whitespace()
            {
                return;
            }
        }
    }

    fn advance_until_different_token(&mut self) {
        loop {
            if self.fast >= self.tokens.len() - 1
                || self
                    .tokens
                    .chars()
                    .nth(self.fast + 1)
                    .unwrap()
                    .is_whitespace()
                || (self.current_token.is_some()
                    && self.current_token.clone().unwrap().token_type == TokenType::Identifier
                    && self
                        .tokens
                        .chars()
                        .nth(self.fast)
                        .unwrap()
                        .is_ascii_punctuation())
                || (self.current_token.is_some()
                    && self.current_token.clone().unwrap().token_type != TokenType::Symbol
                    && self
                        .tokens
                        .chars()
                        .nth(self.fast + 1)
                        .unwrap()
                        .is_ascii_punctuation())
                || (self.current_token.is_some()
                    && self.current_token.clone().unwrap().token_type == TokenType::Symbol
                    && (self
                        .tokens
                        .chars()
                        .nth(self.fast + 1)
                        .unwrap()
                        .is_whitespace()
                        || self
                            .tokens
                            .chars()
                            .nth(self.fast + 1)
                            .unwrap()
                            .is_ascii_punctuation()))
                || (self
                    .tokens
                    .chars()
                    .nth(self.fast)
                    .unwrap()
                    .is_ascii_punctuation()
                    && !self
                        .tokens
                        .chars()
                        .nth(self.fast + 1)
                        .unwrap()
                        .is_ascii_punctuation())
            {
                return;
            }
            self.fast += 1;
        }
    }
}

pub enum TokenizerErrors {}
