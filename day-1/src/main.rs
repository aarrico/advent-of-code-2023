use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(err) => {
            println!("failed reading input: {}", err);
            panic!()
        }
    }
}


fn part_1(filename: &str) {
    let mut sum: isize = 0;

    let lines = read_lines(Path::new(&filename));

    for line in lines {
        let to_parse = match line {
            Ok(line) => line,
            Err(err) => {
                println!("{}", err);
                String::new()
            }
        };

        if !to_parse.is_empty() {
            sum += find_digits_part_1(to_parse);
        }
    }

    println!("{}", sum);
}

fn find_digits_part_1(line: String) -> isize {
    let mut all_digits = Vec::new();

    for c in line.chars() {
        if c.is_digit(10) {
            all_digits.push(c);
        }
    }

    let mut to_convert = String::new();
    let len = all_digits.len();

    if len > 0 {
        to_convert.push(all_digits[0]);
        to_convert.push(all_digits[len - 1]);
    }

    match to_convert.parse::<isize>() {
        Ok(num) => num,
        Err(_) => {
            println!("No digits found");
            0
        }
    }
}

fn part_2() {}

fn main() {
    const INPUT_FILE_PATH: &str = "inputs/input.txt";

    part_1(INPUT_FILE_PATH);
    part_2(INPUT_FILE_PATH);
}
