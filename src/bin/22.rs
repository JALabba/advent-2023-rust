use std::{ collections::{HashMap, HashSet}, fmt::{ Debug, Error } };

advent_of_code::solution!(22);

pub fn part_one(_input: &str) -> Option<u64> {
    // let mut bricks = parse_input(&advent_of_code::template::read_file("examples", DAY));
    let mut bricks = parse_input(_input);
    bricks.sort_by_key(|brick| brick.a.z.min(brick.b.z));

    // let bricks fall -z
    bricks = settle_bricks(bricks);

    let structures = build_structures(&bricks);

    let removable = identify_removable_structures(&structures);

    Some(removable.len() as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
        // let mut bricks = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let mut bricks = parse_input(_input);
        bricks.sort_by_key(|brick| brick.a.z.min(brick.b.z));

        // let bricks fall -z
        bricks = settle_bricks(bricks);

        let structures = build_structures(&bricks);

        let removable = identify_removable_structures(&structures);

        let casuality: Vec<usize> = count_chain_reaction(&structures, &removable);

        Some(casuality.iter().sum::<usize>() as u64)
}

fn count_chain_reaction(structures: &HashMap<ID, Structure>, removable: &Vec<ID>) -> Vec<usize> {
    structures.iter().filter(|(id, _)|{
        !removable.contains(&id)
    }).map(|(_id, s)|{
        let mut disintegrated= HashSet::new();
        shaka_when_the_bricks_fell(structures, s, &mut disintegrated);
        // println!("id:{} dints: {}, {:?}", id.to_char(), disintegrated.len() -1, s);
        disintegrated.len() -1
    }).collect::<Vec<_>>()
}

fn shaka_when_the_bricks_fell(structures: &HashMap<ID, Structure>, s: &Structure, disintegrated: &mut HashSet<ID>) {
    disintegrated.insert(s.id);

    for above in &s.supporting {
        let a = structures.get(above).unwrap();
        if a.supported_by.iter().all(|id|{
            disintegrated.contains(id)
        }){
            let a = structures.get(&above).unwrap();
            shaka_when_the_bricks_fell(structures, a, disintegrated);
        }
    }
}

/* --- */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl From<Vec<usize>> for Coord {
    fn from(item: Vec<usize>) -> Self {
        assert!(item.len() == 3);
        Coord::new(item[0], item[1], item[2])
    }
}

impl TryFrom<&str> for Coord {
    type Error = std::fmt::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let item = value
            .split(",")
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        assert!(item.len() == 3);
        let item = item
            .iter()
            .map(|&s| s.parse::<usize>().expect("parsed usize"))
            .collect::<Vec<_>>();
        Ok(Coord::from(item))
    }
}

impl Coord {
    fn new(x: usize, y: usize, z: usize) -> Self {
        assert!(z >= 1);
        Self { x, y, z }
    }

    fn below(&self) -> Coord {
        Self { x: self.x, y: self.y, z: self.z - 1 }
    }

    fn above(&self) -> Coord {
        Self { x: self.x, y: self.y, z: self.z + 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ID(u16);

impl ID {
    fn to_char(&self) -> char {
        if self.0 < 26 {
            (self.0 as u8 + b'A') as char
        } else {
            '#'
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Brick {
    id: ID,
    a: Coord,
    b: Coord,
}

impl TryFrom<&str> for Brick {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a, b) = value.split_once("~").expect("two parts");
        let a = Coord::try_from(a).expect("Coord");
        let b = Coord::try_from(b).expect("Coord");
        Ok(Brick { id: ID(0), a, b })
    }
}

impl Brick {
    #[allow(dead_code)]
    fn area(&self) -> usize {
        let x = self.a.x.abs_diff(self.b.x) + 1;
        let y = self.a.y.abs_diff(self.b.y) + 1;
        let z = self.a.z.abs_diff(self.b.z) + 1;
        x * y * z
    }

    fn below(&self) -> Self {
        Self { id: self.id, a: self.a.below(), b: self.b.below() }
    }

    fn overlaps(&self, brick: &Brick) -> bool {
        !(
            self.a.x > brick.b.x ||
            self.b.x < brick.a.x ||
            self.a.y > brick.b.y ||
            self.b.y < brick.a.y ||
            self.a.z > brick.b.z ||
            self.b.z < brick.a.z
        )
    }

    #[allow(dead_code)]
    fn above(&self) -> Brick {
        Self { id: self.id, a: self.a.above(), b: self.b.above() }
    }

    fn min(&self, axis: Axis) -> usize {
        match axis {
            Axis::X => self.a.x.min(self.b.x),
            Axis::Y => self.a.y.min(self.b.y),
            Axis::Z => self.a.z.min(self.b.z),
        }
    }

    #[allow(dead_code)]
    fn max(&self, axis: Axis) -> usize {
        match axis {
            Axis::X => self.a.x.max(self.b.x),
            Axis::Y => self.a.y.max(self.b.y),
            Axis::Z => self.a.z.max(self.b.z),
        }
    }
}

struct Structure {
    #[allow(dead_code)]
    id: ID,
    supporting: Vec<ID>,
    supported_by: Vec<ID>,
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, supporting: {}, supported_by: {}",
            self.id.to_char(),
            self.supporting
                .iter()
                .map(|i| i.to_char())
                .collect::<String>(),
            self.supported_by
                .iter()
                .map(|i| i.to_char())
                .collect::<String>()
        )
    }
}

/// call on a settled stack
fn build_structures(bricks: &Vec<Brick>) -> HashMap<ID, Structure> {
    let mut result = HashMap::new();
    for this_brick in bricks {
        let above = this_brick.above();
        let below = this_brick.below();
        let supporting = bricks
            .iter()
            .filter(|&brick| brick.overlaps(&above))
            .map(|b| b.id)
            .filter(|id| *id != this_brick.id)
            .collect::<Vec<_>>();
        let supported_by = bricks
            .iter()
            .filter(|&brick| brick.overlaps(&below))
            .map(|b| b.id)
            .filter(|id| *id != this_brick.id)
            .collect::<Vec<_>>();
        result.insert(this_brick.id, Structure { id: this_brick.id, supporting, supported_by });
    }
    result
}

fn parse_input(_input: &str) -> Vec<Brick> {
    _input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut brick = Brick::try_from(line).expect("Brick parsed from line");
            brick.id = ID(i as u16);
            brick
        })
        .collect()
}

