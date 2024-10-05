pub mod d13 {

    pub(crate) fn d13(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let result2 = 0;

        let mut grids:Vec<Vec<String>>= Vec::new();
        let mut grid = Vec::new();
        for line in file {
            if line.is_empty(){
                grids.push(grid.drain(..).collect());
                grid.clear();
            }else{
                grid.push(line);
                //.replace("#","#").replace(".","◻")
            }
        }
        grids.push(grid.drain(..).collect());

        for grid in grids {
            let h_lines = grid;
            let mut v_lines = Vec::new();

            //transposing to
            for x in 0..h_lines[0].len() {
                let mut v_line = String::new();
                for y in 0..h_lines.len() {
                    let new_char = h_lines[y].chars().nth(x).expect("Character not found");
                    v_line = format!("{}{}", v_line, new_char);
                }
                v_lines.push(v_line);
            }


            let v = find_symmetry_bounds(v_lines);
            let h = find_symmetry_bounds(h_lines);
            println!("{} {}", h, v);
            result1 += v + h * 100;
        }
        (result1, result2)
    }

    fn find_symmetry_bounds(grid: Vec<String>) -> i32 {
        // for line in &grid{
        //     println!("{}", line);
        // }
        // finding symmetry vertical
        let mut mirror_line_right = 0;
        let mut no_symmetry = true;
        for lines_duo in grid.windows(2) {
            mirror_line_right += 1;
            if lines_duo[0] == lines_duo[1] {
                no_symmetry = false;
                break;
            }
        }
        if no_symmetry {
            return 0;
        }

        // finding symmetry bounds
        //println!("{:?} {:?}", left_index, right_index);
        //println!("{}\n{}", line_1, line_2);

        let mut left_index = mirror_line_right - 1;
        let mut right_index = mirror_line_right;
        loop {
            let line_1 = grid[left_index].clone();
            let line_2 = grid[right_index].clone();
            if line_2 != line_1 {
                return 0;
            }
            //println!("{}\n{}", line_1, line_2);
            if right_index + 1  == grid.len() || left_index == 0 {
                // println!("{}", mirror_line_right);
                return mirror_line_right as i32;
            }
            right_index += 1;
            left_index -= 1;
        }
    }
}
