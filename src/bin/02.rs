advent_of_code::solution!(2);

pub fn part_one(_input: &str) -> Option<u64> {
    Some(
        _input
            .lines()
            .enumerate()
            .fold(0, |acc: u64, (i, line)| {
                let map = line
                    .split_once(":")
                    .unwrap()
                    .1.trim()
                    .split(';')
                    .all(|draw| {
                        draw.trim()
                            .split(',')
                            .map(|pair| { pair.trim().split_once(' ').unwrap() })
                            .all(|(num, color)| {
                                let num = num.parse::<u64>().ok().unwrap();
                                match color {
                                    "red" => num <= 12,
                                    "green" => num <= 13,
                                    "blue" => num <= 14,
                                    _ => unreachable!(),
                                }
                            })
                    });
                if map {
                    acc + ((i + 1) as u64)
                } else {
                    acc
                }
            })
    )
    // None
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(
        _input
            .lines()
            .fold(0, |acc: u64, line| {
                let mut minmax = [u64::MIN, u64::MIN, u64::MIN];
                line
                    .split_once(":")
                    .unwrap()
                    .1.trim()
                    .split(';')
                    .for_each(|draw| {
                        draw.trim()
                            .split(',')
                            .map(|pair| { pair.trim().split_once(' ').unwrap() })
                            .for_each(|(num, color)| {
                                let num = num.parse::<u64>().ok().unwrap();
                                match color {
                                    "red" => minmax[0] = minmax[0].max(num),
                                    "green" => minmax[1] = minmax[1].max(num),
                                    "blue" => minmax[2] = minmax[2].max(num),
                                    _ => unreachable!(),
                                }
                            })
                    });
                acc + (minmax.iter().product::<u64>())
            })
    )
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
