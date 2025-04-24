use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(_input: &str) -> Option<u64> {
    let mut universe = Universe::parse(_input, 1);
    universe.expand();
    let res = universe.count_shortest_paths();
    Some(res as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut universe = Universe::parse(_input, 999_999);
    universe.expand();
    let res = universe.count_shortest_paths();
    Some(res as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let mut universe =
            Universe::parse(&advent_of_code::template::read_file("examples", DAY), 9);
        universe.expand();
        let result = universe.count_shortest_paths();
        assert_eq!(Some(result), Some(1030));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Galaxy {
    coordinate: Coordinate,
    id: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Pair(Galaxy, Galaxy);

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coordinate {
    col: usize,
    row: usize,
}

impl Coordinate {
    fn manhattan_distance(&self, other: Coordinate) -> usize {
        (((self.col as isize) - (other.col as isize)).abs()
            + ((self.row as isize) - (other.row as isize)).abs()) as usize
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
    expand_row: Vec<usize>,
    expand_col: Vec<usize>,
    expand_factor: usize,
}
impl Universe {
    fn parse(input: &str, expand_factor: usize) -> Universe {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut id: usize = 1;
        let mut expand_row = Vec::new();
        let mut expand_col = Vec::new();
        let mut expand_col_guards: HashMap<usize, bool> = HashMap::new();

        for (y, row) in input.lines().enumerate() {
            let mut this_row_should_expand = true;

            for (x, char) in row.char_indices() {
                expand_col_guards.entry(x).or_insert(true);
                // if !expand_col_guards.contains_key(&x) {
                //     expand_col_guards.insert(x, true);
                // }
                if char == '#' {
                    let coordinate = Coordinate { col: x, row: y };
                    expand_col_guards.insert(x, false);
                    this_row_should_expand = false;
                    let galaxy = Galaxy { coordinate, id };
                    id += 1;
                    galaxies.push(galaxy);
                }
            }
            if this_row_should_expand {
                expand_row.push(y);
            }
        }

        for (x, should_expand) in &expand_col_guards {
            if *should_expand {
                expand_col.push(*x);
            }
        }
        Universe {
            galaxies,
            expand_col,
            expand_row,
            expand_factor,
        }
    }

    fn expand(&mut self) {
        // let slice: Vec<Coordinate>= self.galaxies.iter().map(|f| f.coordinate).collect();
        // println!("Before expansion: {:?}", slice);
        let expand_factor = self.expand_factor;
        let expand_col = self.expand_col.clone();
        let expand_row = self.expand_row.clone();

        for galaxy in &mut self.galaxies {
            let col =
                Universe::shift_expansion_left(galaxy.coordinate.col, &expand_col, expand_factor);
            let row =
                Universe::shift_expansion_down(galaxy.coordinate.row, &expand_row, expand_factor);

            let new_coordinates = Coordinate { col, row };
            galaxy.coordinate = new_coordinates;
        }
        // let slice: Vec<Coordinate>= self.galaxies.iter().map(|f| f.coordinate).collect();
        // println!("Before expansion: {:?}", slice);
        self.expand_row = vec![];
        self.expand_col = vec![];
    }

    fn shift_expansion_left(
        from_this_coord: usize,
        expand_col: &[usize],
        expand_factor: usize,
    ) -> usize {
        let expanding_earlier = expand_col
            .iter()
            .filter(|&col| *col < from_this_coord)
            .count();
        from_this_coord + (expanding_earlier * expand_factor)
    }
    fn shift_expansion_down(
        from_this_coord: usize,
        expand_row: &[usize],
        expand_factor: usize,
    ) -> usize {
        let expanding_earlier = expand_row
            .iter()
            .filter(|&row| *row < from_this_coord)
            .count();
        from_this_coord + (expanding_earlier * expand_factor)
    }

    fn count_shortest_paths(&self) -> usize {
        let pairs: Vec<Pair> = self.generate_pairs(self.galaxies.clone());
        let mut paths = Vec::new();
        for pair in pairs {
            let path: usize = pair.0.coordinate.manhattan_distance(pair.1.coordinate);
            paths.push(path);
        }
        paths.iter().sum::<usize>()
    }

    fn generate_pairs(&self, galaxies: Vec<Galaxy>) -> Vec<Pair> {
        let mut pairs = Vec::new();
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                let space1 = galaxies[i];
                let space2 = galaxies[j];
                // pairs.push(Pair(space1, space2));
                if space1.id < space2.id {
                    pairs.push(Pair(space1, space2));
                } else {
                    pairs.push(Pair(space2, space1));
                }
            }
        }

        pairs
    }
}

