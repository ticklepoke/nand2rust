use std::env;

use assembler_l::assemble_l;

mod assembler_l;
mod instructions;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <.asm filepath>");
    }

    let full_file_path = &args[1];
    assemble_l(full_file_path);
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};

    use file_diff::diff_files;

    use super::*;

    #[test]
    fn compare_symbolless_files() {
        let files = vec!["MaxL", "PongL", "RectL"];
        compare_files(files);
    }

    fn compare_files(file_names: Vec<&str>) {
        for file_name in file_names {
            assemble_l(&format!("./data/{}.asm", file_name));
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
