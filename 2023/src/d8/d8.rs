pub mod d8 {
    use std::collections::HashMap;
    use num::Integer;
    use regex::Regex;

    pub(crate) fn d8(file: Vec<String>) -> (i32, usize) {
        let mut result1 = 0;

        let instructions:Vec<bool> = file[0].chars().map(|a|
            return if a == 'L' { false } else { true }
        ).collect();

        let mut mappings = HashMap::new();
        let mut location = "";
        let mut locations = Vec::new();

        for mapping in file.iter().skip(2){
            let regex = Regex::new(r"[^A-Z]+").expect("pattern error");
            let codes: Vec<&str> = regex.split(mapping).filter(|a| !a.is_empty()).collect();
            if location == "" {
                location = codes[0];
            }
            if codes[0].chars().nth(2).expect("parse error") == 'A' {
                locations.push(codes[0]);
            }
            mappings.insert(codes[0],(codes[1],codes[2]));
        }

        while location != "ZZZ"{
            let direction = instructions[result1 % (instructions.len())];
            result1 += 1;
            let options = mappings.get(location).expect("not found error");
            if direction == false {
                let options = mappings.get(location).expect("not found error");
                location = options.0;
            } else {
                location = options.1;
            }
        }

        let mut counters = Vec::new();
        for start_location in locations{
            let mut index = 0;
            let mut location= start_location;
            while location.chars().nth(2).expect("parse error") != 'Z'{
                let direction = instructions[index % (instructions.len())];
                index += 1;
                let options = mappings.get(location).expect("not found error");
                if direction == false {
                    let options = mappings.get(location).expect("not found error");
                    location = options.0;
                } else {
                    location = options.1;
                }
            }
            counters.push(index);
        }

        let mut result2 = counters[0];
        for count in counters.iter().skip(1){
            result2 =  result2.lcm(count);
        }

        (result1 as i32, result2)
    }

}
