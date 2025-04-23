advent_of_code::solution!(6);

pub fn part_one(_input: &str) -> Option<u64> {
    Some(
        _input
            .split_once('\n')
            .iter()
            .map(|(one, two)| {
                one.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u64>().ok().unwrap())
                    .zip(
                        two
                            .split_whitespace()
                            .skip(1)
                            .map(|s| s.parse::<u64>().ok().unwrap())
                    )
            })
            .flatten()
            .map(|(time, distance)| {
                let mut charge = 0;
                let mut wins = 0;
                while charge < time {
                    charge += 1;
                    let travel = charge * (time - charge);
                    if travel > distance {
                        wins += 1;
                    }
                }
                wins
            })
            .product()
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(
        _input
            .split_once('\n')
            .iter()
            .map(|(one, two)| {
                (
                    one.split_whitespace()
                        .skip(1)
                        .collect::<String>()
                        .parse::<u64>().ok().unwrap()
                    ,
                    two.split_whitespace()
                        .skip(1)
                        .collect::<String>()
                        .parse::<u64>().ok().unwrap()
                )

            })
            .map(|(time, distance)| {
                let mut charge = 0;
                let mut wins = 0;
                while charge < time {
                    charge += 1;
                    let travel = charge * (time - charge);
                    if travel > distance {
                        wins += 1;
                    }
                }
                wins
            }).fold(0, |_, r| r)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
