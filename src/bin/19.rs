use std::collections::{HashMap, VecDeque};

// #[allow(dead_code)]
// const INPUT: &str = include_str!("../../data/inputs/19.txt");

advent_of_code::solution!(19);

pub fn part_one(_input: &str) -> Option<u32> {
    // let mut system = parse(&advent_of_code::template::read_file("examples", DAY));
    let mut system = parse(_input);
    let result = system.process();
    Some(result as u32)
    //389114
}

pub fn part_two(_input: &str) -> Option<u64> {
    // let input = include_str!("../../data/inputs/19.txt");

    let system = parse(_input);
    // let system = parse(&advent_of_code::template::read_file("examples", DAY));

    let result: u64 = system.non_recursive_precalculate();

    // working recursion type https://github.com/Zemogus/AOC-2023/blob/main/src/day19.rs
    // let workflows = parse2();
    // let result: u64 = count_matched(&workflows, "in", [(1,4001);4]);
    Some(result)
    //125051049836302
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}

struct System {
    workflows: HashMap<String, Vec<(String, Status)>>,
    parts: Vec<Part1>,
}
enum Ruling {
    Accept,
    Reject,
    Send(String),
    Next,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Status {
    Accepted,
    Rejected,
    Undetermined,
}

struct Part1 {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    accepted: Status,
}

type PartRange = [(u16, u16); 4];

struct Instruction {
    index: usize,
    num: u16,
    sign: char,
    address: String,
}

impl System {
    fn process(&mut self) -> usize {
        let mut iter_parts = self.parts.iter_mut().enumerate();

        'parts: while let Some((_id, part)) = iter_parts.next() {
            // println!("new part id {}", _id);
            let mut current_workflow: String = "in".to_string();
            'workflows: loop {
                // println!("looking at workflow: {}", current_workflow);
                if let Some(rules) = self.workflows.get(&current_workflow) {
                    for i in 0..rules.len() {
                        // println!("rule {}", i);
                        let ruling = apply_rule(rules[i].0.clone(), part);
                        match ruling {
                            Ruling::Accept => {
                                // println!("accept");
                                part.accepted = Status::Accepted;
                                continue 'parts;
                            }
                            Ruling::Reject => {
                                // println!("reject");
                                part.accepted = Status::Rejected;
                                continue 'parts;
                            }
                            Ruling::Send(address) => {
                                // println!("send to {}", address);
                                current_workflow = address;
                                continue 'workflows;
                            }
                            Ruling::Next => (),
                        }
                    }
                    panic!("last rule must apply, doesn't.");
                }
            }
        }
        let mut accumulator = 0;

