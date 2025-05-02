use std::collections::{ HashMap, HashSet, VecDeque };

advent_of_code::solution!(23);

pub fn part_one(_input: &str) -> Option<u64> {
    let grid = Grid::parse(_input);
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: grid.max_x - 2, y: grid.max_y - 1 };

    let mut q: VecDeque<(Coord, usize, HashSet<Coord>)> = VecDeque::new();
    let mut max = 0;

    q.push_back((start, 0, HashSet::from([start])));

    while let Some((pos, cost, mut seen)) = q.pop_front() {
        if pos == end {
            max = cost.max(max);
            continue;
        }

        let neighbours1 = pos.get_neighbors(&grid);
        for (dir, coord) in neighbours1 {
            let tile = grid.get(&coord).unwrap();
            match tile {
                Tile::Path => {
                    if seen.insert(coord) {
                        q.push_back((coord, cost + 1, seen.clone()))
                    }
                },
                Tile::Slope(direction) => {
                    if dir == direction && seen.insert(coord) {
                        q.push_back((coord, cost + 1, seen.clone()))
                    }
                },
                Tile::Forest => unreachable!(),
            }
        }
    }

    Some(max as u64)
}



fn longest(from: Coord, to: Coord, map: &HashMap<Coord, HashMap<Coord, usize>>) -> usize {
    let mut q = VecDeque::new();
    let mut max = 0;

    q.push_back((from, 0, HashSet::from([from])));

    while let Some((pos, cost, seen)) = q.pop_front() {
        if pos == to {
            max = cost.max(max);
            continue;
        }

        for (n, add) in map
            .get(&pos)
            .unwrap()
            .iter()
            .filter(|(pos, _)| !seen.contains(pos))
        {
            let mut new_seen = seen.clone();
            new_seen.insert(*n);
            q.push_back((*n, cost + add, new_seen))
        }
    }

    max
}

fn costmap(points: &HashSet<Coord>, map: &Grid) -> HashMap<Coord, HashMap<Coord, usize>> {
    let initial = HashMap::from_iter(points.iter().map(|node| (*node, HashMap::new())));

    points.iter().fold(initial, |mut acc, point| {
        // add the cost of every reachable point.
        // when you reach a point, keep going and remember where you've been so you don't try to visit impossible points
        let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
        let mut seen: HashSet<Coord> = HashSet::new();
        q.push_back((*point, 0));

        while let Some((pos, cost)) = q.pop_front() {
            // record costs for positions in the points set (the condensed map)
            if points.contains(&pos) && cost != 0 {
                *acc.entry(*point).or_default().entry(pos).or_default() = cost;
                continue;
            }

            // go to an adjacent tile if it's not already seen during this path
            for (_, &n) in pos.get_neighbors(map).iter() {
                if seen.insert(n) {
                    q.push_back((n, cost + 1));
                }
            }

            seen.insert(pos);
        }

        acc
    })
}



