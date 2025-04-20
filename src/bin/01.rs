advent_of_code::solution!(1);

pub fn part_one(_input: &str) -> Option<u64> {
    Some(
        _input.lines().map(|line|{
            (10 * line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0').to_digit(10).unwrap() )
            +
            (line.chars().rev().find(|c| c.is_ascii_digit()).unwrap_or('0').to_digit(10).unwrap() )
        }).sum::<u32>() as u64
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(
        _input.lines().map(|line| {
            let mut line = line.to_owned();
            // println!("{}",line);
            let pats = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            let has_pat = |s:&str| -> Option<usize> {
                pats.iter().position(|&pat|s.contains(pat))
            };
            // forward check

            let mid = line.find(|c:char| c.is_ascii_digit()).unwrap_or(line.len()-1)+1;
            let fwd = line.clone().split_at(mid).0.to_owned();
            if fwd.len() > 2 {
                for i in 1..fwd.len() {
                    let slice = &fwd[0..i];
                    if let Some(pat_index) = has_pat(slice) {
                        line = line.replace(pats[pat_index], (pat_index +1).to_string().as_str());
                        break;
                    }
                }
            }
            // backward check
            let mid = line.char_indices().rev().find(|(_, c)| c.is_ascii_digit()).unwrap().0;
            let bck = line.clone().split_at(mid).1.to_owned();
            if bck.len() > 2 {
                for i in 1..bck.len() {
                    let slice = &bck[bck.len() - i..];
                    if let Some(pat_index) = has_pat(slice) {
                        line = line.replace(pats[pat_index], (pat_index +1).to_string().as_str());
                        break;
                    }
                }
            }
            // dbg!((fwd, bck));
            // println!("{}\n",line);

            line
        } )
        .map(|line|{
            (10 * line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0').to_digit(10).unwrap() )
            +
            (line.chars().rev().find(|c| c.is_ascii_digit()).unwrap_or('0').to_digit(10).unwrap() )
        })
        .sum::<u32>() as u64
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let s = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = part_one(s);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
