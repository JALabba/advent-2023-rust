use std::{ collections::HashMap, fmt::Debug, str::FromStr };

use advent_of_code::lcm;

advent_of_code::solution!(8);

pub fn part_one(_input: &str) -> Option<u64> {
    let network = Network::parse(_input);
    // let network = Network::parse(&advent_of_code::template::read_file("examples", DAY));
    let res = network.walk_from("AAA".to_string(), |s| *s == "ZZZ".to_string());
    Some(res as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let network = Network::parse(_input);
    let starts = network.nodes
        .keys()
        .filter(|s| s.ends_with("A"))
        .collect::<Vec<_>>();
    let counts = starts
        .iter()
        .map(|&start| network.walk_from(start.clone(), |s| s.ends_with("Z")))
        .collect::<Vec<_>>();
    let result = counts.iter().fold(1, |acc, x| lcm(acc, *x));
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

struct Network {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n{}\n",
            self.directions
                .iter()
                .map(|dir| dir.as_char())
                .collect::<String>()
        ).ok();
        for node in self.nodes.iter() {
            writeln!(f, "{} = ({}, {})", node.0, node.1.left, node.1.right).ok();
        }
        Ok(())
    }
}

impl Network {
    fn parse(input: &str) -> Network {
        let mut lines = input.lines();
        let directions = lines
            .next()
            .expect("steps line")
            .trim()
            .chars()
            .map(|c| {
                match c {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        let _ = lines.next();
        let mut nodes: HashMap<String, Node> = HashMap::new();
        for line in lines {
            let parts = line
                .replace("=", "")
                .replace("(", "")
                .replace(")", "")
                .replace(",", "")
                .split_whitespace()
                .map(String::from_str)
                .flatten()
                .collect::<Vec<String>>();
            nodes.insert(parts[0].clone(), Node {
                left: parts[1].clone(),
                right: parts[2].clone(),
            });
        }
        Network {
            directions,
            nodes,
        }
    }

    fn walk_from(&self, start: String, end_condition: impl Fn(&String) -> bool) -> usize {
        let mut stepper = Stepper::new(&self, start);
        loop {
            stepper.walk();
            if end_condition(&stepper.current) {
                break;
            }
        }
        stepper.count
    }
}

struct Stepper<'a> {
    current: String,
    count: usize,
    network: &'a Network,
}

impl Debug for Stepper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stepper")
            .field("current", &self.current)
            .field("count", &self.count)
            .finish()
    }
}

impl<'a> Stepper<'a> {
    fn new(network: &'a Network, current: String) -> Stepper<'a> {
        Self { current, count: 0, network }
    }

    fn get_direction(&mut self) -> Direction {
        self.network.directions[self.count % self.network.directions.len()]
    }

    fn walk(&mut self) {
        let node = self.network.nodes.get(&self.current).unwrap();
        self.current = match self.get_direction() {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
        self.count += 1;
    }
}
