pub mod d2 {

    pub(crate) fn d2(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let mut result2 = 0;
        const MAX_RED: i32 = 12;
        const MAX_GREEN: i32 = 13;
        const MAX_BLUE: i32 = 14;

        for (i, line) in file.iter().enumerate() {
            let i = i as i32 + 1;
            let line: &String = &line.split(':').collect::<Vec<&str>>()[1].to_string();
            let sets = line.split(';');

            let mut highest_red = 0;
            let mut highest_green = 0;
            let mut highest_blue = 0;

            let mut passed = true;
            for set in sets {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                let cubes = set.split(", ");
                for cube in cubes {
                    let cube_count = cube
                        .replace(|c: char| !c.is_numeric(), "")
                        .parse::<i32>()
                        .expect("Regex error");
                    if cube.contains("red") {
                        red += cube_count
                    } else if cube.contains("green") {
                        green += cube_count
                    } else {
                        blue += cube_count
                    }
                }
                if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                    passed = false;
                }
                if red > highest_red { highest_red = red }
                if green > highest_green { highest_green = green }
                if blue > highest_blue { highest_blue = blue }
            }
            if passed {
                result1 += i;
            }
            result2 += highest_red * highest_blue * highest_green;
        }
        (result1, result2)
    }
}
