use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(_input: &str) -> Option<u64> {
    let mut result = parse_card_hands(_input);
    result.sort_by(|a, b| {
        match a.2.cmp(&b.2) {
            std::cmp::Ordering::Equal => (),
            ord => {
                return ord;
            }
        }
        for i in 0..5 {
            match a.0[i].cmp(&b.0[i]) {
                std::cmp::Ordering::Equal => (),
                ord => {
                    return ord;
                }
            }
        }
        std::cmp::Ordering::Equal
    });
    Some(
        result
            .iter()
            .enumerate()
            .map(|(i, (_, bid, _))| bid * ((i as u64) + 1))
            .sum()
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut result = parse_card_hands(_input);
    for hand in result.iter_mut() {
        for card in &mut hand.0 {
            if card == &0xb {
                *card = 0x1;
            }
        }
        let mut map: HashMap<u8, u8> = HashMap::new();
        let mut jokers = 0;
        for card in hand.0.clone() {
            if card == 0x1 {
                jokers += 1;
            } else {
                *map.entry(card).or_insert(0) += 1;
            }
        }
        let hand_type = match map.len() {
            0..=1 => HandType::FiveOfAKind,
            2 => {
                let max_count = *map.values().max().unwrap();
                if max_count == 4 {
                    // there can only be 4 max count if no jokers
                    HandType::FourOfAKind
                } else if jokers >= 1 {
                    //add x to xxyy
                    //add x to xxxy
                    //add xx to xxy
                    //add xxx to xy
                    match jokers {
                        1 => {
                            if max_count == 3 {
                                HandType::FourOfAKind
                            } else {
                                HandType::FullHouse
                            }
                        }
                        2..=3 => HandType::FourOfAKind,
                        _ => panic!("too many jokers"),
                    }
                } else {
                    // no jokers, must be full house
                    HandType::FullHouse
                }
            }
            3 => {
                let max_count = *map.values().max().unwrap();
                if max_count == 3 {
                    HandType::ThreeOfAKind
                } else if jokers >= 1 {
                    //add x to xxyz
                    //add xx to xyz
                    match jokers {
                        1 => HandType::ThreeOfAKind,
                        2 => HandType::ThreeOfAKind,
                        _ => panic!("number of jokers is wrong"),
                    }
                } else {
                    HandType::TwoPair
                }
            }
            //joker can only make a pair
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => HandType::Null,
        };
        hand.2 = hand_type;
    }
    result.sort_by(|a, b| {
        match a.2.cmp(&b.2) {
            std::cmp::Ordering::Equal => (),
            ord => {
                return ord;
            }
        }
        for i in 0..5 {
            match a.0[i].cmp(&b.0[i]) {
                std::cmp::Ordering::Equal => (),
                ord => {
                    return ord;
                }
            }
        }
        std::cmp::Ordering::Equal
    });
    Some(
        result
            .iter()
            .enumerate()
            .map(|(i, (_, bid, _))| bid * ((i as u64) + 1))
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Null = 0,
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn parse_card_hands(_input: &str) -> Vec<(Vec<u8>, u64, HandType)> {
    _input
        .lines()
        .map(|line| { line.split_once(" ").unwrap() })
        .map(|(hand, bid)| {
            (
                hand
                    .chars()
                    .map(|ch| {
                        match ch {
                            '2'..='9' => ch.to_digit(10).unwrap() as u8,
                            'A' => 0xe,
                            'T' => 0xa,
                            'J' => 0xb,
                            'Q' => 0xc,
                            'K' => 0xd,
                            _ => panic!("Invalid card value"),
                        }
                    })
                    .collect::<Vec<u8>>(),
                bid.parse::<u64>().ok().unwrap(),
            )
        })
        .map(|(hand, bid)| {
            let mut counts: HashMap<u8, u8> = HashMap::new();
            hand.iter().for_each(|v| {
                *counts.entry(*v).or_insert(0) += 1;
            });
            let max = *counts.values().max().unwrap();
            let hand_type = match counts.len() {
                1 => HandType::FiveOfAKind,
                2 => {
                    if max == 4 { HandType::FourOfAKind } else { HandType::FullHouse }
                }
                3 => {
                    if max == 3 { HandType::ThreeOfAKind } else { HandType::TwoPair }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!(),
            };
            (hand, bid, hand_type)
        })
        .collect::<Vec<_>>()
}
