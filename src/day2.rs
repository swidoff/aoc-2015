use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day2.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_lines(iter: impl Iterator<Item = String>) -> impl Iterator<Item = (u32, u32, u32)> {
    iter.map(|line| {
        line.split("x")
            .map(|s| u32::from_str(s).unwrap())
            .collect_tuple()
            .unwrap()
    })
}

fn required_wrapping(l: u32, w: u32, h: u32) -> u32 {
    let mut sides = [l, w, h];
    sides.sort();
    let (min_side1, min_side2) = sides.iter().take(2).collect_tuple().unwrap();
    2 * l * w + 2 * w * h + 2 * h * l + min_side1 * min_side2
}

#[cfg(test)]
mod tests {
    use crate::day2::{parse_lines, read_file, required_wrapping};

    #[test]
    fn part1_examples() {
        assert_eq!(58, required_wrapping(2, 3, 4));
        assert_eq!(43, required_wrapping(1, 1, 10));
    }

    #[test]
    fn part1() {
        let input = parse_lines(read_file());
        let res: u32 = input.map(|(l, w, h)| required_wrapping(l, w, h)).sum();
        println!("{}", res);
    }
}
