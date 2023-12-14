pub mod d9;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d9::d9::d9::d9;

fn main() {
    let file = read_file("src/d9/d9.txt").expect("Failed to read file");
    let results = d9(file);
    println!("Result: {} {}", results.0, results.1)
}

fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let value = line.expect("Unexpected tokens");
        result.push(value);
    }
    Ok(result)
}
