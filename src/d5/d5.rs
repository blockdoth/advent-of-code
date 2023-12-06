pub mod d5 {

    pub(crate) fn d5(file: Vec<String>) -> (i32, i32) {

        let seeds = file[0]
            .split(':')
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .skip(1)
            .map(|s| s.parse::<i64>().expect("parse error"))
            .collect::<Vec<i64>>();


        let maps = file
            .split(|line| line.is_empty())
            .map(|line| line.to_vec())
            .collect::<Vec<Vec<String>>>();

        //Calculates maps
        let mut maps_vec = Vec::new();
        for map in maps.iter().skip(1){
            let mut triple :Vec<(i64, i64, i64)> = Vec::new();
            for line in map.iter().skip(1){
                let numbers = line.split_whitespace().map(|a| a.to_string()).collect::<Vec<String>>();
                let destination = numbers[0].parse::<i64>().expect("int parse error");
                let source = numbers[1].parse::<i64>().expect("int parse error");
                let count = numbers[2].parse::<i64>().expect("int parse error");
                triple.push((source,source+count,destination-source));
            }
            maps_vec.push(triple);
        }

        // The alg
        // You keep track of the ranges of all the mappings in a map,
        // and split those into smaller ranges by the edges of the ranges of the next map,
        // if you do this you create a tree of increasingly smaller ranges,
        // until you end up with a bunch of ranges of the form [n,n+1...n+k] where n and k are arbitrary integers.
        // Since each range is internally sorted you can pick the first element of each range
        // and calculate the actual value of that element,
        // this element is guaranteed to have the smallest actual value,
        // if you do this for all ranges and take the minimum you get the answer

        let prev_range_map = maps_vec.iter().nth(0).unwrap().clone();
        println!("First map:\t{:?}", prev_range_map);
        let mut range_maps_vec = Vec::new();
        for mut map in maps_vec.into_iter().skip(1){
            map.sort_by(|a, b| a.0.cmp(&b.0));

            println!("Applying:\t{:?}", maps_vec);

            let mut new_map = Vec::new();
            for prev_mapping in prev_range_map.iter(){
                // (left-bound, right-bound, mapping-value)
                let prev_left_range = prev_mapping.0;
                let prev_right_range = prev_mapping.1;
                let prev_mapping_value = prev_mapping.2;
                for next_mapping in maps_vec.iter(){
                    let new_left_range = next_mapping.0;
                    let new_right_range = next_mapping.1;
                    let new_mapping_value = next_mapping.2;

                    if new_left_range > prev_left_range{  // right side of range
                        // same mapping
                        new_map.push((new_left_range,prev_right_range,prev_mapping_value));
                        // new mapping
                        new_map.push((prev_left_range,new_left_range,new_mapping_value + prev_mapping_value));

                    }
                    if prev_right_range > new_right_range{ // left side of range
                        // same mapping
                        new_map.push((new_right_range,prev_left_range,prev_mapping_value));
                        //new mapping
                        new_map.push((prev_left_range,new_right_range,new_mapping_value + prev_mapping_value));
                    }
                    // if !(prev_left_range > new_left_range && new_right_range < prev_right_range ){
                    //     new_map.push(*(next_map))
                    // }

                }
                println!("created new map {:?}", new_map);
                //  new_map.clear();
            }
            range_maps_vec.add(new_map);
        }

        // Push only the first element in each final range,
        // since the rest of the values of each range are guaranteed to be consecutive
        let mut range_seeds = Vec::new();
        for range in range_maps_vec.into_iter(){
            range_seeds.push(range.0);
        }

        let result1 = calc_mapping(seeds, &maps_vec) as i32;
        let result2 = calc_mapping(range_seeds, &maps_vec) as i32;

        (result1, result2)
    }

    fn calc_mapping(seeds: Vec<i64>, maps_vec: &Vec<Vec<(i64, i64, i64)>>) -> i64{
        let mut result = i64::MAX;
        for mut seed in seeds{
            for map in maps_vec{
                for range in map.iter(){
                    if seed > range.0 && seed <= range.1 {
                        // seed += destination - source
                        seed += range.2;
                        break;
                    }
                }
            }
            if seed < result {
                result = seed;
            }
        }
        result
    }
}
