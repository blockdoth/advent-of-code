pub mod d14 {
    use std::collections::HashSet;
    use crate::d14::d14::d14::Direction::{North, South, West, East};
    enum Direction {
        North,
        East,
        South,
        West
    }

    pub(crate) fn d14(file: Vec<String>) -> (i32, i32){
        let mut result1 = 0;

        // finding rolling stones
        let mut grid = Vec::new();
        let mut rolling_stones = Vec::new();
        for (y,line) in file.iter().enumerate(){
            let mut grid_line = Vec::new();
            for (x,char) in line.chars().enumerate(){
                if char == 'O' {
                    rolling_stones.push((x,y))
                }
                grid_line.push(char);
            }
            grid.push(grid_line)
        }

        // looping until loop found
        let mut first_iter = true;
        let mut loop_not_found = true;
        let mut loop_found = false;
        let mut index = 0;
        let mut start_index_loop = 0;
        let mut iters_left = 1000000000;
        let mut grids= HashSet::new();
        while iters_left > 0{

            rolling_stones.sort_by(|a,b| a.1.cmp(&b.1));
            for rolling_stone in &mut rolling_stones {
                tilt(&mut grid, rolling_stone, North);
            }
            if first_iter {
                result1 = score(&grid);
                first_iter = false;
            }
            rolling_stones.sort_by(|a,b| a.0.cmp(&b.0));
            for rolling_stone in &mut rolling_stones {
                tilt(&mut grid, rolling_stone, West);
            }

            rolling_stones.sort_by(|a,b| b.1.cmp(&a.1));
            for rolling_stone in &mut rolling_stones {
                tilt(&mut grid, rolling_stone, South);
            }

            rolling_stones.sort_by(|a,b| b.0.cmp(&a.0));
            for rolling_stone in &mut rolling_stones {
                tilt(&mut grid, rolling_stone, East);
            }

            index += 1;
            iters_left -= 1;
            if grids.contains(&grid.clone()){
                if loop_not_found {
                    start_index_loop = index;
                }
                if loop_found {
                    iters_left = iters_left % (index - start_index_loop);
                    println!("Loop of size {} found", index-start_index_loop);
                }
                loop_not_found = false;
                loop_found = true;
                grids.clear();
            }
            grids.insert(grid.clone());
        }

        let result2 = score(&mut grid);
        (result1, result2)
    }

    fn score(grid: &Vec<Vec<char>>) -> i32 {
        let mut score = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'O' {
                    score += (grid.len() - y) as i32;
                }
            }
        }
        score
    }

    fn print_grid(grid: &Vec<Vec<char>>) {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                print!("{}", grid[y][x])
            }
            println!();
        }
        println!();
    }

    fn tilt(grid: &mut Vec<Vec<char>>, rolling_stone: &mut (usize, usize), direction: Direction){
        let (x,y) = *rolling_stone;

        let mut new_pos_y = y;
        let mut new_pos_x = x;
        match direction {
            North => {
                while new_pos_y > 0 && grid[new_pos_y - 1][x] == '.' {
                    new_pos_y -=1;
                }
            }
            South => {
                while new_pos_y + 1< grid.len()  && grid[new_pos_y + 1][x] == '.' {
                    new_pos_y +=1;
                }
            }
            West => {
                while new_pos_x > 0 && grid[y][new_pos_x - 1] == '.' {
                    new_pos_x -=1;
                }
            }
            East => {
                while new_pos_x + 1< grid[0].len()  && grid[y][new_pos_x + 1] == '.' {
                    new_pos_x +=1;
                }
            }
        }

        if new_pos_y != y || new_pos_x != x{
            grid[new_pos_y][new_pos_x] = grid[y][x];
            grid[y][x] = '.';
            rolling_stone.0 = new_pos_x;
            rolling_stone.1 = new_pos_y;
        }
    }

}