pub fn part_two(_input: &str) -> Option<u64> {
    let grid = Grid::parse(_input);
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: grid.max_x - 2, y: grid.max_y - 1 };

    let mut points: HashSet<Coord> = grid.all_points();
    points.insert(start);
    points.insert(end);

    let graph: HashMap<Coord, HashMap<Coord, usize>> = costmap(&points, &grid);
    Some(longest(start,end, &graph) as u64)
    // None
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    fn get_neighbors(&self, grid: &Grid) -> HashMap<Direction, Coord> {
        let mut result = HashMap::new();
        if self.x > 0 {
            result.insert(Direction::West, Coord { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            result.insert(Direction::North, Coord { x: self.x, y: self.y - 1 });
        }
        if self.x < grid.max_x - 1 {
            result.insert(Direction::East, Coord { x: self.x + 1, y: self.y });
        }
        if self.y < grid.max_y - 1 {
            result.insert(Direction::South, Coord { x: self.x, y: self.y + 1 });
        }
        result
            .iter()
            .filter(|(_, coord)| grid.get(coord).unwrap() != Tile::Forest)
            .map(|entry| (*entry.0, *entry.1))
            .collect()
    }

    // fn manhattan_distance(&self, other: &Coord) -> usize {
    //     (((self.x as isize) - (other.x as isize)).abs() +
    //         ((self.y as isize) - (other.y as isize)).abs()) as usize
    // }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

// impl Direction {
//     fn reverse(&self) -> Direction {
//         match self {
//             Direction::North => Direction::South,
//             Direction::East => Direction::West,
//             Direction::South => Direction::North,
//             Direction::West => Direction::East,
//         }
//     }
// }

struct Grid {
    grid: Vec<Vec<Tile>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn parse(_input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = _input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        match char {
                            '.' => Tile::Path,
                            '#' => Tile::Forest,
                            '^' => Tile::Slope(Direction::North),
                            '>' => Tile::Slope(Direction::East),
                            'v' => Tile::Slope(Direction::South),
                            '<' => Tile::Slope(Direction::West),
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect();
        let max_x = grid[0].len();
        let max_y = grid.len();
        Self { grid, max_x, max_y }
    }

    fn get(&self, coord: &Coord) -> Option<Tile> {
        // if coord is within bounds
        Some(self.grid[coord.y][coord.x])
        // else none
    }

    fn all_points(&self) -> HashSet<Coord> {
        let mut res = HashSet::new();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let coord = Coord { x, y };
                let tile = self.get(&coord).unwrap();
                if tile != Tile::Forest && coord.get_neighbors(self).iter().count() > 2 {
                    res.insert(coord);
                }
            }
        }
        res
    }

    // fn directional_points(&self) -> HashSet<(Option<Direction>, Coord)> {
    //     let mut res = HashSet::new();
    //     for y in 0..self.max_y {
    //         for x in 0..self.max_x {
    //             let coord = Coord { x, y };
    //             let tile = self.get(&coord).unwrap();
    //             match tile {
    //                 Tile::Slope(direction) => {
    //                     res.insert((Some(direction), coord));
    //                     continue;
    //                 },
    //                 Tile::Forest => (),
    //                 Tile::Path => (),
    //             }
    //             if tile != Tile::Forest && coord.get_neighbors(self).iter().count() > 2 {
    //                 res.insert((None,coord));
    //             }

    //         }
    //     }
    //     res
    // }
}

// struct Node {
//     coord: Coord,
//     direction: Option<Direction>,
//     edges: HashMap<Direction, Coord>,
// }
// struct Graph {
//     nodes: HashMap<Coord, Node>,
// }

// impl Graph {
//     fn parse(grid: &Grid) -> Self {
//
//         // let start_direction = Direction::South;
//         // let mut stack = Vec::new();
//         // stack.push((start_direction, start));

//         // let mut nodes: HashMap<Coord, Node> = HashMap::new();
//         // let start_node = Node {
//         //     coord: start,
//         //     direction: Some(Direction::South),
//         //     edges: HashMap::new(),
//         // };
//         // nodes.insert(start, start_node);

//         // exhaust the stack, filled when we split directions.
//         // while let Some((current_direction, current)) = stack.pop() {
//         //     let node = nodes.get(&current).unwrap();
//         //     let (next, tile, turns) = next_crossroads(current_direction, node, grid);
//         //     let direction = {
//         //         match tile {
//         //             Tile::Path => None,
//         //             Tile::Forest => None,
//         //             Tile::Slope(direction) => Some(direction),
//         //         }
//         //     };
//         //     // not moving does not make new nodes
//         //     if current.manhattan_distance(&next) == 0 {
//         //         continue;
//         //     }
//         //     // if this coord is already a node, we can try updating it, but we don't split
//         //     if nodes.contains_key(&next) {
//         //         // add next node to current's edges
//         //         nodes.entry(current).and_modify(|entry| {
//         //             entry.edges.entry(current_direction).or_insert(next);
//         //         });
//         //         // current node to next's edges
//         //         nodes.entry(next).and_modify(|entry| {
//         //             entry.edges.entry(current_direction.reverse()).or_insert(current);
//         //         });
//         //         continue;
//         //     } else {
//         //         for dir in turns {
//         //             // next to current's edges
//         //             nodes.entry(current).and_modify(|entry|{
//         //                 entry.edges.entry(current_direction).or_insert(next);
//         //             });
//         //             // current to next's edges
//         //             nodes.entry(next).and_modify(|entry|{
//         //                 entry.edges.entry(current_direction.reverse()).or_insert(current);
//         //             }).or_insert_with(||{
//         //                 let mut edges = HashMap::new();
//         //                 edges.insert(current_direction.reverse(), current);
//         //                 Node { coord: next, direction, edges }
//         //             });
//         //             stack.push((dir, next));
//         //         }
//         //     }
//         // }

//         Self { nodes }
//     }

//     fn find_paths(&self) -> Vec<Vec<Coord>> {
//         let start = Coord { x: 1, y: 0 };
//         assert!(&self.nodes.contains_key(&start));
//         let paths = Vec::new();

//         let mut stack: Vec<Vec<Coord>> = Vec::new();
//         stack.push(vec![start]);

//         while let Some(mut state) = stack.pop() {
//             let node = self.nodes.get(state.last().unwrap()).unwrap();
//             if let Some(dir) = &node.direction {
//                 let next = *node.edges.get(dir).unwrap();
//                 if !state.contains(&next) {
//                     state.push(next);
//                     stack.push(state);
//                 }
//                 continue;
//             }
//             for (_dir, next) in &node.edges {
//                 if !state.contains(next) {
//                     state.push(*next);
//                     stack.push(state.clone());
//                 }
//             }
//         }

//         paths
//     }

//     fn path_lengths(&self) -> Vec<usize> {
//         let paths = self.find_paths();
//         let mut results = Vec::new();
//         for path in paths {
//             if path.len() < 2 {
//                 results.push(0);
//                 continue;
//             }
//             let mut length = 0;
//             for i in 1..path.len() - 1 {
//                 let current = path[i];
//                 let next = path[i + 1];
//                 length += current.manhattan_distance(&next);
//             }
//             results.push(length);
//         }
//         results
//     }
// }

// fn next_crossroads(
//     current_direction: Direction,
//     node: &Node,
//     grid: &Grid
// ) -> (Coord, Tile, Vec<Direction>) {
//     let mut current = node.coord;
//     loop {
//         let neighbors = current.get_neighbors(grid);
//         let tile = grid.get(&current).unwrap();
//         let turns = neighbors
//             .iter()
//             .filter(|entry| {
//                 !(*entry.0 == current_direction || node.edges.contains_key(entry.0))
//             })
//             .map(|entry| *entry.0)
//             .collect::<Vec<_>>();

//         // let mut exit = false;
//         if !neighbors.contains_key(&current_direction) {
//             println!("no fwd in neighbors {}", current);
//         }
//         if tile != Tile::Path {
//             println!("Not a path fwd {}", current);
//         }
//         if turns.len() > 0 {
//             println!("Turns here {}", current);
//         }
//         // can't move forward or the tile is a slope or there are perpendicular paths
//         if !neighbors.contains_key(&current_direction) || tile != Tile::Path || turns.len() > 0 {
//             let next_coords = neighbors
//                 .iter()
//                 .filter(|&entry| !node.edges.contains_key(entry.0))
//                 .map(|entry| *entry.0)
//                 .collect();
//             return (current, tile, next_coords);
//         }
//         current = *neighbors.get(&current_direction).unwrap();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
