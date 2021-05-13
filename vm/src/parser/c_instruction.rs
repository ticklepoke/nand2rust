use super::{arithmetic_instruction::ArithmeticInstruction, memory_segment::MemorySegment};

pub struct CArithmetic {
    pub command_type: ArithmeticInstruction,
}

pub struct CPush {
    pub segment: MemorySegment,
    pub index: usize,
}

pub struct CPop {
    pub segment: MemorySegment,
    pub index: usize,
}
pub struct CLabel {
    pub label: String,
}
pub struct CGoto {
    pub label: String,
}
pub struct CIf {
    pub label: String,
}
pub struct CFunction {
    pub function_name: String,
    pub n_locals: usize,
}
pub struct CReturn {}
pub struct CCall {
    pub function_name: String,
    pub n_locals: usize,
}

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
