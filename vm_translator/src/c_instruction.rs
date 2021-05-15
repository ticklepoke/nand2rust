use crate::{memory_segment::MemorySegment, parser::arithmetic_instruction::ArithmeticInstruction};

#[derive(Debug)]
pub struct CArithmetic {
    pub command_type: ArithmeticInstruction,
}

#[derive(Debug)]
pub struct CPush {
    pub segment: MemorySegment,
    pub index: usize,
}

#[derive(Debug)]
pub struct CPop {
    pub segment: MemorySegment,
    pub index: usize,
}

#[derive(Debug)]
pub struct CLabel {
    pub label: String,
}

#[derive(Debug)]
pub struct CGoto {
    pub label: String,
}

#[derive(Debug)]
pub struct CIf {
    pub label: String,
}

#[derive(Debug)]
pub struct CFunction {
    pub function_name: String,
    pub n_locals: usize,
}

#[derive(Debug)]
pub struct CReturn {}

#[derive(Debug)]
pub struct CCall {
    pub function_name: String,
    pub n_locals: usize,
}

#[derive(Debug)]
pub enum CInstruction {
    CArithmetic(CArithmetic),
    CPush(CPush),
    CPop(CPop),
    CLabel(CLabel),
    CGoto(CGoto),
    CIf(CIf),
    CFunction(CFunction),
    CReturn(CReturn),
    CCall(CCall),
}
