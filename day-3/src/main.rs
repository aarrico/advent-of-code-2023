mod matrix_data;
use matrix_data::{MatrixData};

fn main() {
    let matrix = MatrixData::build_from_text("input.txt");

    matrix.get_part_number_sum();
}
