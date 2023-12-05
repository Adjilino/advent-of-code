use std::{fs, collections::HashMap};

const DIGIT_ARRAY: [(u32, [&str; 2]); 10] = [
    (0, ["0", "zero"]),
    (1, ["1", "one"]),
    (2, ["2", "two"]),
    (3, ["3", "three"]),
    (4, ["4", "four"]),
    (5, ["5", "five"]),
    (6, ["6", "six"]),
    (7, ["7", "seven"]),
    (8, ["8", "eight"]),
    (9, ["9", "nine"]),
];

fn main() {
    let file_text = fs::read_to_string("./input.txt").unwrap();

    let lines = file_text.split_whitespace().collect::<Vec<&str>>();
    // println!("lines! {:?}", lines);

    let digits = lines.iter().map(|line| find_digit(line)).collect::<Vec<u32>>();

    let sum = digits.iter().sum::<u32>();

    println!("sum! {:?}", sum);
}

fn find_digit(line: &str) -> u32 {
    let digit_map = HashMap::from(DIGIT_ARRAY);
    // print!("digit_map! {:?}", digit_map);

    let mut indexes : Vec<(usize, u32)> = Vec::new();

    for (digit, digits_str) in digit_map {
        for digit_str in digits_str {
            if let Some(index) = line.find(digit_str) {
                indexes.push((index, digit));

                if let Some(reverse_index) = line.rfind(digit_str) {
                    if index != reverse_index {
                        indexes.push((reverse_index, digit));
                    }
                }
            }
        }
    }

    let (_, first_digit) = indexes.iter().min().unwrap();
    let (_, last_digit) = indexes.iter().max().unwrap();

    first_digit * 10 + last_digit
}
