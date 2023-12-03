use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};
use phf::phf_map;

const MIN_WORD_LENGTH: usize = 3;
const INPUT_FILE_PATH: &str = "inputs/input.txt";

static WORD_TO_DIGIT: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9'
};

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>> where P: AsRef<Path>, {
    match File::open(filename) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(err) => {
            println!("failed reading input: {}", err);
            panic!()
        }
    }
}

// for part 2, we build a string, digit_as_word, as we loop through the line.
// we know the length of the the numbers 1 - 9 so we can start checking WORD_TO_DIGIT
// when 3 <= len(digit_as_word) <= 5 and if there's no match past 5 chars, clear
// it. we can also clear it as soon as we find a real digit as well.
fn find_digits_in_line(line: String, check_words: bool) -> Vec<char> {
    let mut all_digits = Vec::new();
    let mut str_to_check = String::new();

    for c in line.chars() {
        if c.is_digit(10) {
            all_digits.push(c);
            if check_words {
                str_to_check.clear();
            }
        } else if check_words {
            str_to_check.push(c);

            if str_to_check.len() >= MIN_WORD_LENGTH {
                if let Some(digit) = find_digit_word_in_string(&str_to_check) {
                    all_digits.push(digit);
                }
            }
        }

        if str_to_check.len() == 5 {
            str_to_check = str_to_check[1..].to_string()
        }
    }

    all_digits
}

fn find_digit_word_in_string(word: &String) -> Option<char> {
    let range_end: usize = word.len() - MIN_WORD_LENGTH + 1;

    for i in 0..range_end {
        if let Some(digit) = WORD_TO_DIGIT.get(&word[i..]) {
            return Some(*digit);
        }
    }

    None
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

fn find_sum(check_words: bool) -> u16 {
    let mut sum: u16 = 0;

    let lines = read_lines(Path::new(INPUT_FILE_PATH));

    for line in lines {
        let to_parse = match line {
            Ok(line) => line,
            Err(err) => {
                println!("{}", err);
                String::new()
            }
        };

        if !to_parse.is_empty() {
            let digits = find_digits_in_line(to_parse, check_words);
            sum += convert_to_int(digits);
        }
    }

    sum
}

fn main() {
    let start = Instant::now();
    let sum = find_sum(false);
    let duration = start.elapsed();
    println!("part 1 \n\tsum: {} \n\telapsed time: {}", sum, duration.as_micros());

    let start = Instant::now();
    let sum = find_sum(true);
    let duration = start.elapsed();
    println!("part 2 \n\tsum: {} \n\telapsed time: {}", sum, duration.as_micros());
}
