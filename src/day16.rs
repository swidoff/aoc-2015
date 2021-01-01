use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day16.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

// Sue 1: goldfish: 9, cars: 0, samoyeds: 9
fn parse_input(iter: impl Iterator<Item = String>) -> Vec<HashMap<String, u32>> {
    iter.map(|line| {
        HashMap::from_iter(line.split_whitespace().dropping(2).tuples().map(|(k, v)| {
            (
                k.replace(":", ""),
                u32::from_str(&v.replace(",", "")).unwrap(),
            )
        }))
    })
    .collect_vec()
}

fn part1(input: &Vec<HashMap<String, u32>>, clues: &HashMap<&str, u32>) -> usize {
    input
        .iter()
        .enumerate()
        .find(|(_, map)| {
            map.iter()
                .all(|(k, v)| *clues.get(k.as_str()).unwrap() == *v)
        })
        .unwrap()
        .0
        + 1
}

fn part2(input: &Vec<HashMap<String, u32>>, clues: &HashMap<&str, (u32, u32)>) -> usize {
    input
        .iter()
        .enumerate()
        .find(|(_, map)| {
            map.iter().all(|(k, v)| {
                let (min, max) = *clues.get(k.as_str()).unwrap();
                *v >= min && *v <= max
            })
        })
        .unwrap()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    use crate::day16::{parse_input, part1, part2, read_file};
    use std::collections::HashMap;

    #[test]
    fn test_part1() {
        let mut clues = HashMap::new();
        clues.insert("children", 3);
        clues.insert("cats", 7);
        clues.insert("samoyeds", 2);
        clues.insert("pomeranians", 3);
        clues.insert("akitas", 0);
        clues.insert("vizslas", 0);
        clues.insert("goldfish", 5);
        clues.insert("trees", 3);
        clues.insert("cars", 2);
        clues.insert("perfumes", 1);

        let input = parse_input(read_file());
        let res = part1(&input, &clues);
        println!("{}", res);
    }

    #[test]
    fn test_part2() {
        let mut clues = HashMap::new();
        clues.insert("children", (3, 3));
        clues.insert("cats", (8, u32::max_value()));
        clues.insert("samoyeds", (2, 2));
        clues.insert("pomeranians", (0, 2));
        clues.insert("akitas", (0, 0));
        clues.insert("vizslas", (0, 0));
        clues.insert("goldfish", (0, 4));
        clues.insert("trees", (4, u32::max_value()));
        clues.insert("cars", (2, 2));
        clues.insert("perfumes", (1, 1));

        let input = parse_input(read_file());
        let res = part2(&input, &clues);
        println!("{}", res);
    }
}
