pub mod d4 {
    use std::collections::HashSet;

    pub(crate) fn d4(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let mut cards_count = vec![1; file.len()];
        for (i,line ) in file.iter().enumerate() {
            let split = line.split('|').collect::<Vec<&str>>();
            let winning_numbers = split[0]
                .split(':')
                .collect::<Vec<&str>>()[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().expect("parse error"))
                .collect::<HashSet<i32>>();
            let card_numbers = split[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().expect("parse error"))
                .collect::<HashSet<i32>>();
            let match_count = winning_numbers
                .intersection(&card_numbers)
                .collect::<Vec<_>>().len();
            if match_count > 0 {
                result1 += i32::pow(2, (match_count - 1) as u32);
            }
            for j in 0..match_count  {
                cards_count[i+j+1] += cards_count[i];
            }
        }
        let result2 = cards_count.iter().sum();
        (result1, result2)
    }
}
