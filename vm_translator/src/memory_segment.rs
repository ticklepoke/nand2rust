use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
pub enum MemorySegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl FromStr for MemorySegment {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "argument" => Ok(MemorySegment::Argument),
            "local" => Ok(MemorySegment::Local),
            "static" => Ok(MemorySegment::Static),
            "constant" => Ok(MemorySegment::Constant),
            "this" => Ok(MemorySegment::This),
            "that" => Ok(MemorySegment::That),
            "pointer" => Ok(MemorySegment::Pointer),
            "temp" => Ok(MemorySegment::Temp),
            _ => panic!(),
        }
    }
}

impl Display for MemorySegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
