pub mod d3 {
    use std::collections::HashMap;

    pub(crate) fn d3(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let mut result2 = 0;
        let mut gear_map:HashMap<(i32, i32), i32> = HashMap::new();

        for (row, line) in file.iter().enumerate() {
            let mut num = String::new();
            let mut valid = false;
            let mut gear = (0i32, 0i32);
            for (col,char) in line.chars().enumerate(){
                if char.is_numeric(){
                    num.push(char);
                    for y in -1..=1 {
                        for x in -1..=1 {
                            let offset_y = row.wrapping_add(y as usize);
                            let offset_x = col.wrapping_add(x as usize);
                            if offset_x < line.len() && offset_y < file.len() {
                                let mut chars = file.iter().nth(offset_y).unwrap().chars();
                                let symbol = chars.nth(offset_x).unwrap();
                                if "@#%$&+*-/=".contains(symbol) {
                                    valid = true;
                                    if symbol == '*'{
                                        gear = (offset_x as i32,offset_y as i32);
                                    }
                                    break
                                }
                            }
                        }
                    }
                }
                if !char.is_numeric() || col + 1 == line.len(){
                    if !num.is_empty() {
                        if valid {
                            let num = num.parse::<i32>().unwrap();
                            result1 += num;
                            if gear != (0,0){
                                if gear_map.contains_key(&gear){
                                    let value = *gear_map.get(&gear).unwrap();
                                    if value > 0 && value != i32::MAX {
                                        let ratio = value * num;
                                        result2 += ratio;
                                        gear_map.insert(gear, -1 * ratio);
                                    }else if value < 0{
                                        let substract_ratio = *gear_map.get(&gear).unwrap();
                                        result2 += substract_ratio;
                                    }
                                }
                                gear_map.insert(gear, num);
                            }
                        }
                    }
                    valid = false;
                    gear = (0,0);
                    num.clear()
                }
            }
        }
        (result1, result2)
    }
}
