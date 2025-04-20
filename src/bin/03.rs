use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(3);

pub fn part_one(_input: &str) -> Option<u64> {
    let (numbers, symbols) = parse_numbers_and_symbols(_input);
    let adjacencies = find_adjacencies(numbers, symbols);
    let sum = calc_sum(&adjacencies);
    // let product = calc_product(&adjacencies);
    Some(sum as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (numbers, symbols) = parse_numbers_and_symbols(_input);
    let adjacencies = find_adjacencies(numbers, symbols);
    // let sum = calc_sum(&adjacencies);
    let product = calc_product(&adjacencies);
    Some(product as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Number {
    line: usize,
    range: Range<usize>,
    value: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Symbol {
    line: usize,
    index: usize,
    ch: char,
}

fn parse_numbers_and_symbols(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    fn to_range(sequence: &[usize]) -> Range<usize> {
        let first = *sequence.first().unwrap();
        let last = *sequence.last().unwrap() + 1;
        first..last
    }
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let mut sequence: Vec<usize> = Vec::new();

        for (index, ch) in line.chars().enumerate() {
            let is_digit = ch.is_ascii_digit();

            if is_digit {
                sequence.push(index);
            } else {
                if ch != '.' {
                    symbols.push(Symbol {
                        line: idx,
                        index,
                        ch,
                    });
                }
                if sequence.is_empty() {
                    continue; // Skip .... characters
                } else {
                    let range: Range<usize> = to_range(&sequence);
                    sequence.clear();

                    numbers.push(Number {
                        line: idx,
                        range: range.clone(),
                        value: line[range].parse().unwrap_or_default(),
                    });
                }
            }
        }

        // Check for the last sequence
        if !sequence.is_empty() {
            let range: Range<usize> = to_range(&sequence);
            sequence.clear();
            numbers.push(Number {
                line: idx,
                range: range.clone(),
                value: line[range].parse().unwrap_or_default(),
            });
        }
    }
    (numbers, symbols)
}


fn calc_product(adjacencies: &HashMap<Symbol, Vec<Number>>) -> usize {
    let mut accumulator = 0;

    for (s, vec) in adjacencies {
        if s.ch == '*' && vec.len() == 2 {
            let n: usize = vec.iter().map(|n| n.value).product();
            accumulator += n;
        }
    }

    accumulator
}

fn calc_sum(adjacencies: &HashMap<Symbol, Vec<Number>>) -> usize {
    let mut accumulator = 0;
    for vec in adjacencies.values() {
        vec.iter().for_each(|n| {
            accumulator += n.value;
        });
    }
    accumulator
}

fn find_adjacencies(numbers: Vec<Number>, symbols: Vec<Symbol>) -> HashMap<Symbol, Vec<Number>> {
    let mut adjacencies: HashMap<Symbol, Vec<Number>> = HashMap::new();

    for symbol in symbols.iter() {
        for number in numbers.iter() {
            if (symbol.line - 1 == number.line || symbol.line + 1 == number.line)
                && symbol.index + 1 >= number.range.start
                && symbol.index <= number.range.end
            {
                adjacent_add(&mut adjacencies, symbol, number);
            }

            if symbol.line == number.line
                && (symbol.index + 1 == number.range.start || symbol.index == number.range.end)
            {
                adjacent_add(&mut adjacencies, symbol, number);
            }
        }
    }

    adjacencies
}

fn adjacent_add(adjacencies: &mut HashMap<Symbol, Vec<Number>>, symbol: &Symbol, number: &Number) {
    if !adjacencies.contains_key(symbol) {
        let b: Vec<Number> = vec![number.clone()];
        adjacencies.insert(symbol.clone(), b);
    } else {
        adjacencies.entry(symbol.clone()).and_modify(|vec| {
            if !vec.contains(number) {
                vec.push(number.clone())
            }
        });
    }
}
