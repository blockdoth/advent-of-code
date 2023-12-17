pub mod d11;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d11::d11::d11::d11;

fn main() {
    let file = read_file(r"C:\Users\pepij\RustroverProjects\AOC\src\d11\d11.txt").expect("Failed to read file");
    let results = d11(file);
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
