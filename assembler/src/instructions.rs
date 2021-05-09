pub fn get_comp_instr(comp_instr: &str) -> Option<&str> {
    let bin_instr = match comp_instr {
        "0" => Some("0101010"),
        "1" => Some("0111111"),
        "-1" => Some("0111010"),
        "D" => Some("0001100"),
        "A" => Some("0110000"),
        "!D" => Some("0001101"),
        "!A" => Some("0110001"),
        "-D" => Some("0001111"),
        "-A" => Some("0110011"),
        "D+1" => Some("0011111"),
        "A+1" => Some("0110111"),
        "D-1" => Some("0001110"),
        "A-1" => Some("0110010"),
        "D+A" => Some("0000010"),
        "D-A" => Some("0010011"),
        "A-D" => Some("0000111"),
        "D&A" => Some("0000000"),
        "D|A" => Some("0010101"),
        "M" => Some("1110000"),
        "!M" => Some("1110001"),
        "-M" => Some("1110011"),
        "M+1" => Some("1110111"),
        "M-1" => Some("1110010"),
        "D+M" => Some("1000010"),
        "D-M" => Some("1010011"),
        "M-D" => Some("1000111"),
        "D&M" => Some("1000000"),
        "D|M" => Some("1010101"),
        _ => None,
    };
    bin_instr
}

pub fn get_dest_instr(dest_instr: &str) -> Option<&str> {
    let bin_instr = match dest_instr {
        "null" => Some("000"),
        "M" => Some("001"),
        "D" => Some("010"),
        "MD" => Some("011"),
        "A" => Some("100"),
        "AM" => Some("101"),
        "AD" => Some("110"),
        "AMD" => Some("111"),
        _ => None,
    };
    bin_instr
}

pub fn get_jump_instr(jump_instr: &str) -> Option<&str> {
    let bin_instr = match jump_instr {
        "null" => Some("000"),
        "JGT" => Some("001"),
        "JEQ" => Some("010"),
        "JGE" => Some("011"),
        "JLT" => Some("100"),
        "JNE" => Some("101"),
        "JLE" => Some("110"),
        "JMP" => Some("111"),
        _ => None,
    };
    bin_instr
}