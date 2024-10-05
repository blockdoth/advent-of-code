pub mod d5 {
    use std::collections::{HashMap, HashSet};
    use std::io::BufRead;

    pub(crate) fn d5(file: Vec<String>) -> (i32, i32) {
        let mut result2 = 0;

        let seeds = file[0]
            .split(':')
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .map(|s| s.parse::<i32>().expect("parse error"))
            .collect::<Vec<i32>>();

        let maps = file
            .split(|line| line.is_empty())
            .map(|line| line.to_vec())
            .collect::<Vec<Vec<String>>>();

        let mut maps_vec = Vec::new();
        for map in maps.iter().skip(1){
            maps_vec.push(parse_map(map));
        }

        let mut result1 =  i32::MAX;
        for mut seed in seeds{
            for map in maps_vec{
                for range in map{
                    if seed > range.0 && seed <= range.2{
                        // seed += destination - source
                        seed += range.2 - range.1;
                    }
                }
            }
            if seed < result1{
                result1 = seed;
            }
        }

        (result1, result2)
    }

    fn parse_map(map_values: Vec<String>) -> Vec<(i32,i32,i32)> {
        let mut vec :Vec<(i32,i32,i32)> = Vec::new();
        for line in map_values.iter().skip(1){
            let numbers = line.split_whitespace().map(|a| a.to_string()).collect::<Vec<String>>();
            let destination = numbers[0].parse::<i32>().expect("int parse error");
            let source = numbers[1].parse::<i32>().expect("int parse error");
            let count = numbers[2].parse::<i32>().expect("int parse error");
            // 1 141 059 215 largest val
            vec.push((source,source+count,destination));
        }
        vec
    }
}
