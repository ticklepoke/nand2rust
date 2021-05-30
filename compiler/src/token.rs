use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: &str) -> Self {
        Token {
            token_type,
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Keyword(_) => write!(f, "keyword"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "class" => Ok(Keyword::Class),
            "method" => Ok(Keyword::Method),
            "function" => Ok(Keyword::Function),
            "constructor" => Ok(Keyword::Constructor),
            "int" => Ok(Keyword::Int),
            "boolean" => Ok(Keyword::Boolean),
            "char" => Ok(Keyword::Char),
            "void" => Ok(Keyword::Void),
            "var" => Ok(Keyword::Var),
            "static" => Ok(Keyword::Static),
            "field" => Ok(Keyword::Field),
            "let" => Ok(Keyword::Let),
            "do" => Ok(Keyword::Do),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "while" => Ok(Keyword::While),
            "return" => Ok(Keyword::Return),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "null" => Ok(Keyword::Null),
            "this" => Ok(Keyword::This),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
