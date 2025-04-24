use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let field: Field = Field::parse(input);
    let result = field.count_permutations(1);
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let field: Field = Field::parse(input);
    let result = field.count_permutations(5);
    Some(result as u64)
}

//RULES PART 1:
// Each row consists of two parts separated by a whitespace
// The first part is a string of .#?, where # is a damaged part, and ? is either a # part or .
// A continuous string of # is a group. groups are always separated by at least one . part
// The second part of each row are records that show the size of each contiguous group of damaged part in order
// it accounts for all damaged springs, and the number is the entire contiguous group.
// the goal is to find all permutations of ? to # or . that satisfy the rules

struct Row {
    conditions: Vec<char>,
    records: Vec<usize>,
}

struct Field {
    rows: Vec<Row>,
}

impl Field {
    fn parse(input: &str) -> Field {
        let mut field = Field { rows: vec![] };
        for row in input.lines() {
            // example row
            //.??..??...?##. 1,1,3
            let parts: Vec<&str> = row.split_whitespace().map(|f| f.trim()).collect();
            //.??..??...?##.
            let conditions = parts[0].chars().collect::<Vec<_>>();
            //vec![1,1,3]
            let records: Vec<usize> = parts[1]
                .split(',')
                .map(|c| c.parse().expect("number in records parse"))
                .collect();
            let new_row: Row = Row {
                conditions,
                records,
            };
            field.rows.push(new_row);
        }
        field
    }

    fn count_permutations(&self, repeat: usize) -> usize {
        let mut count = 0;
        for row in &self.rows {
            let mut line: Vec<char> = Vec::new();
            let mut records: Vec<usize> = Vec::new();
            if repeat > 0 {
                for i in 0..repeat {
                    if i > 0 {
                        line.push('?');
                    }
                    line.append(&mut row.conditions.clone());
                    records.append(&mut row.records.clone());
                }
            } else {
                return 0;
            }
            // print!("line: {:?}", line.iter().collect::<String>());
            // println!(" {:?}", records);
            let res = rhash(&line, &records, &mut HashMap::new());
            // println!("result: {}", res);
            // println!("");
            count += res;
        }
        count
    }
}

fn rhash(line: &[char], records: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    //check in cache, return cached
    if let Some(count) = cache.get(&(line.len(), records.len())) {
        return *count;
    }

    let mut count = 0;

    // no more records to find
    if records.is_empty() {
        count = if line.contains(&'#') { 0 } else { 1 };
        cache.insert((line.len(), records.len()), count);
        return count;
    }

    //check line
    for pointer in 0..line.len() {
        let group_size = pointer + records[0];

        // ?? i don't know why to check behind from zero to here
        if line[0..pointer].contains(&'#')
        // can't find group in remaining
        || group_size > line.len()
        {
            break; //out of for loop
        }

        //can't find group ahead if it contains .
        if line[pointer..group_size].contains(&'.') {
            continue;
        }

        if records.len() == 1 {
            if group_size == line.len() {
                //group must be all of remaining
                count += 1;
                break;
            } else {
                //recurse to check and cache line
                count += rhash(&line[group_size..], &[], cache);
                continue;
            }
        } else if group_size + 1 > line.len() {
            //pointer + size has gone out of bounds
            break;
        } else if line[group_size] == '#' {
            // can't enclose a group here, must continue traverse
            continue;
        }

        // group found. Recurse line with a spare, one less record to find
        count += rhash(&line[group_size + 1..], &records[1..], cache);
    }
    cache.insert((line.len(), records.len()), count);

    count
}

// fn count_permutations(&mut self) -> usize {
//     let mut count = 0;
//     let rows = &mut self.rows;
//     for row in rows.iter_mut() {
//         let mut line = row.conditions.clone();
//         let records = &row.records;
//         let res = recurse(&mut line, records, 0);
//         count += res;
//     }
//     count
// }
// fn recurse(line: &mut Vec<char>, records: &Vec<usize>, pointer: usize) -> usize {
//     // end of line, base case if valid return 1 permutation
//     if pointer == line.len() {
//         if is_group(line, records) {
//             return 1;
//         } else {
//             return 0;
//         }
//     }
//     //inflection point for recursing
//     //call twice to exhaust possibilities
//     if line[pointer] == '?' {
//         line[pointer] = '#';
//         println!("Inflect #, for line {:?} at pointer {}",line, pointer);
//         let one = recurse(line, records, pointer + 1);
//         line[pointer] = '.';
//         println!("Inflect ., for line {:?} at pointer {}",line, pointer);
//         let two = recurse(line, records, pointer + 1);
//         return one + two;
//     } else {
//         //not undetermined, keep walking
//         return recurse(line, records, pointer + 1);
//     }
// }

