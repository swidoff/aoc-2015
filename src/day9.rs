use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day9.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn trip_distances(lines: impl Iterator<Item = String>) -> Vec<u64> {
    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in lines {
        let tokens = line.split_whitespace().collect_vec();
        let src = tokens[0];
        let dest = tokens[2];
        let dist = u64::from_str(tokens[4]).unwrap();
        locations.insert(src.to_string());
        locations.insert(dest.to_string());
        distances.insert((src.to_string(), dest.to_string()), dist);
        distances.insert((dest.to_string(), src.to_string()), dist);
    }

    locations
        .iter()
        .permutations(locations.len())
        .map(|locs| {
            locs.iter()
                .map(|s| *s)
                .tuple_windows()
                .map(|(s1, s2)| distances.get(&(s1.to_string(), s2.to_string())).unwrap())
                .sum()
        })
        .collect_vec()
}

fn part1(lines: impl Iterator<Item = String>) -> u64 {
    *trip_distances(lines).iter().min().unwrap()
}

fn part2(lines: impl Iterator<Item = String>) -> u64 {
    *trip_distances(lines).iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day9::{part1, part2, read_file};

    #[test]
    fn test_part1_example() {
        let example = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let res = part1(example.lines().map(|v| v.to_string()));
        assert_eq!(res, 605);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
    }

    #[test]
    fn test_part2_example() {
        let example = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let res = part2(example.lines().map(|v| v.to_string()));
        assert_eq!(res, 982);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
    }
}