        for i in 0..self.parts.len() {
            let part = &self.parts[i];
            if part.accepted == Status::Accepted {
                accumulator += part.x + part.m + part.a + part.s;
            }
        }
        accumulator
    }

    #[allow(dead_code)]
    fn non_recursive_precalculate(&self) -> u64 {
        let mut count = 0;

        fn advance_current_rule(current_rule: &String) -> String {
            let last_char = current_rule.chars().last();
            if let Some(ch) = last_char {
                if let Some(digit) = ch.to_digit(10) {
                    let new_digit = digit + 1;
                    if new_digit > 9 {
                        panic!("Number exceeds 9, cannot advance rule.");
                    }
                    let mut new_rule = current_rule.clone();
                    new_rule.pop(); // Remove the last character
                    new_rule.push_str(&new_digit.to_string()); // Append the new digit
                    return new_rule;
                } else {
                    return current_rule.clone() + &0.to_string();
                }
            }
            // If the last character is not a digit or something goes wrong, return the input unchanged
            current_rule.clone()
        }

        let range: (u16, u16) = (1, 4001);
        let mut range_instructions: HashMap<String, Instruction> = HashMap::new();
        for (workflow_name, rules) in &self.workflows {
            for (i, _rule) in rules.iter().enumerate() {
                if let Some((wf_idx, num, sign, _ruling, address)) = parse_rule(rules[i].0.clone()) {
                    let key = workflow_name.clone() + &i.to_string();
                    let instruction = Instruction { index: wf_idx, num, sign, address };
                    range_instructions.insert(key, instruction);
                }
            }
        }

        let mut hash_ranges: HashMap<String, PartRange> = HashMap::new();
        let mut current_rule: String = "in0".to_string();
        hash_ranges.insert(current_rule.clone(), [range;4]);
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(current_rule.clone());
        let mut other_queue: VecDeque<String> = VecDeque::new();

        loop {
            if !queue.is_empty() {
                current_rule = queue.pop_front().unwrap();
            } else if !other_queue.is_empty() {
                current_rule = other_queue.pop_front().unwrap();
                // println!("returning {}", current_rule);
            } else {
                // println!("empty queues");
                break;
            }
            // print!(" {} :", &current_rule.clone());
            let mut part = *hash_ranges.get(&current_rule).unwrap();
            // println!(" {:?}", part);
            match current_rule.as_str() {
                "R0" => continue,
                "A0" =>{
                    let add: u64 = product_part(part);
                    count += add;
                    // println!("adding {}", add);
                    continue;
                },
                _=> ()
            }
            let mut counterpart = part.clone();

            let Instruction { index, num, sign, address } = range_instructions
                .get(&current_rule)
                .unwrap();


            match sign {
                '<' => {
                    part[*index].1 = *num;
                    counterpart[*index].0 = *num;
                }
                '>' => {
                    part[*index].0 = *num + 1;
                    counterpart[*index].1 = *num +1;
                }
                _ => (),
            }

            match sign {
                '<' | '>' => {
                    // range rules split into two
                    // follow the new path
                    let follow = address.to_string() + &0.to_string();
                    hash_ranges.insert(follow.clone(), part);
                    queue.push_back(follow);

                    // advance this rule to next
                    let opposite = advance_current_rule(&current_rule);
                    hash_ranges.insert(opposite.clone(), counterpart);
                    other_queue.push_front(opposite);
                }
                _ => {
                    // simple rules apply always.
                    // A and R becomes A0 and R0
                    let follow = address.to_string() + &0.to_string();
                    hash_ranges.insert(follow.clone(), part);
                    queue.push_back(follow);
                },
            }

        }

        count
    }


    // fn start_recursion(&self) -> u64 {
    //     recursive("in".to_string(), [(1,4000);4], &self.workflows)
    // }
}

#[allow(dead_code)]
fn product_part(part: [(u16, u16); 4]) -> u64 {
    let mut count = 0;
    //ranges are inclusive
    for (gt, lt) in part {
        if lt < gt {
            return 0;
        } else {
            let res = (lt - gt ) as u64;
            if count == 0 {
                count += res;
            } else {
                count *= res;
            }
        }
    }
    count
}

// fn recursive(name: String, mut part: PartRange, workflows: &WorkflowMap) -> u64 {
//     match name.as_str() {
//         "R"=> return 0,
//         "A"=> return count_part2(part),
//         _=>()
//     }
//     let workflow = workflows.get(&name).unwrap();
//     let mut count = 0;
//     for rule in workflow {
//         let mut matched = part;
//         if let Some((wf_idx, num, sign, _ruling, address)) = find_range(rule.clone()){
//             // first update part if applicable.
//             // ranges are stored as inclusive, so must compensate +-1
//             match sign {
//                 '>' => {
//                     part[wf_idx].1 = num +1;
//                     matched[wf_idx].0 = num +1;
//                 },
//                 '<' => {
//                     part[wf_idx].0 = num ;
//                     matched[wf_idx].1 = num ;
//                 },
//                 _=> ()
//             }
//             count += recursive(address, matched, workflows);
//             match sign {
//                 '<' | '>' =>(),
//                 _=> break,
//             }
//         }
//     }
//     count
// }

