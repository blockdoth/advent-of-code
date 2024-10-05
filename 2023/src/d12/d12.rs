pub mod d12 {

    pub(crate) fn d12(file: Vec<String>) -> (i32, i32) {
        let result1 = 0;
        let result2 = 0;

        for line in file.iter(){
            let split: Vec<&str> = line.split(' ').collect();
            let springs = split[0];
            let encodings: Vec<i32> = split[1].split(',').map(|a| a.parse::<i32>().unwrap()).collect();
            //check certain arrangements
            let mut spring_groups = Vec::new();

            let mut buffer = Vec::new();
            for char in springs.chars(){
                if char == '.' {
                    if !buffer.is_empty() {
                        spring_groups.push(buffer.clone());
                    }
                    buffer.clear();
                }else{
                    buffer.push(char);
                }
            }
            if !buffer.is_empty() {
                spring_groups.push(buffer);
            }

            println!("{:?}", spring_groups);
        }



        (result1, result2)
    }
}
