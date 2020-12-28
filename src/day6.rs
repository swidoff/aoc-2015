use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day6.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

type Coord = (u64, u64);

enum Action {
    On,
    Off,
    Toggle,
}

struct Instruction {
    action: Action,
    upper_left: Coord,
    lower_right: Coord,
}

fn parse_coord(line: &str) -> Coord {
    let (x_str, y_str) = line.split(",").collect_tuple().unwrap();
    (u64::from_str(x_str).unwrap(), u64::from_str(y_str).unwrap())
}

fn parse_instruction(line: &str) -> Instruction {
    let (action, prefix_len) = if line.starts_with("turn on") {
        (Action::On, "turn on".len())
    } else if line.starts_with("turn off") {
        (Action::Off, "turn off".len())
    } else {
        (Action::Toggle, "toggle".len())
    };

    let (ul_str, _, lr_str) = line[prefix_len + 1..]
        .split_whitespace()
        .collect_tuple()
        .unwrap();
    Instruction {
        action,
        upper_left: parse_coord(ul_str),
        lower_right: parse_coord(lr_str),
    }
}

struct Lights {
    map: HashMap<Coord, i64>,
}

impl Lights {
    fn new() -> Lights {
        Lights {
            map: HashMap::new(),
        }
    }

    fn switch_part1(&mut self, instruction: Instruction) {
        for x in instruction.upper_left.0..instruction.lower_right.0 + 1 {
            for y in instruction.upper_left.1..instruction.lower_right.1 + 1 {
                match instruction.action {
                    Action::On => self.map.insert((x, y), 1),
                    Action::Off => self.map.remove(&(x, y)),
                    Action::Toggle if self.map.get(&(x, y)).is_some() => self.map.remove(&(x, y)),
                    _ => self.map.insert((x, y), 1),
                };
            }
        }
    }

    fn switch_part2(&mut self, instruction: Instruction) {
        for x in instruction.upper_left.0..instruction.lower_right.0 + 1 {
            for y in instruction.upper_left.1..instruction.lower_right.1 + 1 {
                let brightness = match self.map.get(&(x, y)) {
                    Some(x) => *x,
                    None => 0,
                };

                let change = match instruction.action {
                    Action::On => 1,
                    Action::Off => -1,
                    Action::Toggle => 2,
                };

                self.map.insert((x, y), (brightness + change).max(0));
            }
        }
    }

    fn count(&self) -> usize {
        self.map.len()
    }

    fn brightness(&self) -> i64 {
        self.map.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::{parse_instruction, read_file, Lights};

    #[test]
    fn part1() {
        let instructions = read_file().map(|line| parse_instruction(line.as_str()));
        let mut lights = Lights::new();
        for i in instructions {
            lights.switch_part1(i);
        }

        println!("{}", lights.count());
    }

    #[test]
    fn part2() {
        let instructions = read_file().map(|line| parse_instruction(line.as_str()));
        let mut lights = Lights::new();
        for i in instructions {
            lights.switch_part2(i);
        }

        println!("{}", lights.brightness());
    }
}
