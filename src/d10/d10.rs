pub mod d10 {
    use std::collections::{HashMap, HashSet, VecDeque};


    pub(crate) fn d10(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let result2 = 0;

/*      The pipes are arranged in a two-dimensional grid of tiles:

        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/
        let straight_pipe_v = [
            [0,1,0],
            [0,0,0],
            [0,1,0]
            ];
        let straight_pipe_h= [
            [0,0,0],
            [1,0,1],
            [0,0,0]
        ];

        let bend_pipe_ne = [
            [0,1,0],
            [0,0,1],
            [0,0,0]
        ];
        let bend_pipe_nw = [
            [0,1,0],
            [1,0,0],
            [0,0,0]
        ];
        let bend_pipe_se = [
            [0,0,0],
            [0,0,1],
            [0,1,0]
        ];
        let bend_pipe_sw = [
            [0,0,0],
            [1,0,0],
            [0,1,0]
        ];

        let start_pipe = [
            [0,1,0],
            [1,0,1],
            [0,1,0]
        ];

        let mut pipe_map: HashMap<char, [[u8; 3]; 3]> = HashMap::new();
        pipe_map.insert('|', straight_pipe_v);
        pipe_map.insert('-', straight_pipe_h);
        pipe_map.insert('L', bend_pipe_ne);
        pipe_map.insert('J', bend_pipe_nw);
        pipe_map.insert('F', bend_pipe_se);
        pipe_map.insert('7', bend_pipe_sw);
        pipe_map.insert('S', start_pipe);

        let height = file.len();
        let width = file.get(0).expect("").len();

        // building the char grid with padding
        let mut char_grid:Vec<Vec<char>> = Vec::new();
        char_grid.push(vec!['.'; width + 2]);
        let mut start_coordinates= (0, 0);
        for (y,line) in file.iter().enumerate() {
            let mut char_row = Vec::new();
            char_row.push('.');
            for (x,char) in line.chars().enumerate() {
                if char == 'S' {
                    start_coordinates = (x + 1, y + 1);
                }
                char_row.push(char);
            }
            char_row.push('.');
            char_grid.push(char_row);
        }
        char_grid.push(vec!['.'; width + 2]);

        // building the value grid with padding
        let mut value_grid = Vec::with_capacity(height);
        for _ in 0..width + 2{
            value_grid.push(vec![0; width + 2]);
        }

        // traversing breadth first
        let mut char_q:VecDeque<(usize,usize)> = VecDeque::new();
        let mut explored: HashSet<(usize,usize)> = HashSet::new();

        char_q.push_back(start_coordinates);
        explored.insert(start_coordinates);
        value_grid[start_coordinates.1][start_coordinates.0] = 0;
        let mut first_iter = true;
        while !char_q.is_empty(){
            let current_coordinates = char_q.pop_front().expect("queue error");
            let current_char = char_grid[current_coordinates.1][current_coordinates.0];
            let options_map = pipe_map.get(&current_char).expect("not found");

            //println!("Current char {}", current_char);
            for y_offset in 0..3{
                for x_offset in 0..3{
                    let x_cord = current_coordinates.0 + x_offset - 1;
                    let y_cord = current_coordinates.1 + y_offset - 1;
                    let char_to_check = char_grid[y_cord][x_cord];
                    if options_map[y_offset][x_offset] == 1 && char_to_check != '.'{
                        if first_iter {
                            let char_options_map = pipe_map.get(&char_to_check).expect("not found");
                            match (x_offset, y_offset) {
                                (0, 1) if char_options_map[1][2] != 1 => continue,
                                (1, 0) if char_options_map[2][1] != 1 => continue,
                                (2, 1) if char_options_map[1][0] != 1 => continue,
                                (1, 2) if char_options_map[0][1] != 1 => continue,
                                _ => {}
                            }
                        }
                        //println!("{}", char_to_check);
                        let next_pipe = (x_cord,y_cord);
                        if !explored.contains(&next_pipe){
                            char_q.push_back(next_pipe);
                            explored.insert(next_pipe);
                            let current_distance = value_grid[current_coordinates.1][current_coordinates.0] + 1;
                            if current_distance > result1 {
                                result1 = current_distance;
                            }
                            //println!("found {} at {}{}",get_char_at_coords(char_grid.clone(),x_cord,y_cord),x_cord,y_cord);
                            value_grid[next_pipe.1][next_pipe.0] = current_distance;
                        }
                    }

                }
            }
            first_iter = false;
            //print!("\x1bc");
            //print_string_grid_with_updates(&char_grid, &value_grid,result1);
        }
        print_string_grid_with_updates(&char_grid, &value_grid,result1);

        (result1, result2)
    }
    fn print_string_grid_with_updates(string_grid: &Vec<Vec<char>>, value_grid: &Vec<Vec<i32>>, highest_value: i32) {
        for (y,line) in string_grid.iter().enumerate() {
            for (x,char) in line.iter().enumerate() {
                let coord_value = value_grid[y][x];
                if coord_value > 0{
                    if coord_value == highest_value {
                        print!("\x1b[32m {}\x1b[0m", char);
                    }else{
                        print!("\x1b[31m {}\x1b[0m", char);
                    }
                } else {
                    print!(" {}", char);
                }
            }
            print!("\n")
        }
    }
    fn generate_string_grid_with_updates(string_grid: &Vec<Vec<char>>, value_grid: &Vec<Vec<i32>>, highest_value: i32) -> String {
        let mut result = String::new();
        for (y, line) in string_grid.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                let coord_value = value_grid[y][x];
                if coord_value > 0 {
                    if coord_value == highest_value {
                        result.push_str(&format!("\x1b[32m {}\x1b[0m", char));
                    } else {
                        result.push_str(&format!("\x1b[31m {}\x1b[0m", char));
                    }
                } else {
                    result.push_str(&format!(" {}", char));
                }
            }
            result.push('\n');
        }
        result
    }
    fn print_int_grid(int_grid: Vec<Vec<i32>>) {
        for line in int_grid{
            for x in line {
                print!(" {}", x);
            }
            print!("\n")
        }
    }
}
