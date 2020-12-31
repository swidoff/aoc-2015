use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day19.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_inputs(iter: impl Iterator<Item = String>) -> (Vec<(String, String)>, String) {
    let mut rules = Vec::new();
    let mut molecule = None;
    let mut parsing_rules = true;

    for line in iter {
        if parsing_rules {
            if line == "" {
                parsing_rules = false;
            } else {
                let (lhs, rhs) = line.split(" => ").collect_tuple().unwrap();
                rules.push((lhs.to_string(), rhs.to_string()));
            }
        } else {
            molecule.replace(line);
        }
    }

    (rules, molecule.unwrap())
}

fn part1(rules: &Vec<(String, String)>, input: &String) -> usize {
    rules
        .iter()
        .flat_map(|(lhs, rhs)| {
            input.match_indices(lhs).map(move |(i, _)| {
                let mut res = String::from(input);
                for _ in 0..lhs.len() {
                    res.remove(i);
                }
                res.insert_str(i, rhs);
                res
            })
        })
        .unique()
        .count()
}

fn part2(rules: &Vec<(String, String)>, input: String) -> Option<usize> {
    part2_recurse(rules, input, &mut HashMap::new())
}

fn part2_recurse(
    rules: &Vec<(String, String)>,
    input: String,
    seen: &mut HashMap<String, Option<usize>>,
) -> Option<usize> {
    if input == "e" {
        Some(0)
    } else if let Some(res) = seen.get(&input) {
        *res
    } else {
        let res = rules
            .iter()
            .filter_map(|(lhs, rhs)| {
                let first_match_index = input.match_indices(rhs).take(1).map(|(i, _)| i).next();
                if let Some(i) = first_match_index {
                    let res = input.replacen(rhs, lhs, 1);
                    Some((res, i))
                } else {
                    None
                }
            })
            .sorted_by_key(|(_, score)| *score)
            .find_map(|(s, _)| part2_recurse(rules, s, seen).map(|v| v + 1));
        seen.insert(input.to_string(), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::{parse_inputs, part1, part2, read_file};

    const EXAMPLE1: &str = "H => HO
H => OH
O => HH

HOH
";

    const EXAMPLE2: &str = "H => HO
H => OH
O => HH

HOHOHO
";

    const EXAMPLE3: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOH
";

    const EXAMPLE4: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
";

    #[test]
    fn test_part1_example() {
        let (rules, molecule) = parse_inputs(EXAMPLE1.lines().map(|s| s.to_string()));
        let res = part1(&rules, &molecule);
        assert_eq!(4, res);

        let (rules, molecule) = parse_inputs(EXAMPLE2.lines().map(|s| s.to_string()));
        let res = part1(&rules, &molecule);
        assert_eq!(7, res);
    }

    #[test]
    fn test_part1() {
        let (rules, molecule) = parse_inputs(read_file());
        let res = part1(&rules, &molecule);
        println!("{}", res);
    }

    #[test]
    fn test_part2_example() {
        let (mut rules, molecule) = parse_inputs(EXAMPLE3.lines().map(|s| s.to_string()));
        let res = part2(&rules, molecule);
        assert_eq!(Some(3), res);

        let (mut rules, molecule) = parse_inputs(EXAMPLE4.lines().map(|s| s.to_string()));
        let res = part2(&rules, molecule);
        assert_eq!(Some(6), res);
    }

    #[test]
    fn test_part2() {
        let (mut rules, molecule) = parse_inputs(read_file());
        let res = part2(&rules, molecule);
        println!("{}", res.unwrap());
        // assert_eq!(Some(3), res);
    }
}