// fn count_part2(part: [(u16, u16); 4]) -> u64 {
//     let mut count = 0;
//     //ranges are inclusive
//     for (gt, lt) in part {
//         if lt < gt {
//             return 0
//         } else {
//             count+=  (lt - gt) as u64;
//         }
//     }
//     count
// }

#[allow(dead_code)]
fn parse_rule(input: String) -> Option<(usize, u16, char, Ruling, String)> {
    if input.contains(':') {
        let (condition, address) = input.split_once(':').unwrap();
        if condition.contains('<') {
            let (letter_str, number) = condition.split_once('<').unwrap();
            let letter = letter_str.chars().next().unwrap();

            let pi = match letter {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            };

            let ru = match address {
                "A" => Ruling::Accept,
                "R" => Ruling::Reject,
                _ => Ruling::Send(address.to_string()),
            };
            Some((pi, number.parse().unwrap(), '<', ru, address.to_string()))
        } else {
            let (letter_str, number) = condition.split_once('>').unwrap();
            let letter = letter_str.chars().next().unwrap();

            let pi = match letter {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            };

            let ru = match address {
                "A" => Ruling::Accept,
                "R" => Ruling::Reject,
                _ => Ruling::Send(address.to_string()),
            };
            Some((pi, number.parse().unwrap(), '>', ru, address.to_string()))
        }
    } else {
        match input.as_str() {
            "A" => Some((0, 0, 'A', Ruling::Accept, input)),
            "R" => Some((0, 0, 'R', Ruling::Reject, input)),
            _ => Some((0, 0, '_', Ruling::Send(input.clone()), input)),
        }
    }
}

fn parse(input: &str) -> System {
    let mut sections = input.split("\n\n").peekable();

    let mut workflows: HashMap<String, Vec<(String, Status)>> = HashMap::new();

    for line in sections.next().unwrap().lines() {
        let parts = line.trim_end_matches('}').split('{').collect::<Vec<_>>();
        let name = parts[0];
        let rules: Vec<(String, Status)> = parts[1]
            .split(',')
            .map(|item| (item.trim().to_string(), Status::Undetermined))
            .collect();

        if workflows.contains_key(name) {
            panic!("duplicate names");
        }
        workflows.insert(name.to_string(), rules);
    }

    let mut parts_map: Vec<Part1> = vec![];

    for (_id, line) in sections.next().unwrap().lines().enumerate() {
        let parts: Vec<&str> = line
            .trim()
            .trim_matches(|c| (c == '{' || c == '}'))
            .split(',')
            .collect();
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for part in parts {
            let (letter, number) = part.split_once('=').unwrap();
            match letter.chars().next().unwrap() {
                'x' => {
                    x = number.parse().unwrap();
                }
                'm' => {
                    m = number.parse().unwrap();
                }
                'a' => {
                    a = number.parse().unwrap();
                }
                's' => {
                    s = number.parse().unwrap();
                }
                _ => unreachable!(),
            }
        }
        parts_map.push(Part1 { x, m, a, s, accepted: Status::Undetermined });
    }

    System { workflows, parts: parts_map }
}

