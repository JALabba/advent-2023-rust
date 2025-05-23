use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(_input: &str) -> Option<i64> {
    let almanac = Almanac::parse(_input);
    let mut seeds = almanac.seeds.clone();
    for index in 0..almanac.maps.len() {
        seeds = seeds
            .iter()
            .map(|&value| almanac.apply_map(value, index))
            .collect();
    }
    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(_input: &str) -> Option<i64> {
    let almanac = Almanac::parse(_input);
    // let almanac = Almanac::parse(&advent_of_code::template::read_file("examples", DAY));
    let mut seeds = almanac.seeds
        .clone()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<Range<i64>>>();

    // dbg!(&seeds);
    for index in 0..almanac.maps.len() {
        seeds = seeds
            .iter()
            .map(|range| almanac.apply_range(range.clone(), index))
            .flatten()
            .collect();
        seeds = merge_ranges(seeds);
        seeds.sort_by_key(|r| r.start);
        // println!("maps: {:?}", almanac.maps[index]);
        // dbg!(&seeds);
    }
    Some(
        seeds
            .iter()
            .map(|r| r.start)
            .min()
            .unwrap()
    )
    // None
}

#[derive(Debug, PartialEq, Eq)]
struct RangeMap {
    to: i64,
    range: Range<i64>,
}

impl IntoIterator for RangeMap {
    type Item = i64;

    type IntoIter = std::ops::Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.range.into_iter()
    }
}

impl RangeMap {
    fn new(dest: i64, start: i64, length: i64) -> Self {
        Self { to: dest, range: start..start + length }
    }

    fn from_str(s: &str) -> Self {
        let parts = s
            .split_whitespace()
            .map(|p| p.parse::<i64>().ok().unwrap())
            .collect::<Vec<_>>();
        assert!(parts.len() == 3);
        RangeMap::new(parts[0], parts[1], parts[2])
    }

    fn contains(&self, value: i64) -> bool {
        self.range.contains(&value)
    }

    fn apply(&self, value: i64) -> i64 {
        if self.contains(value) {
            self.to + (value.abs_diff(self.range.start) as i64)
        } else {
            value
        }
    }

    fn overlaps(&self, range: &Range<i64>) -> bool {
        !(range.end <= self.range.start || range.start >= self.range.end)
    }

    fn overlap_split(&self, range: &Range<i64>) -> Range<i64> {
        let start = self.range.start.max(range.start);
        let end = self.range.end.min(range.end);
        start..end
    }

    fn shift(&self, range: &Range<i64>) -> Range<i64> {
        let offset = self.to.abs_diff(self.range.start) as i64;
        if self.to >= self.range.start {
            let start = range.start.saturating_add(offset);
            let end = range.end.saturating_add(offset);
            start..end
        } else {
            let start = range.start.saturating_sub(offset);
            let end = range.end.saturating_sub(offset);
            start..end
        }
    }
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<RangeMap>>,
}

impl Almanac {
    fn parse(_input: &str) -> Self {
        let seeds = _input
            .lines()
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1.split_whitespace()
            .map(|s| s.trim().parse().ok().unwrap())
            .collect::<Vec<i64>>();

        let maps: Vec<Vec<RangeMap>> = _input
            .split("\n\n")
            .skip(1)
            .map(|map| {
                let mut maps = map
                    .lines()
                    .skip(1)
                    .map(|line| RangeMap::from_str(line))
                    .collect::<Vec<_>>();
                maps.sort_by_key(|r| r.range.start);
                maps
            })
            .collect();

        Almanac { seeds, maps }
    }

    fn apply_map(&self, value: i64, index: usize) -> i64 {
        if let Some(r) = self.maps[index].iter().find(|&r| r.contains(value)) {
            r.apply(value)
        } else {
            value
        }
    }

