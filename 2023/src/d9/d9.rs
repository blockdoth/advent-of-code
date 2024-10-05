pub mod d9 {
    use regex::Regex;

    pub(crate) fn d9(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let mut result2 = 0;

        for line in file {
            let regex = Regex::new(r" ").expect("pattern error");
            let mut values: Vec<i32> = regex.split(&*line).map(|s| s.parse().unwrap()).collect();
            let mut differences:Vec<Vec<i32>> = Vec::new();
            differences.push(values.clone());
            let mut zero_count = 0;
            while zero_count != values.len(){
                zero_count = 0;
                let mut lower_values = Vec::new();
                for value in values.windows(2) {
                    let next_value = value[1] - value[0];
                    lower_values.push(next_value);
                    if next_value == 0 {
                        zero_count += 1;
                    }
                }
                values = lower_values.clone();
                differences.push(lower_values);
            }

            for list in &differences{
                result1 += list.iter().last().expect("error");
            }
            let mut count = 0;
            for list in differences.iter().skip(1).rev() {
                count = list[0] - count;
            }
            result2 += differences.get(0).expect("").get(0).expect("") - count;
        }
        (result1, result2)
    }
}
