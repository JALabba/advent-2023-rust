use std::{ collections::{ HashMap, VecDeque }, fmt::{ Debug, Error }, str::FromStr };

use advent_of_code::lcm;

advent_of_code::solution!(20);

const BROADCASTER: ID = ID(0x00);

pub fn part_one(_input: &str) -> Option<u64> {
    let (modules, network) = parse_input(_input);
    let mut state = Stepper::new(&modules, &network);
    for _ in 0..1000 {
        let button_signal = Signal { from: BROADCASTER, to: BROADCASTER, strength: Pulse::Low };
        state.start(button_signal);
    }
    Some((state.highs * state.lows) as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (modules, network) = parse_input(_input);
    // find the parent module of rx
    let rx = ID::from_str("rx").unwrap();
    let parent = *network
        .iter()
        .find(|(&_, outputs)| outputs.contains(&rx))
        .expect("parent to rx").0;
    let mut state = Stepper::new(&modules, &network);
    // the goal is for rx to recieve a LOW pulse, so it's parent must recieve all HIGH pulses
    // parent is a conjunction module, so it will remember it's parent signals
    // keep track of "cycles" of the grandparents, as when they all sync to HIGH, that is the answer
    let mut cycles: HashMap<ID, usize> = HashMap::new();
    let target_len = match &state.modules.get(&parent).unwrap().class {
        Class::Conjunction { memory } => { memory.len() }
        _ => unreachable!(),
    };

    for i in 0..10_000 {
        let button_signal = Signal { from: BROADCASTER, to: BROADCASTER, strength: Pulse::Low };
        let investigated: Vec<ID> = state.investigate(button_signal, &parent);

        if investigated.len() > 0 {

            for id in investigated {
                cycles.insert(id, i+1);
            }
            if cycles.len() >= target_len {
                break;
            }
        }
    }

    Some(cycles.values().fold(1, |acc: usize, count| lcm(acc, *count)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32_000_000));
    }

    #[test]
    fn test_part_one_once() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (modules, network) = parse_input(input);
        let mut state = Stepper::new(&modules, &network);
        let button_signal = Signal { from: BROADCASTER, to: BROADCASTER, strength: Pulse::Low };
        state.start(button_signal);
        assert_eq!(state.highs, 4);
        assert_eq!(state.lows, 8);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let input = "broadcaster -> a, b
%a -> c, f
%f -> g
&g -> con
%c -> a, con
%b -> d
%d -> b, con
&con -> rx";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct ID(u16);

impl FromStr for ID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.starts_with("broadcast") {
            Ok(BROADCASTER)
        } else {
            let low_byte = s.as_bytes()[0] as u16;
            let mut id: u16 = low_byte;
            if s.trim().len() > 1 {
                let high_byte = (s.as_bytes()[1] as u16) << 8;
                id = id | high_byte;
            }
            if id == BROADCASTER.0 {
                Err(Error)
            } else {
                Ok(ID(id))
            }
        };
        res
    }
}

impl ID {
    fn to_string(&self) -> String {
        if *self == BROADCASTER {
            return "broadcaster".to_string();
        }
        let high_byte = ((self.0 >> 8) & 0xff) as u8;
        let low_byte = (self.0 & 0xff) as u8;
        let mut result = String::new();
        if low_byte != 0 {
            result.push(low_byte as char);
        }
        if high_byte != 0 {
            result.push(high_byte as char);
        }
        result
    }
}

impl Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self.to_string();
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}
impl Pulse {
    fn flip(&self) -> Pulse {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }

    fn as_bool(&self) -> bool {
        match self {
            Pulse::High => true,
            Pulse::Low => false,
        }
    }
}

#[derive(Debug)]
struct Signal {
    from: ID,
    to: ID,
    strength: Pulse,
}

