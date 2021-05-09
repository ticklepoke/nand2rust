use std::env;

use assembler_s::assemble_s;

mod assembler_l;
mod assembler_s;
mod instructions;
mod symbols;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <.asm filepath>");
    }

    let full_file_path = &args[1];
    // assemble_l(full_file_path);
    assemble_s(full_file_path);
}

#[cfg(test)]
mod tests {
    use assembler_l::assemble_l;
    use std::fs::{self, File};

    use file_diff::diff_files;

    use super::*;

    #[test]
    fn compare_symbolless_files() {
        let files = vec!["MaxL", "PongL", "RectL"];
        compare_files(files, &assemble_l);
    }

    #[test]
    fn compare_symbol_files() {
        let files = vec!["Max", "Pong", "Rect"];
        compare_files(files, &assemble_s);
    }

    fn compare_files(file_names: Vec<&str>, assemble_fn: &dyn Fn(&str)) {
        for file_name in file_names {
            assemble_fn(&format!("./data/{}.asm", file_name));
            let mut generated_file =
                File::open(format!("./data/{}.hack", file_name)).expect("Unable to open file");
            let mut reference_file =
                File::open(format!("./data/{}-ref.hack", file_name)).expect("Unable to open file");

            let cmp_res = diff_files(&mut generated_file, &mut reference_file);

            assert!(cmp_res);

            // Clean up
            fs::remove_file(format!("./data/{}.hack", file_name)).expect("Failed to clean up")
        }
    }
}
