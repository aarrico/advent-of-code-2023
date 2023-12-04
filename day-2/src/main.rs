use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

//use std::time::{Instant};

const INPUT_FILE_PATH: &str = "inputs/input.txt";

#[derive(Debug, Eq, Ord)]
struct CubeSet {
    blue: u16,
    red: u16,
    green: u16,
}

impl CubeSet {
    fn parse_from_str(set_str: &str) -> Self {
        let set = set_str.split(',');
        let mut blue: u16 = 0;
        let mut red: u16 = 0;
        let mut green: u16 = 0;

        for cube in set {
            let cube_data: Vec<&str> = cube.split_whitespace().collect();

            if cube_data.len() != 2 {
                panic!("invalid cube data:\n\t{:?}", cube_data);
            }

            let count =
                match cube_data[0].parse::<u16>() {
                    Ok(count) => count,
                    Err(err) =>
                        panic!("couldn't process cube count for round {}:\n\terr: {}", set_str, err)
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

    fn parse_sets_from_str(sets_str: &str) -> Vec<CubeSet> {
        let rounds_to_parse: Vec<&str> = sets_str.split(";").collect();

        let mut sets: Vec<CubeSet> = Vec::new();

        for g in rounds_to_parse {
            sets.push(CubeSet::parse_from_str(g));
        }

        sets
    }
}

impl PartialEq<Self> for CubeSet {
    fn eq(&self, other: &Self) -> bool {
        &self.blue == &other.blue
            && &self.red == &other.red
            && &self.green == &other.green
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.red < other.red && self.blue < other.blue && self.green < other.green {
            return Some(Ordering::Less);
        } else if self.red > other.red && self.blue > other.blue && self.green > other.green {
            return Some(Ordering::Greater);
        } else if self.red > other.red || self.green > other.green || self.blue > other.blue {
            return Some(Ordering::Greater);
        } else if self == other {
            return Some(Ordering::Equal);
        }

        return Some(self.cmp(&other));
    }
}

#[derive(Debug)]
struct GameData {
    game_num: u16,
    rounds: Vec<CubeSet>,
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

fn find_sum(cubes_to_check: &CubeSet) -> u16 {
    let mut sum: u16 = 0;
    let lines = read_lines(Path::new(INPUT_FILE_PATH));

    let games = populate_game_data(lines);
    for g in games {
        if is_game_possible(&g, cubes_to_check) {
            sum += g.game_num;
        }
    }

    sum
}

fn main() {
    //12 red, 13 green, 14 blue
    let part1_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum = find_sum(&part1_cubes);
    println!("{}", sum)
}
