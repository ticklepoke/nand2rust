use std::str::FromStr;

pub enum MemoryInstruction {
    Push,
    Pop,
}

impl FromStr for MemoryInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "push" => Ok(MemoryInstruction::Push),
            "pop" => Ok(MemoryInstruction::Pop),
            _ => Err(()),
        }
    }
}
