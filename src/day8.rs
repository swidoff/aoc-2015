use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day8.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn count_memory_characters(line: &String) -> usize {
    let mut iter = line[1..line.len() - 1].chars();
    let mut count = 0;
    while let Some(c) = iter.next() {
        if c == '\\' {
            match iter.next() {
                Some('x') => {
                    iter.next().unwrap();
                    iter.next().unwrap();
                }
                _ => {}
            }
        }
        count += 1;
    }
    count
}

fn part1(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|line| line.len() - count_memory_characters(&line))
        .sum()
}

fn count_encoded_characters(line: &String) -> usize {
    let mut iter = line.chars();
    let mut count = 2;
    while let Some(c) = iter.next() {
        let chars = match c {
            '\\' | '"' => 2,
            _ => 1,
        };
        count += chars;
    }
    count
}

fn part2(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(|line| count_encoded_characters(&line) - line.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2, read_file};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    fn read_examples_file() -> impl Iterator<Item = String> {
        let file = File::open("input/day8_examples.txt").unwrap();
        BufReader::new(file).lines().map(|s| s.unwrap().to_string())
    }

    #[test]
    fn test_part1_examples() {
        let res = part1(read_examples_file());
        assert_eq!(12, res);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
    }

    #[test]
    fn test_part2_examples() {
        let res = part2(read_examples_file());
        assert_eq!(19, res);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
    }
}