fn apply_rule(input: String, part: &Part1) -> Ruling {
    //x>10:one => x > 10 , send to "one" in workflows
    //a>30:R => a > 30, set reject, stop processing this part.
    //A => accept, stop process
    //rfg => send to rfg

    if input.contains(':') {
        let (condition, address) = input.split_once(':').unwrap();

        if condition.contains('<') {
            let (letter_str, number) = condition.split_once('<').unwrap();
            let letter = letter_str.chars().next().unwrap();

            let res = match letter {
                'x' => part.x < number.parse().unwrap(),
                'm' => part.m < number.parse().unwrap(),
                'a' => part.a < number.parse().unwrap(),
                's' => part.s < number.parse().unwrap(),
                _ => unreachable!(),
            };
            if res {
                match address {
                    "A" => Ruling::Accept,
                    "R" => Ruling::Reject,
                    _ => Ruling::Send(address.to_string()),
                }
            } else {
                Ruling::Next
            }
        } else if condition.contains('>') {
            let (letter_str, number) = condition.split_once('>').unwrap();
            let letter = letter_str.chars().next().unwrap();

            let res = match letter {
                'x' => part.x > number.parse().unwrap(),
                'm' => part.m > number.parse().unwrap(),
                'a' => part.a > number.parse().unwrap(),
                's' => part.s > number.parse().unwrap(),
                _ => unreachable!(),
            };
            if res {
                match address {
                    "A" => Ruling::Accept,
                    "R" => Ruling::Reject,
                    _ => Ruling::Send(address.to_string()),
                }
            } else {
                Ruling::Next
            }
        } else {
            unreachable!()
        }
    } else {
        match input.as_str() {
            "A" => Ruling::Accept,
            "R" => Ruling::Reject,
            _ => Ruling::Send(input),
        }
    }
}

// #[allow(dead_code)]
// type Part2 = [u16;4];
// type PartRange = [(u16,u16);4];
// type Workflow = [(Condition, &'static str); 4];
// type WorkflowMap = HashMap<&'static str, Workflow>;
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Condition {
//     Gt(u8, u16),
//     Lt(u8, u16),
//     True,
// }

// impl Condition {
//     const CAT_MAP: [char; 4] = ['x', 'm', 'a', 's'];
// }

// impl std::str::FromStr for Condition {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let split = if s.contains('<') {
//             '<'
//         } else if s.contains('>') {
//             '>'
//         } else {
//             return Err(());
//         };

//         let cat = Condition::CAT_MAP
//             .iter()
//             .position(|&c| c == s.chars().next().unwrap())
//             .unwrap();
//         let tresh = s[2..].parse().unwrap();
//         match split {
//             '<' => Ok(Condition::Lt(cat as u8, tresh)),
//             '>' => Ok(Condition::Gt(cat as u8, tresh)),
//             _ => unreachable!(),
//         }
//     }
// }

// fn parse2() -> WorkflowMap {
//     let (workflows, _) = INPUT.split_once("\n\n").unwrap();

//     let mut workflow_map = HashMap::new();
//     for line in workflows.lines() {
//         let (name, rest) = line.split_once('{').unwrap();

//         let mut rules = [(Condition::True, ""); 4];
//         for (i, rule) in rest[..rest.len() - 1].split(',').enumerate() {
//             rules[i] = if !rule.contains(':') {
//                 (Condition::True, rule)
//             } else {
//                 let (cond, dest) = rule.split_once(':').unwrap();
//                 (cond.parse().unwrap(), dest)
//             };
//         }

//         workflow_map.insert(name, rules);
//     }
//     workflow_map
// }

// fn count_matched(workflows: &WorkflowMap, workflow_name: &str, mut range: PartRange) -> u64 {
//     if workflow_name == "R" {
//         return 0;
//     }
//     if workflow_name == "A" {
//         return range
//             .iter()
//             .map(|(start, end)| (end - start) as u64)
//             .product();
//     }

//     let workflow = workflows.get(workflow_name).unwrap();
//     let mut count = 0;

//     for rule in workflow {
//         let mut matched = range;
//         match rule.0 {
//             Condition::True => (),
//             Condition::Lt(cat, tresh) => {
//                 matched[cat as usize].1 = tresh;
//                 range[cat as usize].0 = tresh;
//             }
//             Condition::Gt(cat, tresh) => {
//                 matched[cat as usize].0 = tresh + 1;
//                 range[cat as usize].1 = tresh + 1;
//             }
//         }
//         count += count_matched(workflows, rule.1, matched);

//         if rule.0 == Condition::True {
//             break;
//         }
//     }

//     count
// }
