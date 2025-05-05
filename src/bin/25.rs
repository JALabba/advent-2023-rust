use std::collections::{ HashMap, HashSet, VecDeque };

use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(_input: &str) -> Option<u64> {
    let graph = parse(_input);
    let result = graph
    .keys()
    .tuple_combinations()
    .find_map(|(from, to)| {
        let mut copy = graph.clone();

        // delete 3 routes starting at "from" and ending in "to"
        for _ in 0..3 {
            delete_route(&mut copy, from, to)
        }

        // if "from" and "to" were in 2 halves, the connecting edges were deleted and "to" will not be reachable starting at "from"
        // if this is the case, count how many nodes were reachable, this is one half. If not, move on to an other "from"-"to" pair
        let half1 = reachable_nodes(&copy, from, to)?;
        let half2 = copy.len() - half1;

        Some(half1 * half2)
    })
    .unwrap();
    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .fold(HashMap::new(), |mut acc, (from, rhs)| {
            for to in rhs.split_whitespace() {
                acc.entry(from).or_default().insert(to);
                acc.entry(to).or_default().insert(from);
            }
            acc
        })
}

fn delete_route<'a>(graph: &mut HashMap<&'a str, HashSet<&'a str>>, from: &'a str, to: &'a str) {
    let mut q = VecDeque::from([from]);
    let mut seen = HashSet::from([from]);
    let mut prev_map = HashMap::new();

    'outer: while let Some(node) = q.pop_front() {
        for &neighbour in graph.get(node).unwrap() {
            if seen.insert(neighbour) {
                q.push_back(neighbour);
                prev_map.insert(neighbour, node);
                if neighbour == to {
                    break 'outer;
                }
            }
        }
    }

    // delete every edge on the path "from"-"to"
    // if "from" and "to" were in the 2 halves, one of the connecting edges is guaranteed to be deleted this way
    let mut curr = to;
    while curr != from {
        let prev = prev_map.get(curr).unwrap();
        graph.entry(curr).or_default().remove(prev);
        graph.entry(prev).or_default().remove(curr);
        curr = prev;
    }
}

fn reachable_nodes(graph: &HashMap<&str, HashSet<&str>>, from: &str, to: &str) -> Option<usize> {
    let mut q = VecDeque::from([from]);
    let mut seen = HashSet::from([from]);

    while let Some(node) = q.pop_front() {
        for neighbour in graph.get(node).unwrap() {
            if *neighbour == to {
                // the graph was not cut in 2
                return None;
            }
            if seen.insert(neighbour) {
                q.push_back(neighbour);
            }
        }
    }

    Some(seen.len())
}
