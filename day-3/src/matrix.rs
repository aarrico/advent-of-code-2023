use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub enum MatrixObj {
    Number(u8),
    Symbol(char),
    Period,
}

impl MatrixObj {
    pub fn is_symbol(&self) -> bool {
        if let MatrixObj::Symbol { .. } = self {
            return true;
        }

        return false;
    }

    pub fn determine_type(c: char) -> Self {
        if c.is_ascii_digit() {
            return Self::Number(c as u8);
        } else if c != '.' {
            return Self::Symbol(c);
        }

        return Self::Period;
    }
}

impl fmt::Display for MatrixObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MatrixObj::Number(num) => { write!(f, "{}", num) }
            MatrixObj::Symbol(sym) => { write!(f, "{}", sym) }
            MatrixObj::Period => write!(f, ".")
        }
    }
}

pub struct Matrix {
    pub data: Vec<Vec<MatrixObj>>,
    pub symbol_positions: Vec<(usize, usize)>,
}

impl Matrix {
    pub fn build_from_text(filename: &str) -> Self {
        let file = BufReader::new(File::open(filename).unwrap());

        let lines = file.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let mut data: Vec<Vec<MatrixObj>> = Vec::new();
        let mut symbol_positions: Vec<(usize, usize)> = Vec::new();

        for (x, chars) in lines.enumerate() {
            data.push(Vec::new());

            for (y, c) in chars.iter().enumerate() {
                let char_type = MatrixObj::determine_type(*c);

                if matches!(char_type.is_symbol(), true) {
                    symbol_positions.push((x, y))
                }

                data[x].push(char_type);
            }
        }

        Matrix {
            data,
            symbol_positions,
        }
    }

    pub fn get_dimensions(self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}
