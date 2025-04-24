use std::cmp::{ min, max };

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    // grid.fill();
    // let result = grid.count_filled();
    let result = grid.lagoon_area();
    println!("lagoon result: {}", result);
    println!(" shoelace result: {}", grid.shoelace_formula());
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let vertices = parse2(input);
    // for vert in &vertices {
    //     println!("{:?}", vert);
    // }
    let result = lagoon_area(vertices);

    Some(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate(i16, i16);

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up = 0,
    Right,
    Down,
    Left,
}

// #[allow(dead_code)]
#[derive(Debug)]
struct Trench {
    start: Coordinate,
    direction: Dir,
    distance: i16,
    color: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Hole {
    num_of_tot: (i8, i8),
    dir: Dir,
    color: String,
    // fill: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Grid {
    grid: Vec<Vec<Hole>>,
    trenches: Vec<Trench>,
    vertices: Vec<Coordinate>,
    width: i16,
    height: i16,
}

impl Grid {
    fn parse(_input: &str) -> Self {
        let lines = _input.lines().peekable();

        let mut width: i16 = 0;
        let mut minw = 0;
        let mut maxw = 0;

        let mut height: i16 = 0;
        let mut miny = 0;
        let mut maxy = 0;

        let mut trenches = Vec::new();
        let mut vertices = vec![];

        for line in lines {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            let direction = parts[0].chars().next().unwrap();

            let distance = parts[1].trim().parse::<i16>().ok().unwrap();

            let hex: String = parts[2]
                .trim()
                .chars()
                .filter(|c| {
                    match c {
                        '(' | ')' | '#' => false,
                        _ => true,
                    }
                })
                .collect();

            if hex.len() != 6 {
                panic!("hex parse problem");
            }

            // if first trench, compensate for starting one off
            // counting boundaries for the grid
            // updating direction to a Dir
            let direction = match direction {
                'L' => { Dir::Left }
                'R' => { Dir::Right }
                'U' => { Dir::Up }
                'D' => { Dir::Down }
                _ => unreachable!(),
            };

            //track min and max
            minw = min(minw, width);
            miny = min(miny, height);
            maxw = max(maxw, width);
            maxy = max(maxy, height);

            //push
            trenches.push(Trench {
                start: Coordinate(width, height),
                direction,
                distance,
                color: hex,
            });
            vertices.push(Coordinate(width, height));

            match direction {
                Dir::Up => {
                    height -= distance;
                }
                Dir::Down => {
                    height += distance;
                }
                Dir::Left => {
                    width -= distance;
                }
                Dir::Right => {
                    width += distance;
                }
            }
        } // for lines
        vertices.push(Coordinate(0, 0));
        // abs diff gives the size of the grid, +1 to stop overflow
        let width_diff = (maxw - minw).abs() + 1;
        let height_diff = (maxy - miny).abs() + 1;
        // println!("Width: {}, range: {}..={}, diff: {}", width, minw, maxw, width_diff);
        // println!("Height: {}, range: {}..={}, diff: {}", height, miny, maxy, height_diff);

        // create grid vec
        let default: Hole = Hole { num_of_tot: (0, 0), dir: Dir::Down, color: "".to_string() };
        let mut grid: Vec<Vec<Hole>> =
            vec![vec![ default ; width_diff as usize];height_diff as usize];

        // offset is the distance from minimum to 0
        let x_offset = -minw;
        let y_offset = -miny;

        for trench in &trenches {
            let (x, y) = (trench.start.0 + x_offset, trench.start.1 + y_offset);

            let dist = trench.distance;

            //ranges start at 0 to use start position.
            // each for loop will go one further than distance which will be the next trench start
            match trench.direction {
                Dir::Left => {
                    for dx in 0..dist {
                        let nx = x - dx;
                        grid[y as usize][nx as usize] = Hole {
                            num_of_tot: (dx as i8, dist as i8),
                            dir: trench.direction,
                            color: trench.color.clone(),
                        };
                    }
                }
                Dir::Up => {
                    for dy in 0..dist {
                        let ny = y - dy;
                        grid[ny as usize][x as usize] = Hole {
                            num_of_tot: (dy as i8, dist as i8),
                            dir: trench.direction,
                            color: trench.color.clone(),
                        };
                    }
                }
                Dir::Right => {
                    for dx in 0..dist {
                        let nx = x + dx;
                        grid[y as usize][nx as usize] = Hole {
                            num_of_tot: (dx as i8, dist as i8),
                            dir: trench.direction,
                            color: trench.color.clone(),
                        };
                    }
                }
                Dir::Down => {
                    for dy in 0..dist {
                        let ny = y + dy;
                        grid[ny as usize][x as usize] = Hole {
                            num_of_tot: (dy as i8, dist as i8),
                            dir: trench.direction,
                            color: trench.color.clone(),
                        };
                    }
                }
            }
        }

        Self { grid, trenches, vertices, width: width_diff, height: height_diff }
    }

    fn shoelace_formula(&self) -> i64 {
        let n = self.vertices.len();

        if n < 3 {
            panic!("must have more than 3 vertices");
        }

        let mut area: i64 = 0;

        for i in 0..n {
            let j = (i + 1) % n;
            area +=
                (self.vertices[i].0 as i64) * (self.vertices[j].1 as i64) -
                (self.vertices[j].0 as i64) * (self.vertices[i].1 as i64);
        }

        area = area.abs() / 2;
        area += self.count_loop() / 2 + 1;
        area
    }

    fn count_loop(&self) -> i64 {
        let mut count = 0;
        for row in &self.grid {
            for hole in row {
                if hole.color.len() > 2 {
                    count += 1;
                }
            }
        }
        count
    }