// fn is_group( line: &Vec<char>, records: &Vec<usize>) -> bool {
//     //decides if a line meets the criteria
//     let mut current: usize = 0;
//     let mut seen: Vec<usize> = Vec::new();

//     print!("is group: line: ");
//     //walk over the line
//     for char in line {
//         print!("{char}");
//         match char {
//             '#' => {
//                 current += 1;
//             }
//             '.' => if current > 0 {
//                 seen.push(current);
//                 current = 0;
//             }
//             _ => {
//                 //failure
//                 return false;
//             }
//         }
//     }
//     // push if any remaining
//     if current > 0 {
//         seen.push(current);
//     }
//     let res  = seen == *records;
//     println!(", res {}", res);
//     res
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn one_line() {
        // An example line followed by it's permutations, 10
        // ?###???????? 3,2,1
        // .###.##.#...
        // .###.##..#..
        // .###.##...#.
        // .###.##....#
        // .###..##.#..
        // .###..##..#.
        // .###..##...#
        // .###...##.#.
        // .###...##..#
        // .###....##.#

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}

// fn count_permutations(&mut self) -> usize {
//     let mut count = 0;
//     for row in &self.rows {
//         let mut line = row.conditions.clone();
//         count += self.count_recursive(&mut line, 0, &row.records);
//     }
//     count
// }

// fn count_recursive(
//     &self,
//     line: &mut Vec<char>,
//     pointer: usize,
//     remaining_numbers: &[usize]
// ) -> usize {
//     // Base case: All numbers matched
//     if remaining_numbers.is_empty() {
//         return 1;
//     }

//     let mut count = 0;

//     for &size in remaining_numbers {

//         let group_end = pointer + size;
//         if group_end <= line.len() {
//             let find_group_in_slice = line[pointer..group_end]
//                 .iter()
//                 .all(|&c| (c == '#' || c == '?'));

//             if find_group_in_slice {

//                 for i in pointer..group_end {
//                     match line[i] {
//                         '?'=> line[i] = '#',
//                         '#'=>(),
//                         _=> panic!("Fault in group finding"),
//                     }
//                 }

//                 let new_pointer = group_end + 1;
//                 let new_remaining_numbers = &remaining_numbers[1..];

//                 count += self.count_recursive(
//                     line,
//                     new_pointer,
//                     new_remaining_numbers
//                 );

//             }
//         }

//     }
//     count
// }

//fn py_recurse(mut field: Field) ->usize {
//     let mut count = 0;
//     for row in &mut field.rows {
//         let line = row.conditions.clone();
//         count += f(line, row.records.clone(), 0);
//     }
//     count
// }

// fn f( line: Vec<char>, records: Vec<usize>, pointer: usize)-> usize{
//     if pointer == line.len() {
//         if is_valid(line, records) {
//             return 1;
//         } else {
//             return 0;
//         }
//     }
//     if line[pointer] == '?' {
//         let mut line1 = line.clone();
//         let mut line2 = line.clone();
//         line1[pointer] = '#';
//         line2[pointer] = '.';
//         return f(line1, records.clone(), pointer+1) +
//                 f(line2, records, pointer+1)
//     } else {
//         return f(line, records, pointer+1)
//     }
// }

// fn is_valid(line: Vec<char>, records: Vec<usize>) -> bool {
//     let mut current:usize  = 0;
//     let mut seen: Vec<usize> = Vec::new();
//     for char in line {
//         if char == '.' {
//             if current > 0 {
//                 seen.push(current);
//             }
//         } else if char == '#' {
//             current +=1;
//         } else {
//             return false;
//         }
//     }
//     if current > 0 {
//         seen.push(current);
//     }
//     return seen == records
// }
