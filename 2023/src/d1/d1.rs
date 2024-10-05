pub mod d1{
    use regex::Regex;


    pub(crate) fn d1_1(file:Vec<String>) -> i32 {
        let mut sum = 0;
        let pattern = Regex::new(r"[a-z]").expect("Failed to create regex");
        for line in file {
            let line = pattern.replace_all(&*line, "");
            if let (Some(first_char), Some(last_char)) = (line.chars().next(), line.chars().last()) {
                let digit_sum = (first_char.to_string() + &last_char.to_string()).parse::<i32>().expect("Parse error");
                sum += digit_sum;
            }
        }
        sum
    }
    pub(crate) fn d1_2(file:Vec<String>) -> i32 {
        let mut sum = 0;

        let number_strings = vec!["zero","one","two","three", "four","five","six","seven","eight", "nine"];
        let mut regex_patterns = Vec::new();

        for number_string in &number_strings {
            match Regex::new(number_string) {
                Ok(regex) => regex_patterns.push(regex),
                _ => ()
            };
        }
        for mut line in file {
            for i in 0.. regex_patterns.len(){
                let number_string = number_strings[i].to_string();
                let replacement = number_string.to_string() + &i.to_string() + &number_string.chars().last().unwrap().to_string();
                line = regex_patterns[i].replace_all(line.as_str(), replacement.as_str()).to_string();
            }

            let pattern = Regex::new(r"[a-z]").expect("Failed to create regex");
            let line = pattern.replace_all(&*line, "");
            if let (Some(first_char), Some(last_char)) = (line.chars().next(), line.chars().last()) {
                let digit_sum = (first_char.to_string() + &last_char.to_string()).parse::<i32>().expect("Parse error");
                sum += digit_sum;
            }
        }
        sum
    }
}


