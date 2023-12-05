use std::ops::Range;

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_adjacent_positions(&self, x_upper_bound: usize, y_upper_bound: usize) -> Vec<Self> {
        let mut adj_pos: Vec<Self> = Vec::new();

        for x in get_range(self.x, x_upper_bound) {
            for y in get_range(self.y, y_upper_bound) {
                adj_pos.push(Position { x, y });
            }
        }

        adj_pos
    }
}

fn get_range(val: usize, upper_bound: usize) -> Range<usize> {
    if val == 0 {
        return 0..(val + 1);
    } else if val == upper_bound {
        return (val - 1)..(upper_bound + 1);
    }

    (val - 1)..(val + 1)
}