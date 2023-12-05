use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let lines_in_chars = input_to_lines_in_chars(input);

    // println!("{:?}", lines_in_chars);

    let mut sum = 0;

    for (i_line, line) in lines_in_chars.iter().enumerate() {
        let mut number = 0;
        let mut have_adjacent_symbol = false;

        for (i_char, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                if number > 0 {
                    number *= 10;
                }

                number += c.to_digit(10).unwrap();

                if have_adjacent_symbol {
                    continue;
                }

                have_adjacent_symbol =
                    is_adjacent_to_symbol(lines_in_chars.clone(), i_line, i_char);
            } else {
                if have_adjacent_symbol {
                    sum += number;
                }

                number = 0;
                have_adjacent_symbol = false;
            }
        }

        if have_adjacent_symbol {
            sum += number;
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let lines_in_chars = input_to_lines_in_chars(input);

    // println!("{:?}", lines_in_chars);

    let mut power = 0;

    for (i_line, line) in lines_in_chars.iter().enumerate() {
        for (i_char, c) in line.iter().enumerate() {
            if c.clone() == '*' {
                let adjacent_numbers =
                    find_adjacent_numbers(lines_in_chars.clone(), i_line, i_char);

                println!("adjacent_numbers: {:?}", adjacent_numbers);
                if adjacent_numbers.len() == 2 {
                    power += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }

    println!("Part 2: {}", power);
}

fn input_to_lines_in_chars(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn is_adjacent_to_symbol(lines_in_chars: Vec<Vec<char>>, i_line: usize, i_char: usize) -> bool {
    // -1, 0, 1
    // if i 0 and i_c 0 (skip)
    const I: [isize; 3] = [-1, 0, 1];
    let mut have_adjacent_symbol = false;

    for i_l in I {
        for i_c in I {
            if i_l == 0 && i_c == 0 {
                continue;
            }

            if i_line as isize + i_l < 0 || i_char as isize + i_c < 0 {
                continue;
            }

            if i_line as isize + i_l >= lines_in_chars.len() as isize
                || i_char as isize + i_c >= lines_in_chars[i_line].len() as isize
            {
                continue;
            }

            let index_line = i_line as isize + i_l;
            let index_char = i_char as isize + i_c;

            let char_to_verify = lines_in_chars[index_line as usize][index_char as usize];

            if !char_to_verify.is_digit(10) && char_to_verify != '.' {
                have_adjacent_symbol = true;
                break;
            }
        }

        if have_adjacent_symbol {
            break;
        }
    }

    have_adjacent_symbol
}

fn find_adjacent_numbers(lines_in_chars: Vec<Vec<char>>, i_line: usize, i_char: usize) -> Vec<u32> {
    const I: [isize; 3] = [-1, 0, 1];
    let mut adjacent_numbers = Vec::new();

    for i_l in I {
        let mut last_was_digit = false;

        for i_c in I {
            if i_l == 0 && i_c == 0 {
                last_was_digit = false;
                continue;
            }

            if i_line as isize + i_l < 0 || i_char as isize + i_c < 0 {
                continue;
            }

            if i_line as isize + i_l >= lines_in_chars.len() as isize
                || i_char as isize + i_c >= lines_in_chars[i_line].len() as isize
            {
                continue;
            }

            let index_line = (i_line as isize + i_l) as usize;
            let index_char = (i_char as isize + i_c) as usize;

            let char_to_verify = lines_in_chars[index_line][index_char];

            if char_to_verify.is_digit(10) && !last_was_digit {
                last_was_digit = true;
                let number = get_number(lines_in_chars.clone(), index_line, index_char);

                adjacent_numbers.push(number);
            } else if !char_to_verify.is_digit(10) {
                last_was_digit = false;
            }
        }
    }

    adjacent_numbers
}

fn get_number(lines_in_chars: Vec<Vec<char>>, i_line: usize, i_char: usize) -> u32 {
    let mut number = 0;
    let mut start_of_number = i_char;

    if start_of_number > 0 {
        while lines_in_chars[i_line][start_of_number - 1].is_digit(10) {
            start_of_number -= 1;

            if start_of_number == 0 {
                break;
            }
        }
    }

    while lines_in_chars[i_line][start_of_number].is_digit(10) {
        number *= 10;
        number += lines_in_chars[i_line][start_of_number]
            .to_digit(10)
            .unwrap();

        start_of_number += 1;

        if start_of_number >= lines_in_chars[i_line].len() {
            break;
        }
    }

    number
}
