pub mod d15 {
    use std::ops::Index;

    pub(crate) fn d15(file: Vec<String>) -> (i32, i32){
        let mut result1 = 0;
        let mut result2 = 0;
        let mut boxes: Vec<Vec<(String,usize)>> = Vec::with_capacity(256);
        for _ in 0..255 {
            boxes.push(Vec::new());
        }

        for line in file.iter() {
            let segments:Vec<&str> = line.split(',').collect();
            for segment in segments.iter() {
                result1 += hash(segment) as i32;
                // part 2
                if segment.contains('-') {
                    remove_lens(&mut boxes, segment);
                }else if segment.contains('=') {
                    insert_lens(&mut boxes, segment);
                }
                println!("After {}", segment);
                print_boxes(&boxes);
                println!();

            }
            print_boxes(&boxes);
            for (box_num,lens_box) in boxes.iter().enumerate(){
                let mut box_power = 0;
                for (index,lens) in lens_box.iter().enumerate(){
                    box_power = (1 + box_num) * (index + 1) * lens.1
                }
                //println!("box power {}", box_power);
                result2 += box_power as i32;
            }
        }
        (result1,result2)
    }

    fn insert_lens(boxes: &mut Vec<Vec<(String, usize)>>, string: &str) {
        let value = string.chars().last().unwrap() as usize - '0' as usize;
        let hashed_value = hash(&string[0..string.len()-2]);
        //println!("Inserting {} at {}",string,hashed_value);
        let box_content = boxes.get_mut(hashed_value).unwrap();
        if box_content.is_empty(){
            box_content.push((string.parse().unwrap(),value));
        }else {
            for content in box_content.iter_mut() {
                if content.0 == string {
                    content.1 = value;
                    return;
                }
            }
            box_content.push((string.parse().unwrap(), value));

        }
    }

    fn remove_lens(boxes: &mut Vec<Vec<(String, usize)>>, string: &str) {
        let hashed_value = hash(&string[0..string.len()-1]);
        //println!("Removing element at {}",hashed_value);
        let box_content = boxes.get_mut(hashed_value).unwrap();
        if !box_content.is_empty() {
            for (i,content) in box_content.iter_mut().enumerate() {
                if content.0 == string {
                    box_content.remove(i);
                    return;
                }
            }
        }
    }

    fn hash(segment: &str) -> usize{
        let mut hashed_value = 0;
        for char in segment.chars(){
            hashed_value += char as usize;
            hashed_value *= 17;
            hashed_value %= 256;
        }
        hashed_value
    }

    fn print_boxes(boxes: &Vec<Vec<(String, usize)>>){
        for (box_num,lens_box) in boxes.iter().enumerate() {
            if !lens_box.is_empty() {
                print!("box {}: ", box_num);
                for lens in lens_box.iter(){
                    print!("[{} {}]",lens.0,lens.1);
                }
                println!();
            }
        }
    }
}
