mod matrix;
use matrix::{Matrix};

fn main() {
    let matrix = Matrix::build_from_text("input.txt");

    matrix.check_adjacent_values();
}
