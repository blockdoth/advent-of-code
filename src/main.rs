pub mod d1;

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::d1::d1::d1::d1_2;

fn main() {
    let file = read_file("src/d1/d1.txt").expect("Failed to read file");
    let result = d1_2(file);
    println!("Result: {}", result)

}


fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error>{
    // Open the file for reading
    let file = File::open(file_name)?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line

    let mut result = Vec::new();

    for line in reader.lines() {
        let value = line.expect("Unexpected tokens");
        result.push(value);
    }
    Ok(result)
}