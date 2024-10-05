pub mod d7 {
    use std::collections::HashMap;

    pub(crate) fn d7(file: Vec<String>) -> (i32, i32) {
        let mut result1 = 0;
        let result2 = 0;
        let mut hand_list = Vec::new();

        for line in file{
            println!("{:?}", line);
            let mut line =line.split(' ');

            let cards =  line.next().unwrap().to_string();
            let score = calculate_score(&cards);
            let hand:(i32,String,i32) = (
                score,
                cards,
                line.next().unwrap().parse::<i32>().expect("Parse error")
            );
            hand_list.push(hand);
            println!("{:?}", hand);
        }
        hand_list.sort_by(|a, b| a.0.cmp(&b.0));

        for (i, hand) in hand_list.iter().enumerate(){
            result1 += hand.2 * i;
        }

        (result1, result2)
    }

    fn calculate_score(card: &String) -> i32 {
        // highest to lowest
        let ranking =   ["A","K","Q","J","T","9","8","7","6","5","4","3","2"];
        //                        12  11  10   9   8   7   6   5   4   3   2   1   0
        let hand_type = ["five_of", "four_of","full_house","three_of","two_pair","one_pair","high_card"];
        //                        100000     10000        10000       1000        100         10          1
        //                          1           2           2          3           3           4          5





        let mut hash_map = HashMap::new();
        let score = 0;
        for char in card.chars(){
            if !hash_map.contains_key(&char){
                hash_map.insert(char, 0);
            }else{
                let mut count = *hash_map.get(&char).unwrap();
                count += 1;
                hash_map.insert(char, count);
            }
        }
        let size = hash_map.len();
        if size == 1 {
            let key = hash_map.keys().;
        }

        for key in hash_map{

        }

        score;
    }
}
