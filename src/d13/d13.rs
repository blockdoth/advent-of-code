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

            println!("Vertical");
            result1 += find_symmetry_bounds(v_lines);
            println!("horizontal");
            result1 += find_symmetry_bounds(h_lines) * 100;
        }
        (result1, result2)
    }

    fn find_symmetry_bounds(v_lines: Vec<String>) -> i32 {
        for line in &v_lines{
            println!("{}", line);
        }
        // finding symmetry vertical
        let mut left_index = 0;
        for lines_duo in v_lines.windows(2) {
            if lines_duo[0] == lines_duo[1] {
                break;
            }
            left_index += 1;
        }

        // finding symmetry bounds
        //println!("{:?} {:?}", left_index, right_index);
        //println!("{}\n{}", line_1, line_2);
        let mut line_1 = String::new();
        let mut line_2 = String::new();
        let mut right_index = left_index + 1;
        println!();
        loop {
            line_1 = v_lines[left_index].clone();
            line_2 = v_lines[right_index].clone();
            println!("{}\n{}", line_1, line_2);
            right_index += 1;
            left_index -= 1;
            if line_2 != line_1 || right_index == v_lines.len() || left_index == 0{
                break
            }
        }
        println!("{:?} {:?}", left_index, right_index);
        if left_index == 0 || right_index + 1 == v_lines.len() {
            return right_index as i32
        }else{
            return 0;
        }

    }
}
