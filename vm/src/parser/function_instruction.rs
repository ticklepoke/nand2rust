use std::str::FromStr;

pub enum FunctionInstruction {
    Function,
    Call,
    Return,
}

impl FromStr for FunctionInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "function" => Ok(FunctionInstruction::Function),
            "call" => Ok(FunctionInstruction::Call),
            "return" => Ok(FunctionInstruction::Return),
        }
    }
}
