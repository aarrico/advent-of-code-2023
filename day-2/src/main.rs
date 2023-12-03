use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

const INPUT_FILE_PATH: &str = "inputs/input.txt";

#[derive(Debug)]
struct Round {
    blue: u16,
    red: u16,
    green: u16,
}

impl Round {
    fn new(round_str: &str) -> Self {
        let round = round_str.split(',');
        let mut blue: u16 = 0;
        let mut red: u16 = 0;
        let mut green: u16 = 0;

        for cube in round {
            let cube_data: Vec<&str> = cube.split_whitespace().collect();

            if cube_data.len() != 2 {
                panic!("invalid cube data:\n\t{:?}", cube_data);
            }

            let count =
                match cube_data[0].parse::<u16>() {
                    Ok(count) => count,
                    Err(err) =>
                        panic!("couldn't process cube count for round {}:\n\terr: {}", round_str, err)
                };

            match cube_data[1] {
                "red" => { red = count; }
                "blue" => { blue = count; }
                "green" => { green = count; }
                _ => panic!("bad cube color: {:?}", cube_data)
            }
        }

        Self {
            blue,
            red,
            green,
        }
    }

    fn parse_rounds(rounds_str: &str) -> Vec<Round> {
        let rounds_to_parse: Vec<&str> = rounds_str.split(";").collect();

        let mut games: Vec<Round> = Vec::new();

        for g in rounds_to_parse {
            games.push(Round::new(g));
        }

        games
    }
}

#[derive(Debug)]
struct GameData {
    game_num: u16,
    rounds: Vec<Round>,
}

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

    if let Ok(game_num) = game_num_str[5..].parse::<u16>() {
        GameData {
            game_num,
            rounds: Round::parse_rounds(&rounds[1..]),
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

fn find_sum() -> u16 {
    let mut sum: u16 = 0;
    let lines = read_lines(Path::new(INPUT_FILE_PATH));

    let games = populate_game_data(lines);
    for g in games {
        println!("{:?}", g);
    }

    sum
}

fn main() {
    let sum = find_sum();
    println!("{}", sum)
}
