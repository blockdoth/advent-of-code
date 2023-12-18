pub mod d11 {
    use std::collections::HashSet;
    use num::abs;

    pub(crate) fn d11(file: Vec<String>) -> (i64, i64) {
        let mut result1 = 0;
        let mut result2 = 0;

        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut galaxy_coords = Vec::new();
        for (y,line) in file.iter().enumerate(){
            let mut line_vec = Vec::new();
            for (x, char) in line.chars().enumerate(){
                if char != '.' {
                    galaxy_coords.push((x as i32,y as i32));
                }
                line_vec.push(char);

            }
            grid.push(line_vec)
        }

        let mut h_lines = Vec::new();
        for y in 0..grid.len(){
            let mut empty = true;
            for x in 0..grid[0].len(){
                if grid[y][x] != '.' {
                    empty = false;
                }
            }
            if empty {
                h_lines.push(y as i32);
            }
        }

        let mut v_lines = Vec::new();
        for x in 0..grid[0].len(){
            let mut empty = true;
            for y in 0..grid.len(){
                if grid[y][x] != '.' {
                    empty = false;
                }
            }
            if empty {
                v_lines.push(x as i32);
            }
        }

        result1 = calc_distances_with_expansion_factor(&galaxy_coords,&h_lines,&v_lines, 2);
        result2 = calc_distances_with_expansion_factor(&galaxy_coords,&h_lines,&v_lines, 1000000);

        (result1, result2)
    }

    fn calc_distances_with_expansion_factor(galaxy_coords: &Vec<((i32, i32))>, h_lines: &Vec<i32>, v_lines: &Vec<i32>, factor: i32) -> i64 {
        let mut updated_coords = Vec::new();
        for (coords_x,coords_y) in galaxy_coords.iter() {
            let mut new_coord_x = *coords_x;
            for v_line in v_lines {
                if coords_x > v_line {
                    new_coord_x += factor - 1;
                }
            }
            let mut new_coord_y = *coords_y;
            for h_line in h_lines {
                if coords_y > h_line {
                    new_coord_y += factor  - 1;
                }
            }
            updated_coords.push((new_coord_x,new_coord_y))
        }

        let mut galaxy_pairs= HashSet::new();
        for (i, coords1) in updated_coords.iter().enumerate() {
            for coords2 in updated_coords.iter().skip(i + 1) {
                galaxy_pairs.insert((coords1, coords2));
            }
        }

        let mut result = 0;
        for pair in galaxy_pairs.iter(){
            let manhattan_distance = abs(pair.0.0 - pair.1.0) + abs(pair.0.1 - pair.1.1);
            result += manhattan_distance as i64;
        }
        result
    }
}
