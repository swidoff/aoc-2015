use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day23.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

//noinspection RsEnumVariantNaming
enum Instruction {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(i64),
    jie(usize, i64),
    jio(usize, i64),
}

fn register(line: &String) -> usize {
    if line[4..].chars().next().unwrap() == 'a' {
        0
    } else {
        1
    }
}

fn parse_instructions(iter: impl Iterator<Item = String>) -> Vec<Instruction> {
    iter.map(|line| match &line[0..3] {
        "hlf" => Instruction::hlf(register(&line)),
        "tpl" => Instruction::tpl(register(&line)),
        "inc" => Instruction::inc(register(&line)),
        "jmp" => Instruction::jmp(i64::from_str(&line[4..]).unwrap()),
        "jie" => Instruction::jie(register(&line), i64::from_str(&line[7..]).unwrap()),
        "jio" => Instruction::jio(register(&line), i64::from_str(&line[7..]).unwrap()),
        _ => panic!("Unknown instruction: {}", line),
    })
    .collect_vec()
}

fn execute(instructions: &Vec<Instruction>, a: u64, b: u64) -> u64 {
    let mut registers = [a, b];
    let mut pc: i64 = 0;

    while (pc as usize) < instructions.len() {
        match instructions[pc as usize] {
            Instruction::hlf(r) => {
                registers[r] /= 2;
                pc += 1;
            }
            Instruction::tpl(r) => {
                registers[r] *= 3;
                pc += 1;
            }
            Instruction::inc(r) => {
                registers[r] += 1;
                pc += 1;
            }
            Instruction::jmp(offset) => {
                pc += offset;
            }
            Instruction::jie(r, offset) => {
                pc += if registers[r] % 2 == 0 { offset } else { 1 };
            }
            Instruction::jio(r, offset) => {
                pc += if registers[r] == 1 { offset } else { 1 };
            }
        }
    }

    registers[1]
}

#[cfg(test)]
mod tests {
    use crate::day23::{execute, parse_instructions, read_file};

    #[test]
    fn test_part1_example() {
        let example = "inc b
jio b, +2
tpl b
inc b";
        let instructions = parse_instructions(example.lines().map(|line| line.to_string()));
        let res = execute(&instructions, 0, 0);
        assert_eq!(2, res);
    }

    #[test]
    fn test_part1() {
        let instructions = parse_instructions(read_file());
        let res = execute(&instructions, 0, 0);
        println!("{}", res);
        // assert_eq!(2, res);
    }

    #[test]
    fn test_part2() {
        let instructions = parse_instructions(read_file());
        let res = execute(&instructions, 1, 0);
        println!("{}", res);
        // assert_eq!(2, res);
    }
}