    fn apply_range(&self, range: Range<i64>, index: usize) -> Vec<Range<i64>> {
        // tried to translate my shitty code from 2023 that passed into this, it failed.
        // This relies on sorting to look up the next possible overlapping map.

        // let mut new_ranges = vec![];
        // let mut current = range;
        // loop {
        //     // println!("{:?}", current);
        //     // sleep(Duration::from_millis(100));
        //     match self.maps[index].iter().find(|&rm| rm.range.end - 1 > current.start) {
        //         Some(rule) => {
        //             // this rule starts after current
        //             // if current ends before this rule, push and break
        //             if current.end < rule.range.start {
        //                 new_ranges.push(current);
        //                 break;
        //             }
        //             // there is some overlap
        //             let new_range = rule.overlap_split(&current);
        //             let diff = subtract_range_from(&current, &new_range);
        //             let shifted_range = rule.shift(&new_range);
        //             // complete overlap
        //             if diff == (None, None) {
        //                 new_ranges.push(shifted_range);
        //                 break;
        //             }
        //             if let Some(before) = diff.0 {
        //                 new_ranges.push(before);
        //             }
        //             // overlapping end
        //             if current.end <= rule.range.end {
        //                 break;
        //             }
        //             if let Some(after) = diff.1 {
        //                 new_ranges.push(after);
        //             }

        //             current = rule.range.end..current.end;
        //         }
        //         None => {
        //             new_ranges.push(current);
        //             break;
        //         }
        //     }
        // }
        // new_ranges.sort_by_key(|r| r.start);

        // My final solution filters first, but that is the only thing i see different from before 20 hrs of debug
        let overlapping = self.maps[index]
            .iter()
            .filter(|&rule| rule.overlaps(&range))
            .collect::<Vec<_>>();

        if overlapping.len() == 0 {
            vec![range]
        } else {
            let mut next_ranges = Vec::new();
            for rule in overlapping {
                let new_range = rule.overlap_split(&range);
                let diff = subtract_range_from(&range, &new_range);
                let shifted_range = rule.shift(&new_range);
                if shifted_range.start < shifted_range.end {
                    next_ranges.push(shifted_range);
                }
                match diff {
                    (Some(before), None) => next_ranges.extend(self.apply_range(before, index)),
                    (None, Some(after)) => next_ranges.extend(self.apply_range(after, index)),
                    (Some(before), Some(after)) => {
                        next_ranges.extend(self.apply_range(before, index));
                        next_ranges.extend(self.apply_range(after, index));
                    }
                    (None, None) => (),
                }
            }
            next_ranges
        }

        // my initial new attempt was trying it multiple ways, straight, recursive, feedback, all failed.

        // for rule in &self.maps[index] {
        //     let mut next_ranges = Vec::new();
        //     for range in new_ranges {
        //         if rule.overlaps(&range) {
        //             let new_range = rule.overlap_split(&range);
        //             let diff = subtract_range_from(&range, &new_range);
        //             let shifted_range = rule.shift(&new_range);
        //             if shifted_range.start < shifted_range.end {
        //                 next_ranges.push(shifted_range);
        //             }
        //             match diff {
        //                 (Some(before), None) => next_ranges.push(before),
        //                 (None, Some(after)) => next_ranges.push(after),
        //                 (Some(before), Some(after)) => next_ranges.extend(vec![before, after]),
        //                 (None, None) => {},
        //             }
        //         } else {
        //             next_ranges.push(range);
        //         }
        //     }
        //     next_ranges.sort_by_key(|r|r.start);
        //     new_ranges = next_ranges;
        // }
    }
}

fn subtract_range_from(
    this: &Range<i64>,
    other: &Range<i64>
) -> (Option<Range<i64>>, Option<Range<i64>>) {
    let mut result = (None, None);
    if this.start < other.start {
        result.0 = Some(this.start..other.start);
    }
    if this.end > other.end {
        result.1 = Some(other.end..this.end);
    }
    result
}

