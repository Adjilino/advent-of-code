use std::{collections::HashMap, fs};

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(id: u32, winning_numbers: Vec<u32>, my_numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            my_numbers,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let cards = get_cards(get_lines(input));

    let mut sum = 0;

    for card in cards {
        let mut card_points = 0;

        for my_number in card.my_numbers {
            if card.winning_numbers.contains(&my_number) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }

        sum += card_points;
    }

    println!("part 1 {}", sum);
}

fn part2(input: &str) {
    let cards = get_cards(get_lines(input));

    let mut card_copies_map: HashMap<u32, u32> = HashMap::new();
    let mut sum = 0;

    for card in cards {
        let mut number_matching = 0;

        for my_number in card.my_numbers {
            if card.winning_numbers.contains(&my_number) {
                number_matching += 1;
            }
        }

        let mut multipier = 1;

        if card_copies_map.contains_key(&card.id) {
            let copies = card_copies_map.get(&card.id).unwrap();
            multipier = copies + 1;
        }

        sum += multipier;

        if number_matching > 0 {
            for increment_id in 1..=number_matching {
                let card_id = card.id + increment_id;

                if card_copies_map.contains_key(&card_id) {
                    let card_copies = card_copies_map.get(&card_id).unwrap();

                    card_copies_map.insert(card_id, card_copies + multipier);
                } else {
                    card_copies_map.insert(card_id, multipier);
                }
            }
        }
    }

    println!("part 2 {}", sum);
}

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_cards(lines: Vec<&str>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        cards.push(get_card(line));
    }

    cards
}

fn get_card(line: &str) -> Card {
    let mut winning_numbers: Vec<u32> = Vec::new();
    let mut my_numbers: Vec<u32> = Vec::new();

    let card_splitted = line.split(":").collect::<Vec<&str>>();

    let id = card_splitted[0].split_whitespace().collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let numbers = card_splitted[1].split("|").collect::<Vec<&str>>();

    for winning_number in numbers[0].trim().split_whitespace() {
        winning_numbers.push(winning_number.parse::<u32>().unwrap());
    }

    for my_number in numbers[1].trim().split_whitespace() {
        my_numbers.push(my_number.parse::<u32>().unwrap());
    }

    Card::new(id, winning_numbers, my_numbers)
}
