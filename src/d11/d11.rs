pub mod d11 {
    use std::collections::HashSet;
    use num::abs;

    pub(crate) fn d11(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let mut result2 = 0;

        // padding horizontally
        let mut intermediary_grid: Vec<Vec<char>> = Vec::new();
        for line in file.iter() {
            let mut line_vec = Vec::new();
            let mut empty = true;
            for char in line.chars() {
                if char == '#' {
                    empty = false;
                }
                line_vec.push(char);
            }
            if empty {
                intermediary_grid.push(vec!['.'; line.len()]);
            }
            intermediary_grid.push(line_vec);
        }

        // padding vertically
        let mut grid: Vec<Vec<char>> = vec![Vec::new(); intermediary_grid.len()];
        let mut galaxy_coords = Vec::new();
        let mut galaxy_id = 1;
        for x in 0..intermediary_grid[0].len() {
            let mut empty = true;
            for y in 0..intermediary_grid.len() {
                if intermediary_grid[y][x] == '#' {
                    empty = false;
                    intermediary_grid[y][x] = format!("{}", galaxy_id).parse().unwrap();
                    galaxy_id += 1;
                }
            }
            for y in 0..intermediary_grid.len() {
                if empty {
                    grid[y].push('.');
                }
                grid[y].push(intermediary_grid[y][x]);
            }
        }

        // finding galaxies
        for (y,line) in grid.iter().enumerate(){
            for (x, char) in line.iter().enumerate(){
                if *char != '.' {
                    galaxy_coords.push((char,(x as i32 ,y as i32)));
                }
            }
        }

        // making pairs
        let mut galaxy_pairs= HashSet::new();
        for (i, coords1) in galaxy_coords.iter().enumerate() {
            for coords2 in galaxy_coords.iter().skip(i + 1) {
                galaxy_pairs.insert((coords1, coords2));
            }
        }

        println!("{}", galaxy_pairs.len());
        for pair in galaxy_pairs.iter(){
            let manhattan_distance = abs(pair.0.1.0 - pair.1.1.0) + abs(pair.0.1.1 - pair.1.1.1);
            println!("{:?}-{:?} {:?}-{:?} {:?}", pair.0.0, pair.1.0, pair.0.1, pair.1.1, manhattan_distance);
            result1 += manhattan_distance;
        }


        for line in grid {
            for char in line {
                print!("{}", char);
            }
            println!();
        }

        (result1, result2)
    }
}
