use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day13.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_input(iter: impl Iterator<Item = String>) -> HashMap<(char, char), i64> {
    HashMap::from_iter(iter.map(|line| {
        let tokens = line.split_whitespace().collect_vec();
        let source = tokens[0].chars().next().unwrap();
        let dest = tokens[tokens.len() - 1].chars().next().unwrap();
        let sign = if tokens[2] == "gain" { 1 } else { -1 };
        let amount = i64::from_str(tokens[3]).unwrap();
        ((source, dest), sign * amount)
    }))
}

fn find_optimal_happiness(matrix: &HashMap<(char, char), i64>) -> i64 {
    let people = matrix.keys().map(|(p1, _)| *p1).unique().collect_vec();
    let num_people = people.len();
    people
        .into_iter()
        .permutations(num_people)
        .map(|order| total_happiness(&order, matrix))
        .max()
        .unwrap()
}

fn total_happiness(order: &Vec<char>, matrix: &HashMap<(char, char), i64>) -> i64 {
    order
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let prev_index = if i == 0 { order.len() - 1 } else { i - 1 };
            let next_index = (i + 1) % order.len();
            let prev_happiness = *matrix.get(&(*p, order[prev_index])).unwrap();
            let next_happiness = *matrix.get(&(*p, order[next_index])).unwrap();
            prev_happiness + next_happiness
        })
        .sum()
}
#[cfg(test)]

mod tests {
    use crate::day13::{find_optimal_happiness, parse_input, read_file};
    use itertools::Itertools;

    #[test]
    fn test_part1_example() {
        let example = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

        let matrix = parse_input(example.lines().map(|v| v.to_string()));
        assert_eq!(330, find_optimal_happiness(&matrix));
    }

    #[test]
    fn test_part1() {
        let matrix = parse_input(read_file());
        println!("{}", find_optimal_happiness(&matrix));
    }

    #[test]
    fn test_part2() {
        let mut matrix = parse_input(read_file());
        let people = matrix.keys().map(|(p1, _)| *p1).unique().collect_vec();
        for p in people {
            matrix.insert(('X', p), 0);
            matrix.insert((p, 'X'), 0);
        }

        println!("{}", find_optimal_happiness(&matrix));
    }
}
