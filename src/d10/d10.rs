pub mod d10 {
    use std::collections::HashMap;

    pub(crate) fn d10(file: Vec<String>) -> (i32, i32) {
        let result1 = 0;
        let result2 = 0;

        let mut pipe_map = HashMap::new();
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
        let straight_pipe_v = (
            (0,1,0),
            (0,0,0),
            (0,1,0)
            );
        let straight_pipe_h= (
            (0,0,0),
            (1,0,1),
            (0,0,0)
        );

        let bend_pipe_ne = (
            (0,1,0),
            (0,0,1),
            (0,0,0)
        );
        let bend_pipe_nw = (
            (0,1,0),
            (1,0,0),
            (0,0,0)
        );
        let bend_pipe_se = (
            (0,0,0),
            (0,0,1),
            (0,1,0)
        );
        let bend_pipe_sw = (
            (0,0,0),
            (1,0,0),
            (0,1,0)
        );


        pipe_map.insert('|', straight_pipe_v);
        pipe_map.insert('-', straight_pipe_h);
        pipe_map.insert('L', bend_pipe_ne);
        pipe_map.insert('J', bend_pipe_nw);
        pipe_map.insert('F', bend_pipe_se);
        pipe_map.insert('7', bend_pipe_sw);


        let height = file.len();
        let width = file.get(0).expect("").len();

        let mut start_coordinates = (0, 0);

        for (y,line) in file.iter().enumerate() {
            for (x,char) in line.chars().enumerate() {
                if char == 'S' {
                    start_coordinates = (x,y);
                }
            }
        }

        let mut values = Vec::with_capacity(height);
        for _ in 0..width {
            values.push(vec![0; width]);
        }
        println!("{:?}", start_coordinates);

        println!("{}", file.get(start_coordinates.1).expect("").chars().nth(start_coordinates.0).expect(""));

        print_string_grid(file);
        print_int_grid();
        (result1, result2)
    }

    fn print_string_grid(grid: Vec<Vec<String>>) {
        for line in grid{
            for x in line {
                print!("{}", x);
            }
            print!("\n")
        }
    }
    fn print_int_grid(grid: Vec<Vec<i32>>) {
        for line in grid{
            for x in line {
                print!("{}", x);
            }
            print!("\n")
        }
    }

}
