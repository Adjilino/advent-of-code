use std::fs;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let races = read_input(input);

    println!("{:?}", races);

    part1(&races);
    part2(&races);
}

fn read_input(input: String) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<&str>>();

    get_races_from_lines(lines)
}

fn get_races_from_lines(lines: Vec<&str>) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    for line in lines {
        let splitted_line = line.split(":").collect::<Vec<&str>>();

        match splitted_line[0] {
            "Time" => times = get_line_values(splitted_line[1]),
            "Distance" => distances = get_line_values(splitted_line[1]),
            _ => println!("Error"),
        }
    }

    for (index, time) in times.iter().enumerate() {
        races.push(Race::new(*time, distances[index]));
    }

    races
}

fn get_line_values(line_values: &str) -> Vec<usize> {
    let value_str = line_values.split_whitespace().collect::<Vec<&str>>();

    value_str
        .iter()
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn part1(races: &Vec<Race>) {
    let mut races_number_of_ways: Vec<usize> = Vec::new();

    for race in races {
        races_number_of_ways.push(get_race_number_of_ways(race));
    }

    let mut final_score = 1;

    for n in races_number_of_ways {
        final_score *= n;
    }

    println!("Part 1 {:?}", final_score);
}

fn part2(races: &Vec<Race>) {
    let mut actual_time: String = String::new();
    let mut actual_distance: String = String::new();

    for race in races {
        actual_time = actual_time + &race.time.to_string();
        actual_distance = actual_distance + &race.distance.to_string();
    }

    // println!("{:?} {:?}", actual_time, actual_distance);

    let actual_race = Race::new(
        actual_time.parse::<usize>().unwrap(),
        actual_distance.parse::<usize>().unwrap(),
    );

    println!("{}", get_race_number_of_ways(&actual_race));
}

fn get_race_number_of_ways(race: &Race) -> usize {
    let mut first_time_record = 0;
    let mut last_time_record = 0;

    for time in 1..race.time {
        if first_time_record == 0 {
            let distance = (race.time - time) * time;

            if distance > race.distance {
                first_time_record = time;
            }
        }

        if last_time_record == 0 {
            let press_time = race.time - time;
            let distance = (race.time - press_time) * press_time;

            if distance > race.distance {
                last_time_record = press_time;
            }
        }

        if first_time_record != 0 && last_time_record != 0 {
            break;
        }
    }

    let number_of_ways = (first_time_record..=last_time_record).collect::<Vec<usize>>();

    number_of_ways.len() as usize
}
