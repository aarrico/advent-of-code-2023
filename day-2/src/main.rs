mod cube_set;
mod game_data;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::cube_set::cube_set::CubeSet;
use crate::game_data::game_data::GameData;

const INPUT_FILE_PATH: &str = "inputs/input.txt";

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(err) => {
            println!("failed reading input: {}", err);
            panic!()
        }
    }
}

fn parse_line(to_split: &String) -> GameData {
    let [game_num_str, rounds] = to_split.split(":").collect::<Vec<&str>>()[..]
        else { panic!("malformed line found:/n/t{}", to_split) };

    if let Ok(game_num) = game_num_str[5..].parse::<u32>() {
        GameData {
            game_num,
            rounds: CubeSet::parse_sets_from_str(&rounds[1..]),
        }
    } else {
        panic!("couldn't process game num for\n\t{}\n\t", to_split)
    }
}

fn populate_game_data(lines: io::Lines<io::BufReader<File>>) -> Vec<GameData> {
    let mut games: Vec<GameData> = Vec::new();

    for line in lines {
        if let Ok(to_split) = line {
            games.push(parse_line(&to_split));
        } else {
            panic!("issue reading line!");
        }
    }

    games
}

fn is_game_possible(game_data: &GameData, cubes_to_check: &CubeSet) -> bool {
    for round in &game_data.rounds[0..] {
        if round > cubes_to_check {
            return false;
        }
    }

    return true;
}

fn find_power_of_minimum_cubes_needed(game_data: &GameData) -> u32 {
    let mut min_cubes = CubeSet {
        blue: 0,
        red: 0,
        green: 0,
    };

    for round in &game_data.rounds {
        min_cubes.set_min_cubes(round);
    }

    min_cubes.get_power()
}

fn find_sum(games: &Vec<GameData>, cubes_to_check: &Option<CubeSet>) -> u32 {
    let mut sum: u32 = 0;

    for g in games {
        if let Some(cubes_to_check) = cubes_to_check {
            if is_game_possible(&g, cubes_to_check) {
                sum += g.game_num;
            }
        } else {
            sum += find_power_of_minimum_cubes_needed(&g);
        }
    }

    sum
}

fn main() {
    let lines = read_lines(Path::new(INPUT_FILE_PATH));

    let games = populate_game_data(lines);

    let part1_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let part1_sum = find_sum(&games, &Some(part1_cubes));
    println!("part 1 sum: {}", part1_sum);

    let part2_sum = find_sum(&games, &None);
    println!("part 2 sum: {}", part2_sum);
}
