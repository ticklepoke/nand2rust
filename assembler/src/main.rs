use std::{env, io::{BufRead, BufReader}};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       panic!("Usage: cargo run <.asm filepath>"); 
    }
    
    let filename = &args[1];

    let f = File::open(filename).expect("File not found!");
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        let line = line.expect("Unable to read line!");
        println!("{:?}", line);
    }
}