fn settle_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    let mut settled = Vec::new();

    while !bricks.is_empty() {
        let mut next_bricks = Vec::new();

        for mut brick in bricks {
            if can_move_down(&brick, &settled) {
                brick = brick.below();
                next_bricks.push(brick);
            } else {
                settled.push(brick);
            }
        }

        bricks = next_bricks;
    }

    settled
}

fn can_move_down(brick: &Brick, settled: &[Brick]) -> bool {
    if brick.min(Axis::Z) == 1 {
        return false;
    }

    let moved_brick = brick.below();
    !settled.iter().any(|settled_brick| moved_brick.overlaps(settled_brick))
}


fn identify_removable_structures(structures: &HashMap<ID,Structure>) -> Vec<ID> {
    let removable: Vec<ID> = structures
        .iter()
        .filter_map(|(id,structure)| {
            if structure.supporting.is_empty() {
                // println!("brick {} is not supporting any bricks and can be removed.", id.to_char());
                return Some(*id);
            }
            let count = structure.supporting
                .iter()
                .filter(|&over| {
                    structures.get(over).is_some_and(|s| s.supported_by.len() > 1)
                })
                .count();

            if structure.supporting.len() == count {
                // println!(
                //     "brick {} is supporting bricks that are supported by {} or more others. can be removed.",
                //     id.to_char(),
                //     count
                // );
                Some(*id)
            } else {
                // println!("brick {} can not be removed.", id.to_char());
                None
            }
        })
        .collect();
    removable
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[allow(dead_code)]
fn visualize_bricks(bricks: &Vec<Brick>, show_axis: Axis) {
    let (mut x_max, mut y_max, mut z_max) = (0, 0, 0);
    for brick in bricks {
        x_max = x_max.max(brick.a.x).max(brick.b.x);
        y_max = y_max.max(brick.a.y).max(brick.b.y);
        z_max = z_max.max(brick.a.z).max(brick.b.z);
    }
    let some_brick_has = |this: Coord| -> Option<&Brick> {
        bricks
            .iter()
            .find(|brick| {
                (brick.a.x..=brick.b.x).contains(&this.x) &&
                    (brick.a.y..=brick.b.y).contains(&this.y) &&
                    (brick.a.z..=brick.b.z).contains(&this.z)
            })
    };
    let mut grid = vec![vec![vec!['!'; x_max + 1]; y_max + 1]; z_max + 1];
    for z in 0..=z_max {
        for y in 0..=y_max {
            for x in 0..=x_max {
                let this = Coord { x, y, z };
                if let Some(brick) = some_brick_has(this) {
                    grid[z][y][x] = brick.id.to_char();
                }
            }
        }
    }
    println!("");
    match show_axis {
        Axis::X => visualize_axis(&grid, x_max, y_max, z_max, |z, x| (z, x)),
        Axis::Y => visualize_axis(&grid, y_max, x_max, z_max, |z, y| (z, y)),
        _ => panic!("Nope"),
    }
}

fn visualize_axis<F>(
    grid: &Vec<Vec<Vec<char>>>,
    max1: usize,
    max2: usize,
    z_max: usize,
    axis_mapper: F
)
    where F: Fn(usize, usize) -> (usize, usize)
{
    for z in (0..=z_max).rev() {
        for i in 0..=max1 {
            if z == 0 {
                print!("-");
                continue;
            }
            let mut line = Vec::new();
            for j in 0..=max2 {
                let (z_idx, idx) = axis_mapper(z, i);
                let label = grid[z_idx][j][idx];
                if label != '!' {
                    line.push(label);
                }
            }
            let mut seen = HashSet::new();
            line.retain(|item| seen.insert(*item));
            if line.len() == 1 {
                print!("{}", line[0]);
            } else if line.len() > 1 {
                print!("{}", line.len());
            } else {
                print!(".");
            }
        }
        print!(" {}\n", z);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn brick_area() {
        let mut brick = Brick {
            id: ID(0),
            a: Coord { x: 0, y: 0, z: 0 },
            b: Coord { x: 0, y: 0, z: 0 },
        };
        assert_eq!(brick.area(), 1, "Single cube");
        brick.b.z = 9;
        assert_eq!(brick.area(), 10, "10 cubes");
        brick.b.x = 1;
        assert_eq!(brick.area(), 20, "20 cubes");
        brick.b.y = 2;
        assert_eq!(brick.area(), 60, "60 cubes");
    }
}
