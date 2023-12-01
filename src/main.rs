pub mod d1;

use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

use crate::d1::d1::d1::hello_world;


fn main() {
    hello_world();
    let file = read_file("src/d1/d1.txt").expect("Failed to read file");

    let mut sum = 0;

    let pattern = Regex::new(r"[a-z]").expect("Failed to create regex");
    for line in file {
        let line = pattern.replace_all( &* line, "");


        if let (Some(first_char), Some(last_char)) = (line.chars().next(), line.chars().last()) {
            let digit_sum = (first_char.to_string() + &last_char.to_string()).parse::<i32>().expect("Parse error");
            sum += digit_sum;
            //println!("{}", digit_sum);
        }
    }
    print!("{}", sum);

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