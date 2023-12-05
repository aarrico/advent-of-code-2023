mod schematic_obj;
mod position;

use schematic_obj::SchematicObj;
use position::Position;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub struct Schematic {
    matrix: Vec<Vec<SchematicObj>>,
    symbol_positions: HashSet<Position>,
    row_count: usize,
    col_count: usize,
}

impl Schematic {
    pub fn build_from_text(filename: &str) -> Self {
        let file = BufReader::new(File::open(filename).unwrap());

        let lines = file.lines()
            .map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let mut matrix: Vec<Vec<SchematicObj>> = Vec::new();
        let mut symbol_positions: HashSet<Position> = HashSet::new();

        for (x, chars) in lines.enumerate() {
            matrix.push(Vec::new());

            for (y, c) in chars.iter().enumerate() {
                let char_type = SchematicObj::determine_type(*c);

                if matches!(char_type.is_symbol(), true) {
                    symbol_positions.insert(Position::new(x, y));
                }

                matrix[x].push(char_type);
            }
        }

        let row_count = (&matrix).len();
        let col_count = (&matrix)[0].len();

        Schematic {
            matrix,
            symbol_positions,
            row_count,
            col_count,
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.row_count - 1, self.col_count - 1)
    }

    fn get_digit_positions(&self) -> HashSet<Position> {
        let mut adj_pos: HashSet<Position> = HashSet::new();

        let (x_dim, y_dim) = self.get_dimensions();

        for sym_pos in &self.symbol_positions {
            let (sym_x, sym_y) = sym_pos.get();

            for x in position::get_range(sym_x, x_dim - 1) {
                for y in position::get_range(sym_y, y_dim - 1) {
                    if SchematicObj::is_number(&self.matrix[x][y]) {
                        adj_pos.insert(Position::new(x, y));
                    }
                }
            }
        }

        adj_pos
    }

    pub fn get_part_number_sum(self) -> usize {
        let mut sum: usize = 0;
        let digit_positions = self.get_digit_positions();

        for pos in &digit_positions {
            let (x, y) = pos.get();
            let (_, y_max) = self.get_dimensions();

            let num_str = build_number_str(&self.matrix[x], y, y_max);

            if let Ok(num) = num_str.parse::<usize>() {
                sum += num;
            } else {
                panic!("bug in building number string\n\tfound: {}", num_str);
            }
        }

        sum
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.matrix {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}

fn build_number_str(row: &Vec<SchematicObj>, y_initial: usize, y_max: usize) -> String {
    let mut prev = &row[y_initial];
    let mut next = &row[y_initial];

    let mut prev_y = y_initial;
    let mut next_y = y_initial;
    let mut num_str = prev.get_value().to_string();

    loop {
        if (prev.is_period() || prev_y == 0) && (next.is_period() || next_y == y_max) {
            break;
        }

        if prev_y > 0 {
            prev_y -= 1;
            prev = &row[prev_y];

            if prev.is_number() {
                num_str = format!("{}{num_str}", row[prev_y].get_value());
            }
        }

        if next_y < y_max {
            next_y += 1;
            next = &row[next_y];

            if prev.is_number() {
                num_str.push(row[prev_y].get_value());
            }
        }
    }

    num_str
}
