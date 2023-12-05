mod schematic;
use schematic::{Schematic};

fn main() {
    let matrix = Schematic::build_from_text("input.txt");

    matrix.get_part_number_sum();
}
