pub mod cube_set {
    use std::cmp::Ordering;

    #[derive(Debug, Eq, Ord)]
    pub struct CubeSet {
        pub blue: u32,
        pub red: u32,
        pub green: u32,
    }

    impl CubeSet {
        pub fn parse_from_str(set_str: &str) -> Self {
            let set = set_str.split(',');
            let mut blue: u32 = 0;
            let mut red: u32 = 0;
            let mut green: u32 = 0;

            for cube in set {
                let cube_data: Vec<&str> = cube.split_whitespace().collect();

                if cube_data.len() != 2 {
                    panic!("invalid cube data:\n\t{:?}", cube_data);
                }

                let count =
                    match cube_data[0].parse::<u32>() {
                        Ok(count) => count,
                        Err(err) =>
                            panic!("couldn't process cube count for round {}:\n\terr: {}", set_str, err)
                    };

                match cube_data[1] {
                    "red" => { red = count; }
                    "blue" => { blue = count; }
                    "green" => { green = count; }
                    _ => panic!("bad cube color: {:?}", cube_data)
                }
            }

            Self {
                blue,
                red,
                green,
            }
        }

        pub fn parse_sets_from_str(sets_str: &str) -> Vec<CubeSet> {
            let rounds_to_parse: Vec<&str> = sets_str.split(";").collect();

            let mut sets: Vec<CubeSet> = Vec::new();

            for g in rounds_to_parse {
                sets.push(CubeSet::parse_from_str(g));
            }

            sets
        }

        pub fn set_min_cubes(&mut self, to_check: &CubeSet) {
            if to_check.blue > self.blue {
                self.blue = to_check.blue;
            }

            if to_check.red > self.red {
                self.red = to_check.red;
            }

            if to_check.green > self.green {
                self.green = to_check.green;
            }
        }

        pub fn get_power(self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    impl PartialEq<Self> for CubeSet {
        fn eq(&self, other: &Self) -> bool {
            &self.blue == &other.blue
                && &self.red == &other.red
                && &self.green == &other.green
        }
    }

    impl PartialOrd for CubeSet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.red < other.red && self.blue < other.blue && self.green < other.green {
                return Some(Ordering::Less);
            } else if self.red > other.red && self.blue > other.blue && self.green > other.green {
                return Some(Ordering::Greater);
            } else if self.red > other.red || self.green > other.green || self.blue > other.blue {
                return Some(Ordering::Greater);
            } else if self == other {
                return Some(Ordering::Equal);
            }

            return Some(self.cmp(&other));
        }
    }
}