use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::parse(input);
    platform.tilt(Tilt::Up);
    let res = platform.count_weights();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut platform = Platform::parse(input);
    // platform.cycle_times(1000000000);
    let res = platform.cycle_times(1_000_000_000);
    // let res = platform.count_weights();
    Some(res as u64)
}

enum Tilt {
    #[allow(dead_code)]
    Left,
    #[allow(dead_code)]
    Right,
    Up,
    #[allow(dead_code)]
    Down,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Object {
    Round,
    Cube,
    Empty,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    col: usize,
    row: usize,
}

impl Coordinate {
    fn coordinate_in_direction(&self, tilt: &Tilt) -> Option<Coordinate> {
        match tilt {
            Tilt::Right => Some(Coordinate {
                col: self.col + 1,
                row: self.row,
            }),
            Tilt::Down => Some(Coordinate {
                col: self.col,
                row: self.row + 1,
            }),
            Tilt::Left => {
                if self.col > 0 {
                    Some(Coordinate {
                        col: self.col - 1,
                        row: self.row,
                    })
                } else {
                    None
                }
            }
            Tilt::Up => {
                if self.row > 0 {
                    Some(Coordinate {
                        col: self.col,
                        row: self.row - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

struct Platform {
    rocks: Vec<Vec<Object>>,
}

impl Platform {
    fn parse(input: &str) -> Platform {
        let mut rocks: Vec<Vec<Object>> = Vec::new();
        for (_y, row) in input.lines().enumerate() {
            let mut line = Vec::new();
            for (_x, char) in row.char_indices() {
                // let coordinate = Coordinate { col: x, row: y };
                let object = match char {
                    '#' => Object::Cube,
                    'O' => Object::Round,
                    _ => Object::Empty,
                };
                line.push(object);
            }
            rocks.push(line);
        }
        Platform { rocks }
    }

    fn tilt(&mut self, direction: Tilt) {
        let mut moved = true;
        while moved {
            moved = false;
            for row in 0..self.rocks.len() {
                'innie: for col in 0..self.rocks[0].len() {
                    let this = Coordinate { col, row };
                    if self.object_at(&this) == Object::Round {
                        let coordinate = match this.coordinate_in_direction(&direction) {
                            Some(coord) => coord,
                            None => {
                                continue 'innie;
                            }
                        };

                        if coordinate.row >= self.rocks.len()
                            || coordinate.col >= self.rocks[0].len()
                        {
                            continue 'innie;
                        }

                        match self.object_at(&coordinate) {
                            Object::Round => {
                                continue 'innie;
                            }
                            Object::Cube => {
                                continue 'innie;
                            }
                            Object::Empty => {
                                self.execute_move(&this, row, coordinate);
                                moved = true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn object_at(&self, coordinate: &Coordinate) -> Object {
        self.rocks[coordinate.row][coordinate.col]
    }

    fn execute_move(&mut self, from: &Coordinate, _index: usize, to: Coordinate) {
        //moving to must be empty
        if self.rocks[to.row][to.col] != Object::Empty {
            panic!("attempt to move onto non-empty space");
        }
        self.rocks[from.row][from.col] = Object::Empty;
        self.rocks[to.row][to.col] = Object::Round;
    }

    fn count_weights(&self) -> usize {
        let mut count = 0;

        for row in 0..self.rocks.len() {
            for col in 0..self.rocks[0].len() {
                let this = Coordinate { col, row };
                let o = self.object_at(&this);
                if o == Object::Round {
                    let weight = self.rocks.len() - row;
                    count += weight;
                }
            }
        }

        count
    }

    #[allow(dead_code)]
    fn print_platform(&self) {
        for c in &self.rocks {
            for x in c {
                match x {
                    Object::Round => print!("O"),
                    Object::Cube => print!("#"),
                    Object::Empty => print!("."),
                }
            }
            println!();
        }
        println!();
    }

    fn cycle_times(&mut self, amount: usize) -> usize {
        let mut hash_states = vec![];
        let mut weights = vec![];
        hash_states.push(self.hash_state());
        weights.push(self.count_weights());

        for ticker in 1..=amount {
            self.cycle();
            let after = self.hash_state();
            let weight = self.count_weights();
            // println!("state {}, weight {}", ticker, weight);

            if let Some(first_occurence) = hash_states.iter().position(|state| *state == after) {
                let cycle_length = ticker - first_occurence;
                // println!(
                //     "occurence {}, cycle_length {}",
                //     first_occurence, cycle_length
                // );

                return weights[((amount - first_occurence) % cycle_length) + first_occurence];
            }

            hash_states.push(after);
            weights.push(weight);
        }
        *weights.last().unwrap()
    }

    fn cycle(&mut self) {
        self.tilt(Tilt::Up);
        self.tilt(Tilt::Left);
        self.tilt(Tilt::Down);
        self.tilt(Tilt::Right);
    }

    fn hash_state(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for row in &self.rocks {
            for &object in row {
                object.hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    // #[test]
    // fn test_example_p2() {
    //     let mut platform = Platform::parse(&advent_of_code::template::read_file("examples", DAY));
    //     platform.cycle_times(3);
    //     let result = Some(platform.count_weights());

    //     assert_eq!(result, Some(3));
    // }
}
