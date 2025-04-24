use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let sequence = Sequence::parse(input);
    let result = sequence.sum();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sequence = Sequence::parse(input);
    let result:usize = sequence.arrangement_procedure_focusing_power();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}

/*
BOXES
There are 0..256 boxes

Each box holds lenses
a lens is 1..=9 focal length

*/

struct Sequence {
    strings: Vec<String>,
    boxes: HashMap<u8, Vec<String>>
}

impl Sequence {
    fn parse(input: &str) -> Sequence {
        /*
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        example input:
         ignore newline when parsing
         comma separated list
        */
        let input = input.replace('\n', "");
        let strings: Vec<String> = input.split(',').map(|line|line.to_owned()).collect();
        let mut boxes: HashMap<u8, Vec<String>> = HashMap::new();
        for i in 0..=255 as u8 {
            boxes.insert(i, vec![]);
        }
        Sequence { strings, boxes }
    }

    fn sum(&self) -> usize {
        let mut sum = 0;

        for string in &self.strings {
            sum += hash_string(string) as usize;
        }
        sum
    }

    fn arrangement_procedure_focusing_power(&mut self) -> usize {


        for i in 0..self.strings.len() {
            let instruction = &self.strings[i];
            let (label, operation, focal_length) = self.parse_instruction(instruction);
            let box_nr = hash_string(&label);
            match operation {
                '-' => self.remove_box(box_nr, label),
                '=' => self.add_to_box(box_nr, label, focal_length),
                _ => panic!("unrecognized last char"),
            }
        }

        self.focusing_power()
    }


    fn parse_instruction(&self, instruction: &str) -> (String, char, u8) {
        if let Some(index) = instruction.find('=') {
            // Found '=', parse three parts
            let label = instruction[..index].to_string();
            let operation = '=';
            let focal_length: u8 = instruction[index + 1..].parse().unwrap_or(0);

            (label, operation, focal_length)
        } else if let Some(index) = instruction.find('-') {
            // Found '-', parse two parts with default focal length 0
            let label = instruction[..index].to_string();
            let operation = '-';
            let focal_length: u8 = 0;

            (label, operation, focal_length)
        } else {
            // No '=' or '-' found, handle as needed (default values, panic, etc.)
            panic!("Invalid instruction format: {}", instruction);
        }
    }

    fn remove_box(&mut self, box_nr: u8, label: String) {
        /*
        go to relevant box, remove the lens with the given label if present
        then, move any remaining lenses as far forward as they can go
        without changing order.
        if no label, nothing happens.
         */
        if let Some( box_vec) = self.boxes.get_mut(&box_nr) {
            if let Some(index) = box_vec.iter().position( |str| str.starts_with(&label)){

                box_vec.remove(index);
                box_vec.retain(|item| !item.is_empty());
            }
        }

    }

    fn add_to_box(&mut self, box_nr: u8, label: String, focal_length: u8) {
        /*
        create a new string with label and focal lens, e.g., [rn 1].

        if there is a lens in the box with the same label, replace old lens.
        else, add the lens to the end.
         */
        let lens = format!("{} {}", label, focal_length);

        if let Some(box_vec) = self.boxes.get_mut(&box_nr) {
            if let Some(index) = box_vec.iter().position( |item| item.starts_with(&label)){
                //replace old with new one
                box_vec[index] = lens;
            } else {
                //add new to the end
                box_vec.push(lens);
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut focusing_power = 0;
        for box_nr in 0..=255 {
            if let Some(this_box) = self.boxes.get(&box_nr){
                for (index, lens) in this_box.iter().enumerate() {
                    let focal_length:u8 = extract_focal_length(lens);
                    let res = (1+box_nr as usize) * (1+index) * (focal_length as usize);
                    focusing_power += res;
                }
            }
        }
        focusing_power
    }
}

fn extract_focal_length(lens: &str) -> u8 {
    // take the end of the string that can be parsed as a number between 1..=9
    lens.chars().last().and_then(|c| c.to_digit(10).map(|digit| digit as u8)).unwrap()
}

fn hash_string(str: &str) -> u8 {
    /*
    HASH Algorithm
    start with current value 0
    for each char startin from 0
    determine ASCII code
    value + ascii code
    value *= 17
    value to the remainder of dividing itself by 256, value = value % 256
    output value
    */
    let mut value = 0;
    for c in str.chars() {
        let ascii_code = c as usize;
        value += ascii_code;
        value *= 17;
        value %= 256;
    }
    value as u8
}


