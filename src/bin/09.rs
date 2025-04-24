advent_of_code::solution!(9);

pub fn part_one(_input: &str) -> Option<u64> {
    let mut oasis = Oasis::parse(_input);
    let sum = oasis.report(Part::One);
    Some(sum as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut oasis = Oasis::parse(_input);
    let sum = oasis.report(Part::Two);
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

#[derive(Debug, Clone, Copy)]
enum Part {
    One,
    Two,
}


#[derive(Debug, Clone)]
struct History {
    value: Vec<i64>,
    prediction: i64,
}

impl History {
    fn new(value: Vec<i64>) -> History {
        History {
            value,
            prediction: 0,
        }
    }
    fn evaluate(&mut self, part: Part) -> i64 {
        let mut stack = HistoryStack::new();
        self.prediction = stack.evaluate(self, part);
        self.prediction
    }
}

#[derive(Debug)]
struct HistoryStack {
    stack: Vec<History>,
}

impl HistoryStack {
    fn new() -> HistoryStack {
        HistoryStack { stack: vec![] }
    }
    fn evaluate(&mut self, history: &mut History, part: Part) -> i64 {
        self.stack.push(history.clone());
        self.build_stack();
        self.predict(part);
        // for history in &self.stack {
        //     println!("{:?} {}", history.value, history.prediction);
        // }
        let out = self.stack[0].prediction;
        self.stack.clear(); // Clear the stack for the next evaluation
        out
    }

    fn build_stack(&mut self) {
        while let Some(last_history) = self.stack.last() {
            if last_history.value.iter().all(|&v| v == 0) {
                break;
            } else {
                let vec = HistoryStack::differences(last_history.value.clone());
                self.stack.push(History {
                    value: vec,
                    prediction: 0,
                });
            }
        }
    }
    fn predict(&mut self, part: Part) {
        let len = &self.stack.len();
        for index in (0..*len).rev() {
            let h = &self.stack[index];
            if index != 0 {
                match part {
                    Part::One =>
                    self.stack[index - 1].prediction =
                        h.prediction + self.stack[index - 1].value.last().unwrap(),
                    Part::Two =>
                    self.stack[index - 1].prediction =
                        self.stack[index - 1].value.first().unwrap() - h.prediction,
                }
            }
        }
    }

    fn differences(vec: Vec<i64>) -> Vec<i64> {
        // Compute differences of each pair in the sequence
        let mut result = Vec::new();
        for i in 0..vec.len() - 1 {
            result.push(vec[i + 1] - vec[i]);
        }
        result
    }
}

#[derive(Debug)]
struct Oasis {
    histories: Vec<History>,
}

impl Oasis {
    fn parse(input: &str) -> Oasis {
        let lines = input.lines().peekable();

        let mut histories: Vec<History> = Vec::new();

        for line in lines {
            let value: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("number parse"))
                .collect();
            histories.push(History::new(value));
        }
        Oasis { histories }
    }

    fn report(&mut self, part: Part) -> i64 {
        let sum: i64 = self
            .histories
            .iter_mut()
            .map(|history| {
                let p = history.evaluate(part);
                p
            })
            .sum();
        sum
    }
}

