use std::collections::HashMap;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mountain = Mountain::parse(input);
    let res = mountain.count();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mountain = Mountain::parse(input);
    let res = mountain.fix_and_count();
    Some(res as u64)
}

struct Pattern {
    lines: Vec<String>,
    refl_horizontal: usize,
    refl_vertical: usize,
}

impl Pattern {
    fn parse_section(section: &str) -> Pattern {
        let normal: Vec<String> = section
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Pattern {
            lines: normal,
            refl_horizontal: 0,
            refl_vertical: 0,
        }
    }

    fn evaluate(&mut self) -> usize {
        //vertical, flip to scan
        if let Some(value) = search_reflection(flip(self.lines.clone()), None) {
            self.refl_vertical = value;
            return value;
        }
        //horizontal
        if let Some(value) = search_reflection(self.lines.clone(), None) {
            self.refl_horizontal = value;
            return value * 100;
        }
        0
    }

    fn evaluate_smudges(&self) -> usize {
        // this pattern must :
        // find smudges both vertical and horizontal
        // look for a new pattern that is not the same as before

        //vertical
        let v = find_smudged_reflection(flip(self.lines.clone()), &self.refl_vertical);
        //horizontal
        let h = find_smudged_reflection(self.lines.clone(), &self.refl_horizontal) * 100;

        if (v > h && h != 0) || (v < h && v != 0) {
            println!("conflict");
        }
        v + h
    }
}

fn find_smudged_reflection(lines: Vec<String>, prev_reflection: &usize) -> usize {
    //must find a reflection that is not prev_reflection

    // first look for smudges, indexes for a line where one off char from any other line exist
    let mut smudge_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..lines.len() {
        let mut i_smudges = Vec::new();
        for j in 0..lines.len() {
            if j == i {
                continue;
            }
            if let Some(smudge) = one_off(&lines[i], &lines[j]) {
                // check one line moving towards eachother? fail
                i_smudges.push(smudge);
            }
        }
        smudge_map.insert(i, i_smudges);
    }

    for (line_index, smudges) in smudge_map {
        for smudge in smudges {
            let mut pattern_dot: Vec<String> = lines.clone();
            let mut char = pattern_dot[line_index].chars().nth(smudge).unwrap();
            char = if char == '#' { '.' } else { '#' };
            let mut changed_vec: Vec<char> = pattern_dot[line_index].chars().collect();
            changed_vec[smudge] = char;
            let new_string = changed_vec.into_iter().collect();
            pattern_dot[line_index] = new_string;

            //find horizontal
            if let Some(value) = search_reflection(pattern_dot, Some(*prev_reflection)) {
                if value != *prev_reflection {
                    return value;
                }
            }
        }
    }

    0
}

// fn print_matrix(lines: &Vec<String>, label: &str) {
//     println!();
//     println!("Matrix: {}", label);
//     for line in lines {
//         println!("{}", line);
//     }
//     println!();
// }

fn flip(matrix: Vec<String>) -> Vec<String> {
    let mut flipped: Vec<String> = Vec::new();

    for i in 0..matrix[0].len() {
        let mut row = String::new();
        for j in (0..matrix.len()).rev() {
            row.push_str(&matrix[j].chars().nth(i).unwrap().to_string());
        }
        flipped.push(row);
    }
    flipped
}

fn one_off(first: &String, second: &String) -> Option<usize> {
    if first.len() != second.len() {
        return None; // Strings must be of equal length for comparison
    }

    let mut differing_index: Option<usize> = None;
    let mut differing_count = 0;

    for (index, (char_first, char_second)) in first.chars().zip(second.chars()).enumerate() {
        if char_first != char_second {
            differing_count += 1;
            differing_index = Some(index);

            // If more than one difference is found, return None immediately
            if differing_count > 1 {
                return None;
            }
        }
    }

    differing_index
}

fn search_reflection(pattern: Vec<String>, skip: Option<usize>) -> Option<usize> {
    'outer: for index in 0..pattern.len() {
        // skip first
        if index == 0 {
            continue 'outer;
        }
        if let Some(skip) = skip {
            if index == skip {
                continue 'outer;
            }
        }

        // check backwards and forwards one step at a time from index,
        //starting with index-1 and index until either is out of bounds
        let mut backwards = index - 1;
        let mut forwards = index;

        loop {
            let line_backwards = &pattern[backwards];
            let line_forwards = &pattern[forwards];

            if line_backwards != line_forwards {
                // if any of those lines dont match, then there is not a reflection at index,
                continue 'outer;
            }

            if backwards == 0 || forwards == pattern.len() - 1 {
                // If the loop completes, it means we found a reflection at the current index
                return Some(index);
            } else {
                backwards = backwards.checked_sub(1)?;
                forwards += 1;
            }
        }
    }
    None
}

struct Mountain {
    patterns: Vec<Pattern>,
}

impl Mountain {
    fn parse(input: &str) -> Mountain {
        let mut patterns = vec![];

        // Split input into sections based on empty lines
        let sections: Vec<&str> = input.trim().split("\n\n").collect();

        for section in sections {
            let pattern = Pattern::parse_section(section);
            patterns.push(pattern);
        }

        Mountain { patterns }
    }

    fn count(&mut self) -> usize {
        let mut count = 0;
        for pattern in 0..self.patterns.len() {
            let rest = self.patterns[pattern].evaluate();
            count += rest;
        }
        count
    }

    fn fix_and_count(&mut self) -> usize {
        for pattern_index in 0..self.patterns.len() {
            self.patterns[pattern_index].evaluate();
        }
        let mut count = 0;

        for pattern_index in 0..self.patterns.len() {
            let reflection = self.patterns[pattern_index].evaluate_smudges();
            count += reflection;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    // #[test]
    // fn test_unexpected_zeroes() {
    //     let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
    //     assert_eq!(result, Some(400));
    // }

    #[test]
    fn test_unexpected_zeroes_2() {
        //original
        // ##..#.##..###
        // ....##...#.##
        // ##.....##..##
        // ...#..##..###
        // ####..##..#..
        // ..##..##.#.#.
        // ...##.#...###

        //flipped
        // ..#.#.#
        // ..#.#.#
        // .##....
        // ####...
        // #....##
        // .....#.
        // ####..#
        // .####.#
        // ....#..
        // .#...#.
        // #.##..#
        // #X.####
        // #..####

        // original
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(12));
    }
}
