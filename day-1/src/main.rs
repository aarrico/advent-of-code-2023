use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::time::{Instant};

const MIN_WORD_LENGTH: usize = 3;
const MAX_WORD_LENGTH: usize = 5;

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(err) => {
            println!("failed reading input: {}", err);
            panic!()
        }
    }
}

fn find_digits_in_line(line: String, num_map: Option<&HashMap<&str, char>>) -> Vec<char> {
    let mut all_digits = Vec::new();
    let mut digit_as_word = String::new();
    let check_words: bool = !num_map.is_none();

    // for part 2, we build a string, digit_as_word, as we loop through the line.
    // we know the length of the the numbers 1 - 9 so we can start checking NUM_MAP
    // when 3 <= len(digit_as_word) <= 5 and if there's no match past 5 chars, clear
    // it. we can also clear it as soon as we find a real digit as well.
    for c in line.chars() {
        if c.is_digit(10) {
            all_digits.push(c);

            if check_words {
                digit_as_word.clear();
            }
        } else if check_words {
            digit_as_word.push(c);
            let len = digit_as_word.len();
            if len >= MIN_WORD_LENGTH && len <= MAX_WORD_LENGTH {
                if let Some(num) = num_map.unwrap().get(digit_as_word.as_str()) {
                    all_digits.push(*num);
                }
            } else if len > 5 {
                digit_as_word.clear();
            }
        }
    }

    all_digits
}

fn convert_to_int(all_digits: Vec<char>) -> u16 {
    let mut to_convert = String::new();
    let len = all_digits.len();

    if len > 0 {
        to_convert.push(all_digits[0]);
        to_convert.push(all_digits[len - 1]);
    }

    match to_convert.parse::<u16>() {
        Ok(num) => num,
        Err(_) => {
            println!("No digits found");
            0
        }
    }
}

fn find_sum(filename: &str, num_map: Option<&HashMap<&str, char>>) -> u16 {
    let mut sum: u16 = 0;

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
            let digits = find_digits_in_line(to_parse, num_map);
            sum += convert_to_int(digits);
        }
    }

    sum
}

fn main() {
    const INPUT_FILE_PATH: &str = "inputs/input.txt";

    let num_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    // let start = Instant::now();
    // let sum = find_sum(INPUT_FILE_PATH, None);
    // let duration = start.elapsed();
    // println!("part 1 \n\tsum: {} \n\telapsed time: {}", sum, duration.as_micros());

    let start = Instant::now();
    let sum = find_sum(INPUT_FILE_PATH, Some(&num_map));
    let duration = start.elapsed();
    println!("part 2 \n\tsum: {} \n\telapsed time: {}", sum, duration.as_micros());
}
