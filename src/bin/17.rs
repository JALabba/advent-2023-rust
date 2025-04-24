use std::collections::BinaryHeap;

advent_of_code::solution!(17);

pub fn part_one(_input: &str) -> Option<u16> {
    let grid = parse(_input);
    let grid = grid
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<_>>();

    lowest_loss(&grid, 1, 3)
    // let path = astar(&grid, Node(0, 0), Node(grid[0].len() as i8, grid.len() as i8));
    // match path {
    //     Some(path) => Some(count_heat(path, &grid) as u32),
    //     None => None,
    // }
}

pub fn part_two(_input: &str) -> Option<u16> {
    let grid = parse(_input);
    let grid = grid
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<_>>();

    lowest_loss(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}

fn parse(_input: &str) -> Vec<Vec<u8>> {
    _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right = 0,
    Left,
    Down,
    Up,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: u8,
    y: u8,
    d: Direction,
    g_score: u16,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.g_score == other.g_score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse so that binary heap is a min heap
        self.g_score.cmp(&other.g_score).reverse()
    }
}

struct Array3D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
    depth: usize,
}
impl <T>Array3D<T> where T: Copy{
    fn new( width: usize, height: usize, depth: usize, default: T ) -> Self {
        Self {
            data: vec![default; width * height * depth],
            width,
            height,
            depth
          }
    }
    fn get(&self, x: u8, y: u8, z: u8) -> T {
        let (x, y, z) = (x as usize, y as usize, z as usize);

        debug_assert!(x < self.width && y < self.height && z < self.depth);
        self.data[x + y * self.width + z * self.width * self.height]
    }

    fn set(&mut self, x: u8, y: u8, z: u8, value: T) {
        let (x, y, z) = (x as usize, y as usize, z as usize);

        debug_assert!(x < self.width && y < self.height && z < self.depth);
        self.data[x + y * self.width + z * self.width * self.height] = value;
    }
}
fn lowest_loss(blocks: &[&[u8]], min_straight: u8, max_straight: u8) -> Option<u16> {
    // Dijkstra's algorithm
    let start_x = 0;
    let start_y = 0;
    let target_x = (blocks[0].len() - 1) as u8;
    let target_y = (blocks.len() - 1) as u8;

    let mut g_scores = Array3D::new(blocks[0].len(), blocks.len(), 5, u16::MAX);
    g_scores.set(start_x, start_y, Direction::None as u8, 0);

    let mut open = BinaryHeap::new();
    open.push(Node {
        x: start_x,
        y: start_y,
        d: Direction::None,
        g_score: g_scores.get(start_x, start_y, Direction::None as u8),
    });

    while let Some(Node { x, y, d, g_score }) = open.pop() {
        if (x, y) == (target_x, target_y) {
            return Some(g_score);
        }

        let directions = match d {
            Direction::Right | Direction::Left => [Direction::Down, Direction::Up],
            Direction::Down | Direction::Up => [Direction::Right, Direction::Left],
            Direction::None => [Direction::Right, Direction::Down],
        };
        for next_d in directions {
            let mut tentative_g_score = g_score;
            for step in 1..=max_straight {
                let (next_x, next_y) = match next_d {
                    Direction::Right => (x + step, y),
                    Direction::Left => (x.wrapping_sub(step), y),
                    Direction::Down => (x, y + step),
                    Direction::Up => (x, y.wrapping_sub(step)),
                    Direction::None => unreachable!(),
                };

                // Stay in bounds
                if next_x as usize >= blocks[0].len() || next_y as usize >= blocks.len() {
                    break;
                }

                tentative_g_score = tentative_g_score
                    .saturating_add(blocks[next_y as usize][next_x as usize] as u16);

                // Too early to turn
                if step < min_straight {
                    continue;
                }

                if tentative_g_score < g_scores.get(next_x, next_y, next_d as u8) {
                    // Found better path
                    g_scores.set(next_x, next_y, next_d as u8, tentative_g_score);

                    open.push(Node {
                        x: next_x,
                        y: next_y,
                        d: next_d,
                        g_score: tentative_g_score,
                    });
                }
            }
        }
    }

    None
}

// fn manhattan_distance(from: Node, to: Node) -> usize {
//     (abs(from.0 - to.0) + abs(from.1 - to.1)).try_into().unwrap()
// }

// fn neighbors(current: Node, max: Node) -> Vec<Node> {
//     let dist = 1;
//     let mut res = vec![];
//     for y in current.1 - dist..=current.1 + dist {
//         for x in current.0 - dist..=current.0 + dist {
//             if x >= 0 && y >= 0 && x < max.0 && y < max.1 {
//                 res.push(Node(x, y));
//             }
//         }
//     }
//     res
// }

// #[allow(dead_code)]
// fn count_heat(path: Vec<Node>, grid: &[&[i8]]) -> u32 {
//     path.iter().fold(0, |acc, node| {
//         if node != &Node(0,0){
//             acc + grid[node.1 as usize][node.0 as usize] as u32
//         } else {
//             acc
//         }
//     } )
// }

// #[allow(dead_code)]
// fn astar(grid: &[&[i8]], start: Node, end: Node) -> Option<Vec<Node>> {
//     let mut frontier = BinaryHeap::new();
//     let mut came_from: HashMap<Node, Option<Node>> = HashMap::new();
//     let mut cost_so_far = HashMap::new();

//     frontier.push((start, Cost { g: 0, h: 0 }));

//     came_from.insert(start, None);
//     cost_so_far.insert(start, Cost { g: 0, h: manhattan_distance(end, start) });

//     while let Some((current, _)) = frontier.pop() {
//         if current == end {
//             // Reconstruct the path
//             let mut path = Vec::new();
//             let mut node = current;

//             while let Some(&parent) = came_from.get(&node) {
//                 path.push(node);
//                 if node == start {
//                     break;
//                 }
//                 node = parent.unwrap();
//             }

//             path.reverse();
//             return Some(path);
//         }

//         for next in neighbors(current, end) {
//             let new_cost =
//                 cost_so_far[&current].g + (grid[next.1 as usize][next.0 as usize] as usize);

//             if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next].g {
//                 // let _priority = new_cost + manhattan_distance(end, next);
//                 frontier.push((next, Cost { g: new_cost, h: manhattan_distance(end, next) }));
//                 came_from.insert(next, Some(current));
//                 cost_so_far.insert(next, Cost { g: new_cost, h: manhattan_distance(end, next) });
//             }
//         }
//     }

//     // No path found
//     None
// }

// #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
// struct Node(i8, i8);

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// struct Cost {
//     g: usize,
//     h: usize,
// }

// impl Ord for Cost {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         (other.g + other.h).cmp(&(self.g + self.h))
//     }
// }

// impl PartialOrd for Cost {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
