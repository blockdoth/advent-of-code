pub mod d6 {
    use regex::Regex;

    pub(crate) fn d6(file: Vec<String>) -> (i32, i32) {

        let regex = Regex::new(r"\s+").expect("pattern error");
        let result1 = regex.split(file[0].as_str())
            .skip(1)
            .map(|num| num.parse::<i32>().expect("int parse error"))
            .zip(
                regex.split(file[1].as_str())
                .skip(1)
                .map(|num| num.parse::<i32>().expect("int parse error"))
            )
            .map(|(time, distance)| {
                (0..time)
                    .map(|x| (time - x) * x)
                    .filter(|&value| value > distance)
                    .count() as i32
            })
            .product();
        let tuple = (
            regex.split(file[0].as_str()).skip(1).collect::<String>().parse::<i64>().expect("int parse error")
            ,
            regex.split(file[1].as_str()).skip(1).collect::<String>().parse::<i64>().expect("int parse error")
        );
        let result2 = (0..tuple.0)
            .filter(|&x| (tuple.0 - x) * x > tuple.1)
            .count() as i32;
        (result1, result2)
    }
}
