use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> String {
    let file = File::open("input/day1.txt").unwrap();
    let mut str = String::new();
    BufReader::new(file).read_to_string(&mut str).unwrap();
    str
}

fn net_floor_count(parens: &str) -> i32 {
    parens.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

fn basement_position(parens: &str) -> Option<usize> {
    parens
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .scan(0, |floor, dir| {
            *floor += dir;
            Some(*floor)
        })
        .enumerate()
        .find_map(|(pos, floor)| if floor == -1 { Some(pos + 1) } else { None })
}

#[cfg(test)]
mod tests {
    use crate::day1::{basement_position, net_floor_count, read_file};

    #[test]
    fn part1_examples() {
        assert_eq!(0, net_floor_count("(())"));
        assert_eq!(0, net_floor_count("()()"));
        assert_eq!(3, net_floor_count("((("));
        assert_eq!(3, net_floor_count("(()(()("));
        assert_eq!(3, net_floor_count("))((((("));
        assert_eq!(-1, net_floor_count("())"));
        assert_eq!(-1, net_floor_count("))("));
        assert_eq!(-3, net_floor_count(")))"));
        assert_eq!(-3, net_floor_count(")())())"));
    }

    #[test]
    fn part1() {
        let input = read_file();
        let res = net_floor_count(input.as_str());
        println!("{}", res);
        assert_eq!(74, res);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Some(1), basement_position(")"));
        assert_eq!(Some(5), basement_position("()())"));
    }

    #[test]
    fn part2() {
        let input = read_file();
        let res = basement_position(input.as_str()).unwrap();
        println!("{}", res);
        assert_eq!(1795, res);
    }
}