#[derive(Debug, Clone)]
enum Class {
    FlipFlop {
        state: Pulse,
    },
    Conjunction {
        memory: HashMap<ID, Pulse>,
    },
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    id: ID,
    class: Class,
}
impl Module {
    fn handle_signal(&mut self, signal: Signal, network: &HashMap<ID, Vec<ID>>) -> Vec<Signal> {
        match &mut self.class {
            Class::Broadcaster => {
                network
                    .get(&signal.to)
                    .unwrap()
                    .iter()
                    .map(|x| { Signal { from: signal.to, to: *x, strength: signal.strength } })
                    .collect::<Vec<Signal>>()
            }
            Class::FlipFlop { state } => {
                match signal.strength {
                    Pulse::High => vec![],
                    Pulse::Low => {
                        *state = state.flip();
                        network
                            .get(&signal.to)
                            .unwrap()
                            .iter()
                            .map(|x| { Signal { from: signal.to, to: *x, strength: *state } })
                            .collect::<Vec<Signal>>()
                    }
                }
            }
            Class::Conjunction { memory } => {
                memory.insert(signal.from, signal.strength);
                let pulse = match memory.values().all(|pulse| pulse.as_bool()) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                network
                    .get(&signal.to)
                    .unwrap()
                    .iter()
                    .map(|x| { Signal { from: signal.to, to: *x, strength: pulse } })
                    .collect::<Vec<Signal>>()
            }
        }
    }
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'b' => Ok(Module { id: BROADCASTER, class: Class::Broadcaster }),
            '%' => {
                let s1 = s.chars().skip(1).collect::<String>();
                Ok(Module {
                    id: ID::from_str(&s1).unwrap(),
                    class: Class::FlipFlop { state: Pulse::Low },
                })
            }
            '&' => {
                let s1 = s.chars().skip(1).collect::<String>();
                Ok(Module {
                    id: ID::from_str(&s1).unwrap(),
                    class: Class::Conjunction { memory: HashMap::new() },
                })
            }
            _ => panic!("Can't make a module from a "),
        }
    }
}

fn parse_line(s: &str) -> (Module, Vec<ID>) {
    let (left, right) = s.split_once("->").unwrap();
    (
        Module::from_str(left).unwrap(),
        right
            .split(",")
            .map(|r| ID::from_str(r.trim()).unwrap())
            .collect::<Vec<ID>>(),
    )
}

fn parse_input(_input: &str) -> (HashMap<ID, Module>, HashMap<ID, Vec<ID>>) {
    let modules = _input.lines().map(parse_line).collect::<Vec<_>>();
    let mut network: HashMap<ID, Vec<ID>> = HashMap::new();
    for (module, outputs) in &modules {
        network.insert(module.id, outputs.clone());
    }
    let mut modules_map: HashMap<ID, Module> = HashMap::from_iter(
        modules
            .iter()
            .cloned()
            .map(|x| (x.0.id, x.0))
    );

    //init the memory maps on Conjunction modules by remembering reciever nodes
    let mut parent_modules: HashMap<ID, Vec<ID>> = HashMap::new();
    for (id, outputs) in &network {
        for child in outputs {
            parent_modules
                .entry(*child)
                .and_modify(|v| v.push(*id))
                .or_insert(vec![*id]);
        }
    }
    for (id, inputs) in &parent_modules {
        for parent in inputs {
            modules_map.entry(*id).and_modify(|module| {
                match &mut module.class {
                    Class::Conjunction { memory } => {
                        memory.insert(*parent, Pulse::Low);
                    }
                    _ => (),
                }
            });
        }
    }

    (modules_map, network)
}

struct Stepper<'a> {
    modules: HashMap<ID, Module>,
    network: &'a HashMap<ID, Vec<ID>>,
    queue: VecDeque<Signal>,
    lows: usize,
    highs: usize,
}

impl<'a> Stepper<'a> {
    fn new(modules: &HashMap<ID, Module>, network: &'a HashMap<ID, Vec<ID>>) -> Self {
        Stepper { modules: modules.clone(), network, queue: VecDeque::new(), lows: 0, highs: 0 }
    }

    fn start(&mut self, signal: Signal) {
        self.queue.push_back(signal);
        while let Some(signal) = self.queue.pop_front() {
            match signal.strength {
                Pulse::High => {
                    self.highs += 1;
                }
                Pulse::Low => {
                    self.lows += 1;
                }
            }
            if let Some(reciever) = self.modules.get_mut(&signal.to) {
                let response = reciever.handle_signal(signal, self.network);
                for new_signal in response {
                    self.queue.push_back(new_signal);
                }
            }
        }
    }

    fn investigate(&mut self, signal: Signal, parent: &ID) -> Vec<ID> {
        self.queue.push_back(signal);
        let mut result = Vec::new();

        while let Some(signal) = self.queue.pop_front() {
            match signal.strength {
                Pulse::High => {
                    self.highs += 1;
                }
                Pulse::Low => {
                    self.lows += 1;
                }
            }

            if let Some(reciever) = self.modules.get_mut(&signal.to) {
                let response = reciever.handle_signal(signal, self.network);
                for new_signal in response {
                    if new_signal.to == *parent && new_signal.strength == Pulse::High {
                        result.push(new_signal.from);
                    }
                    self.queue.push_back(new_signal);
                }
            }
        }
        result
    }
}