fn merge_ranges(ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range<i64>> = Vec::new();
    for range in sorted_ranges {
        if let Some(last) = merged.last_mut() {
            if last.end >= range.start {
                last.end = last.end.max(range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_almanac_parse() {
        let input = "seeds: 1 2 3 4\n\nmap1\n10 5 10\n\nmap2\n20 15 5";
        let almanac = Almanac::parse(input);
        assert_eq!(almanac.seeds, vec![1, 2, 3, 4]);
        assert_eq!(almanac.maps.len(), 2);
        assert_eq!(almanac.maps[0][0], RangeMap::new(10, 5, 10));
        assert_eq!(almanac.maps[1][0], RangeMap::new(20, 15, 5));
    }

    #[test]
    fn test_range_map_contains() {
        let range_map = RangeMap::new(10, 5, 10);
        assert!(!range_map.contains(4));
        assert!(range_map.contains(5));
        assert!(range_map.contains(14));
        assert!(!range_map.contains(15));
    }

    #[test]
    fn test_range_map_apply() {
        let range_map = RangeMap::new(10, 5, 10);
        assert_eq!(range_map.apply(4), 4);
        assert_eq!(range_map.apply(5), 10);
        assert_eq!(range_map.apply(7), 12);
        assert_eq!(range_map.apply(14), 19);
        assert_eq!(range_map.apply(16), 16);
    }

    #[test]
    fn test_range_map_split() {
        let range_map = RangeMap::new(10, 5, 10);
        let range = 7..22;
        let split_range = range_map.overlap_split(&range);
        assert_eq!(split_range, 7..15);
        let range = 7..12;
        let split_range = range_map.overlap_split(&range);
        assert_eq!(split_range, 7..12);
        let range = 0..8;
        let split_range = range_map.overlap_split(&range);
        assert_eq!(split_range, 5..8);
    }

    #[test]
    fn test_range_map_shift() {
        let range_map = RangeMap::new(10, 5, 10);
        let range = 7..12;
        let shifted_range = range_map.shift(&range);
        assert_eq!(shifted_range, 12..17);

        let range_map = RangeMap::new(10, 15, 10);
        let range = 15..20;
        let shifted_range = range_map.shift(&range);
        assert_eq!(shifted_range, 10..15);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![1..3, 2..5, 6..8];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![1..5, 6..8]);
    }

    #[test]
    fn test_subtract_range_from() {
        let range = 1..10;
        let other = 3..7;
        let result = subtract_range_from(&range, &other);
        assert_eq!(result, (Some(1..3), Some(7..10)));
        assert_eq!(subtract_range_from(&(10..20), &(15..25)), (Some(10..15), None));
        assert_eq!(subtract_range_from(&(10..20), &(5..15)), (None, Some(15..20)));
    }

    #[test]
    fn test_almanac_apply_map() {
        let input = "seeds: 1 2 3\n\nmap1\n10 5 10";
        let almanac = Almanac::parse(input);
        let result = almanac.apply_map(6, 0);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_almanac_apply_range() {
        let input = "seeds: 1 2 3\n\nmap1\n10 5 10";
        let almanac = Almanac::parse(input);
        let range = 6..8;
        let result = almanac.apply_range(range, 0);
        assert_eq!(result, vec![11..13]);

        let input = &advent_of_code::template::read_file("examples", DAY);
        let almanac = Almanac::parse(input);
        let seeds = vec![55..68, 79..93];
        // maps: [RangeMap { to: 52, start: 50, length: 48 }, RangeMap { to: 50, start: 98, length: 2 }]
        let result = almanac.apply_range(seeds[0].clone(), 0);
        let result = merge_ranges(result);
        assert_eq!(result, vec![57..70]);
        let result = almanac.apply_range(seeds[1].clone(), 0);
        let result = merge_ranges(result);
        assert_eq!(result, vec![81..95]);

        // maps: [RangeMap { to: 39, start: 0, length: 15 }, RangeMap { to: 0, start: 15, length: 37 }, RangeMap { to: 37, start: 52, length: 2 }]
        let seeds = vec![81..95, 57..70];
        let result = almanac.apply_range(seeds[0].clone(), 1);
        let result = merge_ranges(result);
        assert_eq!(result, vec![81..95]);
        let result = almanac.apply_range(seeds[1].clone(), 1);
        let result = merge_ranges(result);
        assert_eq!(result, vec![57..70]);
    }

    // #[test]
    // fn test_apply_range_custom() {
    //     let input = &advent_of_code::template::read_file("examples", DAY);
    //     let almanac = Almanac::parse(input);
    //     let result = almanac.apply_range(55..68, 0);
    //     assert_eq!(result, vec![57..70]);
    // }
}
