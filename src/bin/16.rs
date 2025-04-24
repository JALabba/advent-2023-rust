use std::collections::HashSet;

// code studied from
//https://github.com/Zemogus/AOC-2023/blob/main/src/day16.rs

advent_of_code::solution!(16);

pub fn part_one(_input: &str) -> Option<u32> {
    let tiles = parse(_input);
    let tiles = tiles
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<_>>();
    let result = energized_map(0, 0, 1, 0, &tiles, &mut HashSet::new(), &mut HashSet::new());

    Some(result as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let tiles = parse(_input);
    let tiles = tiles
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<_>>();

    let mut max_energised = 0;
    let mut seen = HashSet::new();
    let mut seen_with_d = HashSet::new();

    for x in 0..tiles[0].len() {
        //top edge
        let res = energized_map(x as i8, 0, 0, 1, &tiles, &mut seen, &mut seen_with_d);
        max_energised = max_energised.max(res);
        seen.clear();
        seen_with_d.clear();

        //bottom edge
        let res = energized_map(x as i8, tiles.len() as i8 - 1, 0, -1, &tiles, &mut seen, &mut seen_with_d);
        max_energised = max_energised.max(res);
        seen.clear();
        seen_with_d.clear();
    }

    for y in 0..tiles.len() {
        //left edge
        let res = energized_map(0, y as i8, 1, 0, &tiles, &mut seen, &mut seen_with_d);
        max_energised = max_energised.max(res);
        seen.clear();
        seen_with_d.clear();

        let res = energized_map(tiles[0].len() as i8 - 1, y as i8 , -1, 0, &tiles, &mut seen, &mut seen_with_d);
        max_energised = max_energised.max(res);
        seen.clear();
        seen_with_d.clear();
    }

    Some(max_energised as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}

enum Tile {
    Empty,
    FSlash,
    BSlash,
    Vertical,
    Horizontal,
}

fn parse(_input: &str) -> Vec<Vec<Tile>> {
    _input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '.' => Tile::Empty,
                        '/' => Tile::FSlash,
                        '\\' => Tile::BSlash,
                        '|' => Tile::Vertical,
                        '-' => Tile::Horizontal,
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect()
}

fn energized_map(
    mut x: i8,
    mut y: i8,
    mut dx: i8,
    mut dy: i8,
    tiles: &[&[Tile]],
    seen: &mut HashSet<(i8,i8)>,
    seen_with_d: &mut HashSet<(i8,i8,i8,i8)>
) -> usize {
    //while coordinates within bounds
    while x>=0 && y >=0 && x< tiles[0].len() as i8 && y< tiles.len() as i8 {
        //break if seen this coord and direction
        if !seen.insert((x, y)) && !seen_with_d.insert((x,y,dx,dy)) {
            break;
        }

        match tiles[y as usize][x as usize] {
            Tile::Empty => (),
            Tile::FSlash => {(dx, dy) = (-dy, -dx);},
            Tile::BSlash => {(dx, dy) = (dy, dx);},
            Tile::Vertical => {
                //if dx is 0, you're not crashing into mirror, no splitting.
                if dx !=0 {
                    energized_map(x, y-1, 0, -1, tiles, seen, seen_with_d);
                    energized_map(x, y+1, 0, 1, tiles, seen, seen_with_d);
                    break;
                }
            },
            Tile::Horizontal => {
                //if dy is 0, you're not crashing into mirror, no splitting.
                if dy !=0 {
                    energized_map(x-1, y, -1, 0, tiles, seen, seen_with_d);
                    energized_map(x+1, y, 1, 0, tiles, seen, seen_with_d);
                    break;
                }
            },
        }
        x +=dx;
        y +=dy;
    }
    seen.len()
}

// pub fn part_one(_input: &str) -> Option<u32> {
//     let mut grid = Grid::parse(_input);
//     grid.start_beam(Coordinate(0, 0), Dir::East);
//     grid.shoot_beams();
//     let result = grid.count_energizedd();

//     Some(result as u32)
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// struct Tile {
//     energized: usize,
//     col: usize,
//     row: usize,
//     char: char,
// }
// impl Tile {
//     fn energize(&mut self) {
//         self.energized = 1;
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Status {
//     NotStarted,
//     Ended,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// struct Coordinate(usize, usize);

// #[derive(Clone, Debug, PartialEq, Eq)]
// struct Beam {
//     path: Vec<(Coordinate, Dir)>,
//     status: Status,
// }

// #[derive(Debug)]
// struct Grid {
//     tiles: Vec<Vec<Tile>>,
//     beams: Vec<Beam>,
// }
// impl Grid {
//     fn parse(input: &str) -> Grid {
//         let lines = input.lines().peekable();
//         let mut tiles: Vec<Vec<Tile>> = vec![];

//         for (row, line) in lines.enumerate() {
//             let mut tile_row = vec![];
//             for (col, char) in line.char_indices() {
//                 tile_row.push(Tile { energized: 0, col, row, char });
//             }
//             tiles.push(tile_row);
//         }

//         Grid { tiles, beams: vec![] }
//     }

//     fn start_beam(&mut self, next_coordinate: Coordinate, direction: Dir) {
//         //beam enters top left
//         self.beams.push(Beam {
//             path: vec![(next_coordinate, direction)],
//             status: Status::NotStarted,
//         });
//     }

//     fn check_and_move_beam(
//         &mut self,
//         beam: &mut Beam,
//         cache: &mut HashMap<Coordinate, Dir>
//     ) -> Beam {
//         let mut beam = beam.to_owned();
//         // if this beam's status is Ended, continue;
//         if beam.status == Status::Ended {
//             return beam;
//         }

//         let (coordinate, dir) = *beam.path.last().unwrap();
//         // first figure out this tile's direction
//         let mut this_tile = self.tile_at(coordinate);

//         let next_dirs: Vec<Dir> = Dir::next_directions(this_tile.char, dir);

//         this_tile.energize();

//         if
//             let Some(new_coord) = new_coordinates(
//                 coordinate,
//                 &self.tiles[0].len(),
//                 &self.tiles.len(),
//                 dir
//             )
//         {
//             beam.path.push((new_coord, dir));
//             // check cache, end if found.
//             if !cache.contains_key(&new_coord) {
//                 cache.insert(new_coord, dir);
//             } else {
//                 beam.status = Status::Ended;
//             }
//         } else {
//             beam.status = Status::Ended;
//         }

//         if next_dirs.len() == 1 {
//             let ndir = next_dirs[0];
//             if
//                 let Some(new_coord) = new_coordinates(
//                     coordinate,
//                     &self.tiles[0].len(),
//                     &self.tiles.len(),
//                     ndir
//                 )
//             {
//                 beam.path.push((new_coord, ndir));
//             } else {
//                 beam.status = Status::Ended;
//             }
//         } else {
//             beam.status = Status::Ended;
//             // Start two new beams traversing in the dirs
//             for &new_dir in &next_dirs {
//                 if
//                     let Some(new_coord) = new_coordinates(
//                         coordinate,
//                         &self.tiles[0].len(),
//                         &self.tiles.len(),
//                         new_dir
//                     )
//                 {
//                     self.beams.push(Beam {
//                         path: vec![(new_coord, new_dir)],
//                         status: Status::NotStarted,
//                     });
//                 }
//             }
//         }
//         beam
//     }

//     fn shoot_beams(&mut self) {
//         let mut cache: HashMap<Coordinate, Dir> = HashMap::new();

//         loop {
//             let mut moved = false;
//             for i in 0..self.beams.len() {
//                 let beam_len = self.beams[i].path.len();
//                 let beam = self.check_and_move_beam( &mut self.beams[i].clone() , &mut cache);
//                 if beam.path.len() > beam_len{
//                     moved = true;
//                 }
//                 self.beams[i] = beam;
//             }

//             if !moved {
//                 break;
//             }
//         }
//     }

//     fn count_energizedd(&mut self) -> usize {
//         let count = self.tiles
//             .iter()
//             .flat_map(|row| row.iter())
//             .filter(|tile| tile.energized > 0)
//             .count();
//         count
//     }

//     fn tile_at(&self, next_coordinate: Coordinate) -> Tile {
//         self.tiles[next_coordinate.1][next_coordinate.0]
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// enum Dir {
//     North,
//     East,
//     West,
//     South,
// }
// impl Dir {
//     fn mirror_90deg(from_dir: Dir, char: char) -> Dir {
//         match char {
//             '/' =>
//                 match from_dir {
//                     Dir::North => Dir::West,
//                     Dir::West => Dir::North,
//                     Dir::East => Dir::South,
//                     Dir::South => Dir::East,
//                 }
//             '\\' =>
//                 match from_dir {
//                     Dir::North => Dir::East,
//                     Dir::East => Dir::North,
//                     Dir::West => Dir::South,
//                     Dir::South => Dir::West,
//                 }
//             _ => panic!("invalid char to mirror"),
//         }
//     }

//     fn next_directions(char: char, dir: Dir) -> Vec<Dir> {
//         match char {
//             '.' => vec![dir],
//             '/' => vec![Dir::mirror_90deg(dir, '/')],
//             '\\' => vec![Dir::mirror_90deg(dir, '\\')],
//             '|' => {
//                 if dir != Dir::North && dir != Dir::South {
//                     vec![dir]
//                 } else {
//                     vec![Dir::South, Dir::North]
//                 }
//             }
//             '-' => {
//                 if dir != Dir::West && dir != Dir::East {
//                     vec![dir]
//                 } else {
//                     vec![Dir::West, Dir::East]
//                 }
//             }
//             _ => panic!("invalid char"),
//         }
//     }
// }

// fn new_coordinates(
//     coordinate: Coordinate,
//     max_col: &usize,
//     max_row: &usize,
//     direction: Dir
// ) -> Option<Coordinate> {
//     let (col, row) = (coordinate.0, coordinate.1);
//     let mut new_col = col;
//     let mut new_row = row;

//     match direction {
//         Dir::North => {
//             if row > 0 {
//                 new_row -= 1;
//             } else {
//                 return None; // Cannot move North beyond the minimum row
//             }
//         }
//         Dir::East => {
//             if col < max_col - 1 {
//                 new_col += 1;
//             } else {
//                 return None; // Cannot move East beyond the maximum column
//             }
//         }
//         Dir::West => {
//             if col > 0 {
//                 new_col -= 1;
//             } else {
//                 return None; // Cannot move West beyond the minimum column
//             }
//         }
//         Dir::South => {
//             if row < max_row - 1 {
//                 new_row += 1;
//             } else {
//                 return None; // Cannot move South beyond the maximum row
//             }
//         }
//     }

//     Some(Coordinate(new_col, new_row))
// }
