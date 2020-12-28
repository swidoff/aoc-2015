use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day7.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

enum Operand {
    Number(u16),
    Ref(String),
}

enum Operator {
    Assign,
    Not,
    And,
    Or,
    LShift,
    RShift,
}

struct Instruction {
    operator: Operator,
    operands: Vec<Operand>,
}

impl Instruction {
    fn new(operator: Operator, operands: Vec<Operand>) -> Instruction {
        Instruction { operator, operands }
    }
}

fn parse_operand(s: &str) -> Operand {
    u16::from_str(s)
        .map(|v| Operand::Number(v))
        .unwrap_or_else(|_| Operand::Ref(s.to_string()))
}

fn parse_instructions(lines: impl Iterator<Item = String>) -> HashMap<String, Instruction> {
    HashMap::from_iter(lines.map(|line| {
        let (lhs, rhs) = line.split(" -> ").collect_tuple().unwrap();
        let tokens = lhs.split_whitespace().collect_vec();
        let instruction = match tokens.len() {
            1 => Instruction::new(Operator::Assign, vec![parse_operand(tokens[0])]),
            2 => Instruction::new(Operator::Not, vec![parse_operand(tokens[1])]),
            _ => {
                let operator = match tokens[1] {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "LSHIFT" => Operator::LShift,
                    "RSHIFT" => Operator::RShift,
                    _ => panic!("Unknown operator: {}", tokens[1]),
                };
                let operands = vec![parse_operand(tokens[0]), parse_operand(tokens[2])];
                Instruction::new(operator, operands)
            }
        };
        (rhs.to_string(), instruction)
    }))
}

fn process_instructions(instructions: &HashMap<String, Instruction>) -> HashMap<&str, u16> {
    let mut deps = DiGraph::<&String, ()>::new();
    let mut indexes = HashMap::new();
    for rhs in instructions.keys() {
        indexes.insert(rhs, deps.add_node(rhs));
    }

    for (rhs, instruction) in instructions.iter() {
        let rhs_index = indexes.get(rhs).unwrap();
        for lhs in instruction.operands.iter() {
            if let Operand::Ref(r) = lhs {
                let lhs_index = indexes.get(r).unwrap();
                deps.add_edge(*lhs_index, *rhs_index, ());
            }
        }
    }

    let order = toposort(&deps, None).unwrap();
    let mut res = HashMap::new();
    for node_index in order.iter() {
        let rhs = *deps.node_weight(*node_index).unwrap();
        let instruction = instructions.get(rhs).unwrap();
        let operands = instruction
            .operands
            .iter()
            .map(|op| match op {
                Operand::Number(v) => *v,
                Operand::Ref(r) => *res.get(r.as_str()).unwrap(),
            })
            .collect_vec();

        let value = match instruction.operator {
            Operator::Assign => operands[0],
            Operator::Not => !operands[0],
            Operator::And => operands[0] & operands[1],
            Operator::Or => operands[0] | operands[1],
            Operator::LShift => operands[0] << operands[1],
            Operator::RShift => operands[0] >> operands[1],
        };

        res.insert(rhs.as_str(), value);
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::day7::{
        parse_instructions, process_instructions, read_file, Instruction, Operand, Operator,
    };

    #[test]
    fn part1_example() {
        let example = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

        let instructions = parse_instructions(example.lines().map(|s| s.to_string()));
        let res = process_instructions(&instructions);
        assert_eq!(*res.get("d").unwrap(), 72);
        assert_eq!(*res.get("e").unwrap(), 507);
        assert_eq!(*res.get("f").unwrap(), 492);
        assert_eq!(*res.get("g").unwrap(), 114);
        assert_eq!(*res.get("h").unwrap(), 65412);
        assert_eq!(*res.get("i").unwrap(), 65079);
        assert_eq!(*res.get("x").unwrap(), 123);
        assert_eq!(*res.get("y").unwrap(), 456);
    }

    #[test]
    fn part1() {
        let instructions = parse_instructions(read_file());
        let res = process_instructions(&instructions);
        let res = *res.get("a").unwrap();
        println!("{}", res);
        assert_eq!(956, res)
    }

    #[test]
    fn part2() {
        let mut instructions = parse_instructions(read_file());
        instructions.insert(
            "b".to_string(),
            Instruction::new(Operator::Assign, vec![Operand::Number(956)]),
        );

        let res = process_instructions(&instructions);
        let res = *res.get("a").unwrap();
        println!("{}", res);
        assert_eq!(40149, res)
    }
}
