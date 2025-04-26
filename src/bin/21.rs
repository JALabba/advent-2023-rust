use std::collections::HashSet;

advent_of_code::solution!(21);

pub fn part_one(_input: &str) -> Option<u64> {
    let grid: HashSet<Coord> = parse_input(_input);
    let start = parse_start_coord(_input);
    let size = _input.lines().count();
    let res = calculate_reachable_coords(&grid, &start, 64, size);
    Some(res as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    // let grid: HashSet<Coord> = parse_input(_input);
    // let start = parse_start_coord(_input);

    // Part 2:
    // Solving part 2 depends on analyzing the input. This is not a generic solve.
    // The goal is the same as step 1, but in this many steps:
    // let steps = 26_501_365;
    // steps = 202300 * 131 * 65;

    //There are assumptions we make about the input to make this work.
    // assume it is square
    // let (xmax, ymax) = parse_bounds(_input);
    // assert!(xmax == ymax);
    // let size = xmax;

    // assume it is odd sized
    // assert!(size % 2 == 1);

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
    // Some(calculate_infinite_coords(&grid, &start, steps, size) as u64)
    Some(solve(_input) as u64)
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

//     for i in 0..steps {
//         let mut new_set = HashSet::new();
//         for pos in &set {
//             for neighbor in pos.get_neighbors() {
//                 let bounded_pos = Coord(
//                     neighbor.0.rem_euclid(size as i64),
//                     neighbor.1.rem_euclid(size as i64)
//                 );
//                 if !grid.contains(&bounded_pos) {
//                     new_set.insert(neighbor);
//                 }
//             }
//         }
//         set = new_set;

//         let to_side = size / 2;
//         let full_squares_distance = size * results.len();
//         let t = to_side + full_squares_distance;
//         if i == t {
//             dbg!(t);
//             dbg!(set.len());
//             results.push(set.len());

//             if results.len() == 3 {
//                 // let n = steps.div(size) as u64;

//                 // let delta0 = results[0];
//                 // let delta1 = results[1] - delta0;
//                 // let delta2 = results[2] - 2 * results[1] - delta0;

//                 // return Some(delta0
//                 //     + delta1 * n
//                 //     + delta2 * (n * (n-1)/2) );

//                 let n = (steps / size) as usize;

//                 let a0 = results[0];
//                 let a1 = results[1];
//                 let a2 = results[2];

//                 let b0 = a0;
//                 let b1 = a1 - a0;
//                 let b2 = a2 - a1;
//                 return b0 + b1 * n + ((n * (n - 1)) / 2) * (b2 - b1);
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
        let grid = parse_input(input);
        let start = parse_start_coord(input);
        let size = input.lines().count();
        let result = calculate_reachable_coords(&grid, &start, 6, size);
        assert_eq!(Some(result), Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_infinite() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let input = binding.as_str();
        let grid = parse_input(input);
        let start = parse_start_coord(input);
        let size = input.lines().count();
        let result = calculate_reachable_coords(&grid, &start, 6, size);
        assert_eq!(result, 16);
        let result = calculate_reachable_coords(&grid, &start, 10, size);
        assert_eq!(result, 50);
        let result = calculate_reachable_coords(&grid, &start, 50, size);
        assert_eq!(result, 1594);
        let result = calculate_reachable_coords(&grid, &start, 100, size);
        assert_eq!(result, 6536);
        // At this point, the example input is taking over ten seconds
        // meaning this approach is wrong.
        // let result = calculate_reachable_coords(&grid, &start, 500, size);
        // assert_eq!(result, 167004);
    }
}

fn calculate_reachable_coords(
    grid: &HashSet<Coord>,
    start: &Coord,
    steps: u64,
    size: usize
) -> usize {
    let mut set: HashSet<Coord> = HashSet::new();
    set.insert(*start);

    for _ in 0..steps {
        let mut new_set = HashSet::new();
        for pos in set {
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
        set = new_set;
    }

    set.len()
}

fn parse_input(_input: &str) -> HashSet<Coord> {
    _input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter_map(|(x, char)| {
                    match char {
                        '#' => { Some(Coord(x as i64, y as i64)) }
                        _ => None,
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .flatten()
        .collect::<HashSet<Coord>>()
}

fn parse_start_coord(_input: &str) -> Coord {
    let (y, line) = _input
        .lines()
        .enumerate()
        .find(|&line| line.1.contains("S"))
        .unwrap();
    let (x, _) = line
        .char_indices()
        .find(|&ch| ch.1 == 'S')
        .unwrap();
    Coord(x as i64, y as i64)
}

// fn parse_bounds(_input: &str) -> (usize, usize) {
//     (_input.lines().count(), _input.lines().next().unwrap().len())
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord(i64, i64);

impl Coord {
    pub fn get_neighbors(&self) -> Vec<Coord> {
        let &Coord(x, y) = self;
        vec![
            Coord(x.saturating_sub(1), y),
            Coord(x.saturating_add(1), y),
            Coord(x, y.saturating_sub(1)),
            Coord(x, y.saturating_add(1))
        ]
    }
}


/// https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day21/part2.rs
///
fn count_positions(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert(start);

    for _ in 0..steps {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for position in positions {
            let (y, x) = position;
            if y > 0 && map[y - 1][x] == '.' {
                new_positions.insert((y - 1, x));
            }
            if y < map.len() - 1 && map[y + 1][x] == '.' {
                new_positions.insert((y + 1, x));
            }
            if x > 0 && map[y][x - 1] == '.' {
                new_positions.insert((y, x - 1));
            }
            if x < map[y].len() - 1 && map[y][x + 1] == '.' {
                new_positions.insert((y, x + 1));
            }
        }
        positions = new_positions;
    }
    positions.len()
}

pub fn solve(input: &str) -> usize {
    let mut starting_point = (0, 0);
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        starting_point = (y, x);
                        '.'
                    } else {
                        char
                    }
                })
                .collect()
        })
        .collect();

    let map_size = map.len();
    let grid_size = 26501365 / map_size - 1;

    let even_maps_in_grid = ((grid_size + 1) / 2 * 2).pow(2);
    let odd_maps_in_grid = (grid_size / 2 * 2 + 1).pow(2);

    let odd_points_in_map = count_positions(&map, starting_point, map_size * 2 + 1);
    let even_points_in_map = count_positions(&map, starting_point, map_size * 2);

    let total_points_fully_in_grid =
        odd_points_in_map * odd_maps_in_grid + even_points_in_map * even_maps_in_grid;

    let corner_top = count_positions(&map, (map_size - 1, starting_point.1), map_size - 1);
    let corner_right = count_positions(&map, (starting_point.0, 0), map_size - 1);
    let corner_bottom = count_positions(&map, (0, starting_point.1), map_size - 1);
    let corner_left = count_positions(&map, (starting_point.0, map_size - 1), map_size - 1);

    let total_points_in_grid_corners = corner_top + corner_right + corner_bottom + corner_left;

    let small_diag_top_right = count_positions(&map, (map_size - 1, 0), map_size / 2 - 1);
    let small_diag_bottom_right = count_positions(&map, (0, 0), map_size / 2 - 1);
    let small_diag_bottom_left = count_positions(&map, (0, map_size - 1), map_size / 2 - 1);
    let small_diag_top_left = count_positions(&map, (map_size - 1, map_size - 1), map_size / 2 - 1);

    let total_points_in_small_diags = (grid_size + 1)
        * (small_diag_top_right
            + small_diag_bottom_right
            + small_diag_bottom_left
            + small_diag_top_left);

    let big_diag_top_right = count_positions(&map, (map_size - 1, 0), map_size * 3 / 2 - 1);
    let big_diag_bottom_right = count_positions(&map, (0, 0), map_size * 3 / 2 - 1);
    let big_diag_bottom_left = count_positions(&map, (0, map_size - 1), map_size * 3 / 2 - 1);
    let big_diag_top_left =
        count_positions(&map, (map_size - 1, map_size - 1), map_size * 3 / 2 - 1);

    let total_points_in_big_diags = grid_size
        * (big_diag_top_right + big_diag_bottom_right + big_diag_bottom_left + big_diag_top_left);

    let total_points_in_diag = total_points_in_small_diags + total_points_in_big_diags;

    total_points_fully_in_grid + total_points_in_grid_corners + total_points_in_diag
}
