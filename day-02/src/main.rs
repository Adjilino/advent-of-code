use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // println!("input: {}", input);

    part1(&input.clone());
    part2(&input.clone());
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    // println!("{:?}", lines);

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut possible_game_subsets: Vec<u32> = Vec::new();

    for line in lines {
        let game = get_game_subsets(line);
        // println!("{:?}", x);

        let max_red = get_max_color(game.clone(), "red");
        let max_green = get_max_color(game.clone(), "green");
        let max_blue = get_max_color(game.clone(), "blue");

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            possible_game_subsets.push(game.0);
        }
    }

    let sum_games: u32 = possible_game_subsets.iter().sum();

    println!("possible_game_subsets: {:?}", sum_games);
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    // println!("{:?}", lines);

    let mut games_power: Vec<u32> = Vec::new();

    for line in lines {
        let game = get_game_subsets(line);
        // println!("{:?}", x);

        let max_red = get_max_color(game.clone(), "red");
        let max_green = get_max_color(game.clone(), "green");
        let max_blue = get_max_color(game.clone(), "blue");

        println!("max_red: {:?}", max_red);
        println!("max_green: {:?}", max_green);
        println!("max_blue: {:?}", max_blue);
        games_power.push(max_red * max_green * max_blue);
    }

    let sum_games: u32 = games_power.iter().sum();

    println!("possible_game_subsets: {:?}", sum_games);
}

fn get_max_color(game: (u32, Vec<(u32, &str)>), color: &str) -> u32 {
    let max = game.1
        .iter()
        .filter(|x| x.1 == color)
        .max().unwrap();

    max.0
}

fn get_max_possible_color(game: (u32, Vec<(u32, &str)>), color: &str, max: u32) -> u32 {
    let max = game.1
        .iter()
        .filter(|x| x.1 == color && x.0 <= max)
        .max().unwrap();

    max.0
}

fn get_game_subsets(input: &str) -> (u32, Vec<(u32, &str)>) {
    let game = input.split(":").collect::<Vec<&str>>();
    let game_id = game[0].replace("Game ", "");
    let game_subsets_str = game[1].split(";").collect::<Vec<&str>>();
    // println!("game_str: {:?}", game_subsets_str);
    let mut game_subset_numbers = Vec::new();

    for game_subset_str in game_subsets_str {
        let game_subset = game_subset_str.split(",").collect::<Vec<&str>>();

        for subset in game_subset {
            game_subset_numbers.push(get_number_color(subset));
        }
    }

    (game_id.parse().unwrap(), game_subset_numbers)
}

fn get_number_color(input: &str) -> (u32, &str) {
    let data = input.trim().split(" ").collect::<Vec<&str>>();

    (data[0].parse().unwrap(), data[1])
}
