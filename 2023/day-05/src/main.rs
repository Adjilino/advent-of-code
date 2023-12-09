use std::{fs, vec};

#[derive(Debug)]
struct Map {
    from: Instruction,
    to: Instruction,
}

impl Map {
    fn new(from: Instruction, to: Instruction) -> Self {
        Self { from, to }
    }
}

#[derive(Debug)]
struct Instruction {
    category: String,
    start: usize,
    end: usize,
}

impl Instruction {
    fn new(category: String, start: usize, end: usize) -> Self {
        Self {
            category,
            start,
            end,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    read_input_v1(input.as_str());
}

fn read_input_v1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut seeds: Vec<usize> = Vec::new();

    let mut maps: Vec<Map> = Vec::new();
    // while line
    // contains "map" nexts lines are related to the map
    // - split "map" and "-to-" to get categories
    // - while line is not empty
    // - - split line by whitespace
    // - - 0: category 1 number
    // - - 1: category 2 number
    // - - 2: range exclusive
    // - empty line reset category

    let mut category_1: Option<&str> = None;
    let mut category_2: Option<&str> = None;

    for line in lines {
        // seed line
        if line.contains("seeds:") {
            let seeds_string = line.split("seeds:").collect::<Vec<&str>>();

            let seeds_str = seeds_string[1]
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>();

            for seed in seeds_str {
                seeds.push(seed.parse::<usize>().unwrap());
            }

            continue;
        }

        // map line
        if line.contains("map:") {
            let map = line.split("map:").collect::<Vec<&str>>();

            let categories = map[0].trim().split("-to-").collect::<Vec<&str>>();

            category_1 = Some(categories[0]);
            category_2 = Some(categories[1]);

            continue;
        }

        // empty line
        if line.is_empty() {
            category_1 = None;
            category_2 = None;

            continue;
        }

        if category_1.as_ref().is_none() || category_2.is_none() {
            continue;
        }

        // map numbers
        let map_numbers = line.split_whitespace().collect::<Vec<&str>>();

        let category_1_number = map_numbers[1].parse::<usize>().unwrap();
        let category_2_number = map_numbers[0].parse::<usize>().unwrap();
        let range = map_numbers[2].parse::<usize>().unwrap();

        let instruction_1 = Instruction::new(
            category_1.unwrap().to_string(),
            category_1_number,
            category_1_number + range - 1,
        );
        let instruction_2 = Instruction::new(
            category_2.unwrap().to_string(),
            category_2_number,
            category_2_number + range - 1,
        );

        maps.push(Map::new(instruction_1, instruction_2));
    }

    println!("seeds: {:?}", seeds);
    // println!("maps: {:?}", maps);

    let mut maps_numbers: Vec<usize> = Vec::new();

    for seed in seeds.clone() {
        let seeds_maps = get_category_number(&maps, String::from("seed"), seed, seed);

        // println!("seed: {:?} maps: {:?}", seed, seeds_maps);
        for seed_map in seeds_maps {
            maps_numbers.push(seed_map.1);
        }
    }

    let min: usize = maps_numbers.iter().min().unwrap().clone();
    println!("maps_numbers: {:?}", min);

    let mut maps_numbers_range: Vec<usize> = Vec::new();

    for seed_index in 0..seeds.len() / 2 {
        let seed_end = seeds[seed_index * 2] + seeds[seed_index * 2 + 1] - 1;
        let seeds_maps =
            get_category_number(
            &maps,
            String::from("seed"),
            seeds[seed_index * 2],
            seed_end,
        );

        // println!(
        //     "seed range: {:?}-{:?} maps: {:?}",
        //     seeds[seed_index * 2],
        //     seed_end,
        //     seeds_maps
        // );
        for seed_map in seeds_maps {
            maps_numbers_range.push(seed_map.1);
        }
    }

    let min_range: usize = maps_numbers_range.iter().min().unwrap().clone();
    println!("maps_numbers_range: {:?}", min_range);
}

fn get_category_number(
    maps: &Vec<Map>,
    from_category: String,
    start_number: usize,
    end_number: usize,
) -> Vec<(String, usize, usize)> {
    // println!("from_category: {:?} number range: {:?}-{:?}", from_category, start_number, end_number);
    if from_category == "location" {
        return vec![(from_category, start_number, end_number)];
    }

    let mut categories_number: Vec<(String, usize, usize)> = Vec::new();

    let filtered_map = maps
        .iter()
        .filter(|map| map.from.category == from_category)
        .collect::<Vec<&Map>>();

    for map in filtered_map.clone() {
        if map.from.start <= start_number
            && map.from.end >= start_number
            && map.from.start <= end_number
            && map.from.end >= end_number
        {
            let diff_start = start_number - map.from.start;
            let diff_end = map.from.end - end_number;

            categories_number.push((
                map.to.category.clone(),
                map.to.start + diff_start,
                map.to.end - diff_end,
            ));
        } else {
            if map.from.start <= start_number && map.from.end >= start_number {
                let diff_start = start_number - map.from.start;

                categories_number.push((
                    map.to.category.clone(),
                    map.to.start + diff_start,
                    map.to.end,
                ));
            }

            if map.from.start <= end_number && map.from.end >= end_number {
                let diff_end = map.from.end - end_number;

                categories_number.push((
                    map.to.category.clone(),
                    map.to.start,
                    map.to.end - diff_end,
                ));
            }
        }
    }

    if categories_number.len() == 0 {
        categories_number.push((
            filtered_map.clone()[0].to.category.clone(),
            start_number,
            end_number,
        ));
    }

    let mut final_categories_number: Vec<(String, usize, usize)> = Vec::new();

    for category_number in categories_number.clone() {
        let category_number = get_category_number(
            maps,
            category_number.clone().0,
            category_number.clone().1,
            category_number.clone().2,
        );

        for category_number in category_number {
            final_categories_number.push(category_number);
        }
    }

    final_categories_number
}
