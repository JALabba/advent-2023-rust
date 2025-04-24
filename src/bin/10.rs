advent_of_code::solution!(10);

pub fn part_one(_input: &str) -> Option<u64> {
    let grid = parse_grid(_input);
    let loop_list = find_loop(&grid);
    Some((loop_list.len() / 2) as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let grid = parse_grid(_input);
    let loop_list = find_loop(&grid);

    //shoelace formula
    let mut area: isize = 0;
    let n = loop_list.len() as isize;
    for w in loop_list.windows(2) {
        area += (w[0].row * w[1].col) as isize;
        area -= (w[0].col * w[1].row) as isize;
    }
    let area = isize::abs(area) / 2;

    //find number of tiles inside
    Some((area - (n / 2) + 1) as u64  )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = part_one(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_south(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn get_east(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn get_west(&self) -> Option<Coord> {
        if self.col == 0 {
            return None;
        }
        Some(Coord {
            row: self.row,
            col: self.col - 1,
        })
    }

    fn get_north(&self) -> Option<Coord> {
        if self.row == 0 {
            return None;
        }
        Some(Coord {
            row: self.row - 1,
            col: self.col,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Directions {
    North,
    South,
    East,
    West,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl Directions {
    fn enter_from(from: Directions, other: Option<Directions>) -> bool {
        if let Some(od) = other {
            match from {
                Directions::North => od == Directions::NS || od == Directions::SE || od == Directions::SW,
                Directions::South =>  od == Directions::NS || od == Directions::NE || od == Directions::NW,
                Directions::West => od == Directions::EW || od == Directions::NE || od == Directions::SE,
                Directions::East => od == Directions::EW || od == Directions::NW || od == Directions::SW,
                _=> panic!("Called with a non-cardinal direction")
            }
        } else {
            false
        }
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Directions>> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<Directions>> =
        vec![vec![Directions::Ground; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            let dir = match char {
                '|' => Directions::NS,
                '-' => Directions::EW,
                'L' => Directions::NE,
                'J' => Directions::NW,
                '7' => Directions::SW,
                'F' => Directions::SE,
                'S' => Directions::Start,
                _ => Directions::Ground,
            };
            grid[row][col] = dir;
        });
    });
    grid
}

fn determine_start_coord_shape(grid: &Vec<Vec<Directions>>) -> (Coord, Directions) {
    // locate the start coord
    let mut start_pos: Coord = Coord { row: 0, col: 0 };
    'row: for (row, full_row) in grid.iter().enumerate() {
        for (col, char) in full_row.iter().enumerate() {
            if char == &Directions::Start {
                start_pos = Coord { row, col };
                break 'row;
            }
        }
    }
    let grid_direction = |c: Option<Coord>| -> Option<Directions> {
        if let Some(coord) = c {
            Some(grid[coord.row][coord.col])
        } else {
            None
        }
    };
    // look over the neighboring coordinates
    // prettier-ignore
    {
        let entry_north = Directions::enter_from(Directions::North ,grid_direction(start_pos.get_north()));
        let entry_west = Directions::enter_from(Directions::West ,grid_direction(start_pos.get_west()));
        let entry_south = Directions::enter_from(Directions::South ,grid_direction(Some(start_pos.get_south())));
        let entry_east = Directions::enter_from(Directions::East ,grid_direction(Some(start_pos.get_east())));
        if entry_north && entry_south {
            (start_pos, Directions::NS)
        } else if entry_north && entry_west {
            (start_pos, Directions::NW)
        } else if entry_north && entry_east {
            (start_pos, Directions::NE)
        } else if entry_south && entry_west {
            (start_pos, Directions::SW)
        } else if entry_south && entry_east {
            (start_pos, Directions::SE)
        } else if entry_west && entry_east {
            (start_pos, Directions::EW)
        } else {
            panic!("WTF!!!")
        }
    }
}

fn find_loop(grid: &Vec<Vec<Directions>>) -> Vec<Coord> {
    let (start_coord, start_shape) = determine_start_coord_shape(grid);
    let mut loop_list: Vec<Coord> = vec![start_coord];
    let (mut next, mut coming_from) = match start_shape {
        Directions::SE => (start_coord.get_east(), Directions::West),
        Directions::SW => (start_coord.get_south(), Directions::North),
        Directions::NS => (start_coord.get_north().unwrap(), Directions::South),
        _ => panic!("Unimplemented"),
    };
    loop_list.push(next);

    while next != start_coord {
        let cur = grid[next.row][next.col];
        (next, coming_from) = match (cur, coming_from) {
            (Directions::NS, Directions::South) => (next.get_north().unwrap(), Directions::South),
            (Directions::NS, Directions::North) => (next.get_south(), Directions::North),
            (Directions::EW, Directions::West) => (next.get_east(), Directions::West),
            (Directions::EW, Directions::East) => (next.get_west().unwrap(), Directions::East),
            (Directions::NW, Directions::North) => (next.get_west().unwrap(), Directions::East),
            (Directions::NW, Directions::West) => (next.get_north().unwrap(), Directions::South),
            (Directions::NE, Directions::East) => (next.get_north().unwrap(), Directions::South),
            (Directions::NE, Directions::North) => (next.get_east(), Directions::West),
            (Directions::SE, Directions::South) => (next.get_east(), Directions::West),
            (Directions::SE, Directions::East) => (next.get_south(), Directions::North),
            (Directions::SW, Directions::West) => (next.get_south(), Directions::North),
            (Directions::SW, Directions::South) => (next.get_west().unwrap(), Directions::East),
            _ => panic!("Impossible"),
        };
        loop_list.push(next);
    }
    loop_list
}
