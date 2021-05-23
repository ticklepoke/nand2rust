use std::str::FromStr;

pub enum ControlInstruction {
    Label,
    Goto,
    If,
}

impl FromStr for ControlInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "label" => Ok(ControlInstruction::Label),
            "goto" => Ok(ControlInstruction::Goto),
            "if-goto" => Ok(ControlInstruction::If),
            _ => Err(()),
        }
    }
}
