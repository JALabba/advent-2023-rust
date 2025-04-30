use std::collections::HashSet;

use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };

advent_of_code::solution!(21);

pub fn part_one(_input: &str) -> Option<u64> {
    let (grid, start) = parse_input(_input);
    let size = _input.lines().count() as u64;
    let res = calculate_reachable_coords(&grid, &start, 64, size);
    Some(res as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (grid, start) = parse_input(_input);

    // Part 2:
    // Solving part 2 depends on analyzing the input. This is not a generic solve.
    // The goal is the same as step 1, but in this many steps:
    let steps = 26_501_365;
    // steps = 202300 * 131 * 65;

    //There are assumptions we make about the input to make this work.
    // assume it is square
    let (xmax, ymax) = parse_bounds(_input);
    assert!(xmax == ymax);
    let size = xmax as u64;

    // assume it is odd sized
    assert!(size % 2 == 1);

    //Assume start.row and start.col is empty, and that the edges are empty
    // and that there is an empty diamond shape in the square, also empty
    /*
    ...........
    .##.....##.
    .#.......#.
    ....#.#....
    ...##.##...
    .....S.....
    ...##.##...
    ....#.#....
    .#.......#.
    .##.....##.
    ...........
    */

    let half = size / 2;
    let results = [half, half + size, half + size * 2]
        .par_iter()
        .map(|s| { calculate_reachable_coords(&grid, &start, *s, size) })
        .collect::<Vec<_>>();
    let n = steps / size;

    // let delta0 = results[0];
    // let delta1 = results[1] - delta0;
    // let delta2 = results[2] - 2 * results[1] - delta0;

    // Some(delta0 + delta1 * n + delta2 * ((n * (n - 1)) / 2) )

    let a0 = results[0];
    let a1 = results[1];
    let a2 = results[2];

    let b0 = a0;
    let b1 = a1.checked_sub(a0).unwrap();
    let b2 = a2.checked_sub(a1).unwrap();

    let c1 = b1.checked_mul(n).unwrap();
    // let c2 = ((n * (n - 1)) / 2) * (b2 - b1);
    let c2 = n
        .checked_mul(n.checked_sub(1).unwrap())
        .unwrap()
        .checked_div(2)
        .unwrap()
        .checked_mul(b2.checked_sub(b1).unwrap())
        .unwrap();
    Some(b0 + c1 + c2)
    // Some(calculate_infinite_coords(&grid, &start, steps, size) as u64)
    // Some(solve(_input) as u64)
    // Some(part_2(_input) as u64)
    // None
}

fn iterate_positions(grid: &HashSet<Coord>, size: u64, set: &mut HashSet<Coord>) {
    let mut new_set = HashSet::new();
    for pos in set.iter().cloned() {
        for neighbor in pos.get_neighbors() {
            let bounded_pos = Coord(
                neighbor.0.rem_euclid(size as i64),
                neighbor.1.rem_euclid(size as i64)
            );
            if !grid.contains(&bounded_pos) {
                new_set.insert(neighbor);
            }
        }
    }
    *set = new_set;
}

fn calculate_reachable_coords(grid: &HashSet<Coord>, start: &Coord, steps: u64, size: u64) -> u64 {
    let mut set: HashSet<Coord> = HashSet::new();
    set.insert(*start);

    for _ in 0..steps {
        iterate_positions(grid, size, &mut set);
    }
    set.len() as u64
}

// fn calculate_infinite_coords(
//     grid: &HashSet<Coord>,
//     start: &Coord,
//     steps: usize,
//     size: usize
// ) -> usize {
//     let mut set: HashSet<Coord> = HashSet::new();
//     let mut results = Vec::new();
//     set.insert(*start);

//     if steps < size * 3 + size / 2 {
//         for _ in 0..steps {
//             iterate_positions(grid, size, &mut set);
//         }
//     } else {
//         for i in 0..steps {
//             iterate_positions(grid, size, &mut set);

//             let to_side = size / 2;
//             let full_squares_distance = size * results.len();
//             let t = to_side + full_squares_distance;
//             if i == t {
//                 dbg!(t);
//                 dbg!(set.len());
//                 results.push(set.len());

//                 if results.len() == 3 {
//                     // let n = steps.div(size) as u64;

//                     // let delta0 = results[0];
//                     // let delta1 = results[1] - delta0;
//                     // let delta2 = results[2] - 2 * results[1] - delta0;

//                     // return Some(delta0
//                     //     + delta1 * n
//                     //     + delta2 * (n * (n-1)/2) );

//                     let n = steps.checked_div(size).unwrap() as usize;

//                     let a0 = results[0];
//                     let a1 = results[1];
//                     let a2 = results[2];

//                     let b0 = a0;
//                     let b1 = a1.checked_sub(a0).unwrap();
//                     let b2 = a2.checked_sub(a1).unwrap();

//                     let c1 = b1.checked_mul(n).unwrap();
//                     // let c2 = ((n * (n - 1)) / 2) * (b2 - b1);
//                     let c2 = n
//                         .checked_mul(n.checked_sub(1).unwrap())
//                         .unwrap()
//                         .checked_div(2)
//                         .unwrap()
//                         .checked_mul(b2.checked_sub(b1).unwrap())
//                         .unwrap();
//                     return b0 + c1 + c2;
//                 }
//             }
//         }
//     }

//     set.len()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (grid, start) = parse_input(input);
        let size = input.lines().count() as u64;
        let result = calculate_reachable_coords(&grid, &start, 6, size);
        assert_eq!(Some(result), Some(16));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn test_part_two_naive() {
    //     let binding = advent_of_code::template::read_file("examples", DAY);
    //     let input = binding.as_str();
    //     let (grid, start) = parse_input(input);
    //     let size = input.lines().count() as u64;
    //     let result = calculate_reachable_coords(&grid, &start, 6, size);
    //     assert_eq!(result, 16);
    //     let result = calculate_reachable_coords(&grid, &start, 10, size);
    //     assert_eq!(result, 50);
    //     let result = calculate_reachable_coords(&grid, &start, 50, size);
    //     assert_eq!(result, 1594);
    //     let result = calculate_reachable_coords(&grid, &start, 51, size);
    //     assert_eq!(result, 1648);
    //     let result = calculate_reachable_coords(&grid, &start, 100, size);
    //     assert_eq!(result, 6536);
    //     // At this point, the example input is taking over ten seconds
    //     // meaning this approach is wrong.
    //     // let result = calculate_reachable_coords(&grid, &start, 500, size);
    //     // assert_eq!(result, 167004);
    // }

    // #[test]
    // fn test_part_two_infinite() {
    //     let binding = advent_of_code::template::read_file("examples", DAY);
    //     let input = binding.as_str();
    //     let (grid, start) = parse_input(input);
    //     let size = input.lines().count();
    //     let result = calculate_infinite_coords(&grid, &start, 6, size);
    //     assert_eq!(result, 16);
    //     let result = calculate_infinite_coords(&grid, &start, 10, size);
    //     assert_eq!(result, 50);
    //     let result = calculate_infinite_coords(&grid, &start, 51, size);
    //     assert_eq!(result, 1648);
    //     let result = calculate_infinite_coords(&grid, &start, 104, size);
    //     assert_eq!(result, 7873);
    //     // let result = calculate_reachable_coords(&grid, &start, 500, size);
    //     // assert_eq!(result, 167004);
    // }
}

fn parse_input(_input: &str) -> (HashSet<Coord>, Coord) {
    let mut start: Coord = Coord(0, 0);
    (
        _input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .filter_map(|(x, char)| {
                        match char {
                            '#' => { Some(Coord(x as i64, y as i64)) }
                            'S' => {
                                start = Coord(x as i64, y as i64);
                                None
                            }
                            _ => None,
                        }
                    })
                    .collect::<Vec<Coord>>()
            })
            .flatten()
            .collect::<HashSet<Coord>>(),
        start,
    )
}

fn parse_bounds(_input: &str) -> (usize, usize) {
    (_input.lines().count(), _input.lines().next().unwrap().len())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord(i64, i64);

impl Coord {
    pub fn get_neighbors(&self) -> Vec<Coord> {
        let &Coord(x, y) = self;
        vec![
            Coord(x.checked_sub(1).unwrap(), y),
            Coord(x.checked_add(1).unwrap(), y),
            Coord(x, y.checked_sub(1).unwrap()),
            Coord(x, y.checked_add(1).unwrap())
        ]
    }
}
