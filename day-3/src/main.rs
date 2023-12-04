use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum MatrixObj {
    Number(u8),
    Symbol(char),
    Period,
}

fn determine_type(c: char) -> MatrixObj {
    if c.is_ascii_digit() {
        return MatrixObj::Number(c as u8);
    } else if c != '.' {
        return MatrixObj::Symbol(c);
    }

    return MatrixObj::Period;
}

fn main() {
    let mut file = BufReader::new(File::open("input.txt").unwrap());
    let mut line = String::new();

    file.read_line(&mut line).unwrap();

    let matrix: Vec<Vec<MatrixObj>> =
        file.lines()
            .map(|line| line.unwrap().chars()
                .map(|x| determine_type(x)).collect()).collect();
}