    fn lagoon_area(&self) -> u64 {
        let inner_area = self.vertices
            .windows(2)
            .map(|w| {
                // if y1 != y2, then x1==x2 and the area is 0
                // otherwise y1==y2 => y1+y2 == 2(y1), so we can half it here
                let (x1, y1) = (w[0].0 as i64, w[0].1 as i64);
                let (x2, _) = (w[1].0 as i64, w[1].1 as i64);
                (x2 - x1) * y1
            })
            .sum::<i64>()
            .unsigned_abs();
        // a border of width 0.5 because of integer coordinates
        let border_area =
            self.vertices
                .windows(2)
                .map(|w| {
                    let (x1, y1) = (w[0].0 as i64, w[0].1 as i64);
                    let (x2, y2) = (w[1].0 as i64, w[1].1 as i64);
                    x1.abs_diff(x2) + y1.abs_diff(y2)
                })
                .sum::<u64>() /
                2 +
            1;
        //add +1 to accoutn for the 4 extra convex corners

        inner_area + border_area
    }

    // fn fill(&mut self) {
    //     let coord = Coordinate(self.width/2, self.height/2);
    //     let mut visited = vec![coord];
    //     for neighbor in get_neighbors(coord) {
    //         if visited.contains(&neighbor) {
    //             continue;
    //         } else {
    //             visited.push(neighbor)
    //         }

    //     }
    // }

    // fn count_filled(&self) -> u32 {
    //     print_grid(&self.grid, Coordinate(self.width, self.height));
    //     let mut count = 0;
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             if !self.grid[y as usize][x as usize].color.is_empty() {
    //                 count +=1;
    //             }
    //         }
    //     }
    //     count
    // }
}

// fn get_neighbors(coord: Coordinate) -> Vec<Coordinate> {
//     vec![
//         //top
//         Coordinate(coord.0 -1 , coord.1 -1 ),
//         Coordinate(coord.0 , coord.1 -1 ),
//         Coordinate(coord.0 +1 , coord.1 -1 ),
//         //middle
//         Coordinate(coord.0 -1 , coord.1 ),
//         // Coordinate(coord.0 , coord.1 ),
//         Coordinate(coord.0 +1 , coord.1 ),
//         //bottom
//         Coordinate(coord.0 -1 , coord.1 +1 ),
//         Coordinate(coord.0  , coord.1 +1 ),
//         Coordinate(coord.0 +1 , coord.1 +1 ),
//     ]
// }

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Hole>>, coord: Coordinate) {
    println!();
    for (y, row) in grid.iter().enumerate() {
        println!();
        for (x, hole) in row.iter().enumerate() {
            if Coordinate(x as i16, y as i16) == coord {
                print!("O");
            } else if !hole.color.is_empty() {
                if hole.color == "0" {
                    print!("X");
                } else {
                    print!("#");
                }
            } else {
                print!(".");
            }
        }
    }
    println!();
}

fn parse2(input: &str) -> Vec<(i64, i64)> {
    let lines = input.lines().peekable();

    let mut vertices = vec![];
    let (mut x, mut y) = (0,0);

    for line in lines {
        let mut hex = line
            .trim()
            .split_ascii_whitespace()
            .nth(2)
            .map(|part| {
                part.chars()
                    .filter(|c| {
                        match c {
                            '(' | ')' | '#' => false,
                            _ => true,
                        }
                    })
                    .collect::<String>()
            })
            .unwrap();

        let (distance,direction) ={ let (dist, dir) = hex.split_at_mut(5);
            (i64::from_str_radix(&dist, 16).unwrap()  ,
            dir.parse::<i8>().unwrap_or_default())};
            // println!("{distance}");

        vertices.push((x,y));
        match direction {
            0=> x += distance,
            1=> y += distance,
            2=> x -= distance,
            3=> y -= distance,
            _=> unreachable!(),
        };
    }
    vertices.push((x, y));
    vertices
}

fn lagoon_area(vertices: Vec<(i64, i64)>) -> u64 {
    let inner_area = vertices
        .windows(2)
        .map(|w| {
            // if y1 != y2, then x1==x2 and the area is 0
            // otherwise y1==y2 => y1+y2 == 2(y1), so we can half it here
            let (x1, y1) = (w[0].0 as i64, w[0].1 as i64);
            let (x2, _) = (w[1].0 as i64, w[1].1 as i64);
            (x2 - x1) * y1
        })
        .sum::<i64>()
        .unsigned_abs();
    // a border of width 0.5 because of integer coordinates
    let border_area =
        vertices
            .windows(2)
            .map(|w| {
                let (x1, y1) = (w[0].0 as i64, w[0].1 as i64);
                let (x2, y2) = (w[1].0 as i64, w[1].1 as i64);
                x1.abs_diff(x2) + y1.abs_diff(y2)
            })
            .sum::<u64>() /
            2 +
        1;
    //add +1 to accoutn for the 4 extra convex corners

    inner_area + border_area
}

// fn shoelace_formula( vertices: Vec<(i64, i64)>) -> i64 {
//     let n = vertices.len();

//     if n < 3 {
//         panic!("must have more than 3 vertices");
//     }

//     let mut area: i64 = 0;

//     for i in 0..n {
//         let j = (i + 1) % n;
//         area += vertices[i].0 as i64 * vertices[j].1 as i64 - vertices[j].0 as i64 * vertices[i].1 as i64;
//     }

//     area = area.abs() / 2;
//     area += count_loop() /2 +1;
//     area
// }

// fn count_loop() -> i64 {
//     let mut count = 0;
//     for row in &self.grid{
//         for hole in row {
//             if hole.color.len() >2 {
//                 count+=1;
//             }
//         }
//     }
//     count
// }
