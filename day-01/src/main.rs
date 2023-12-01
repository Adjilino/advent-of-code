use std::fs;

fn main() {
    let file_text = fs::read_to_string("./input.txt").unwrap();

    let lines = file_text.split_whitespace();
    // println!("lines! {:?}", lines);

    let digits = lines.map(|s| {
        s.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    })
    .map(|l| l.first().unwrap() * 10 + l.last().unwrap())
    .collect::<Vec<u32>>();
    // println!("digits! {:?}", digits);

    let sum = digits.iter().sum::<u32>();

    println!("sum! {:?}", sum);
}
