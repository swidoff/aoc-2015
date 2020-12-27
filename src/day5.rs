use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::iter::FromIterator;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day5.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn is_nice_part1(line: &str) -> bool {
    let has_three_vowels = line
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
        >= 3;

    let has_double_letters = line.chars().tuple_windows().any(|(c1, c2)| c1 == c2);

    let has_no_forbidden = line.chars().tuple_windows().all(|(c1, c2)| match (c1, c2) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => false,
        _ => true,
    });

    has_three_vowels && has_double_letters && has_no_forbidden
}

fn is_nice_part2(line: &str) -> bool {
    let rule1 = line
        .chars()
        .tuple_windows()
        .enumerate()
        .scan(HashMap::new(), |map, (i, (c1, c2))| {
            match map.get(&(c1, c2)) {
                Some(j) if i > j + 1 => Some(true),
                Some(_) => Some(false),
                _ => {
                    map.insert((c1, c2), i);
                    Some(false)
                }
            }
        })
        .any(|v| v);

    let rule2 = line.chars().tuple_windows().any(|(c1, _, c3)| c1 == c3);

    rule1 && rule2
}

#[cfg(test)]
mod tests {
    use crate::day5::{is_nice_part1, is_nice_part2, read_file};

    #[test]
    fn part1_examples() {
        assert_eq!(true, is_nice_part1("ugknbfddgicrmopn"));
        assert_eq!(true, is_nice_part1("aaa"));
        assert_eq!(false, is_nice_part1("jchzalrnumimnmhp"));
        assert_eq!(false, is_nice_part1("haegwjzuvuyypxyu"));
        assert_eq!(false, is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part1() {
        let res = read_file()
            .filter(|line| is_nice_part1(line.as_str()))
            .count();
        println!("{}", res);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(true, is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, is_nice_part2("xxyxx"));
        assert_eq!(false, is_nice_part2("aaaba"));
        assert_eq!(false, is_nice_part2("abaaa"));
        assert_eq!(true, is_nice_part2("abaaaa"));
        assert_eq!(false, is_nice_part2("uurcxstgmygtbstg"));
        assert_eq!(false, is_nice_part2("ieodomkazucvgmuy"));
    }

    #[test]
    fn part2() {
        let res = read_file()
            .filter(|line| is_nice_part2(line.as_str()))
            .count();
        println!("{}", res);
    }
}
