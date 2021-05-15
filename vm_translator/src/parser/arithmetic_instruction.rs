use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
pub enum ArithmeticInstruction {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl FromStr for ArithmeticInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "add" => Ok(ArithmeticInstruction::Add),
            "sub" => Ok(ArithmeticInstruction::Sub),
            "neg" => Ok(ArithmeticInstruction::Neg),
            "eq" => Ok(ArithmeticInstruction::Eq),
            "gt" => Ok(ArithmeticInstruction::Gt),
            "lt" => Ok(ArithmeticInstruction::Lt),
            "and" => Ok(ArithmeticInstruction::And),
            "or" => Ok(ArithmeticInstruction::Or),
            "not" => Ok(ArithmeticInstruction::Not),
            _ => Err(()),
        }
    }
}

impl Display for ArithmeticInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
