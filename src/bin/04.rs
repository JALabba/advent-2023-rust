use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(_input: &str) -> Option<u64> {
    let r = _input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(id, other)| {
                    (
                        id.split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                        other.split_once('|').unwrap(),
                    )
                })
                .map(|(_id, (one, two))| {
                    let mut g = (
                        _id,
                        one
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect::<Vec<usize>>(),
                        two
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect::<Vec<usize>>(),
                    );
                    g.1.sort();
                    g.2.sort();
                    g
                })
                .unwrap()
        })
        .fold(0, |acc, (_id, winners, yours)| {
            yours.iter().fold(0, |acc, y| {
                if winners.contains(y) {
                    if acc == 0 { 1 } else { acc * 2 }
                } else {
                    acc
                }
            }) + acc
        });
    Some(r)
    // None
}

pub fn part_two(_input: &str) -> Option<u64> {
    let cards = _input
    .lines()
    .map(|line| {
        line.split_once(':')
            .map(|(id, other)| {
                (
                    id.split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                    other.split_once('|').unwrap(),
                )
            })
            .map(|(_id, (one, two))| {
                let mut g = (
                    _id,
                    one
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<usize>>(),
                    two
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<usize>>(),
                );
                g.1.sort();
                g.2.sort();
                g
            })
            .unwrap()
    }).collect::<Vec<_>>();
    let mut copies = HashMap::new();
    let max_id = cards.len();
    cards.iter().for_each(|(id, winning, yours)|{
        let mut next = id + 1;
        let card_wins = yours.iter().filter_map(|your|{
            if winning.contains(your) && next <= max_id {
                next +=1;
                Some(next-1)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        copies.insert(id, card_wins);
    });
    let mut stack = cards.iter().map(|(id, _, _)|*id).collect::<Vec<_>>();
    let mut i = 0;
    while i < stack.len() {
        let card_id = stack[i];
        if let Some(ids) = copies.get(&card_id) {
            stack.splice(i+1..i+1, ids.clone());
        }
        i +=1;
    }
    Some(stack.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
