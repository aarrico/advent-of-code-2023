use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

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

fn find_sum() -> u16 {
    1
}

fn main() {

    let sum = find_sum();
    println!("{}", sum)
}
