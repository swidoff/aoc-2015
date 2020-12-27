use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::FromIterator;

fn read_file() -> String {
    let file = File::open("input/day3.txt").unwrap();
    let mut line = String::new();
    BufReader::new(file).read_to_string(&mut line).unwrap();
    line
}

type Coord = (i64, i64);

fn trace_path(line: impl Iterator<Item = char>) -> impl Iterator<Item = Coord> {
    line.map(|c| match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!("Invalid char: {}", c),
    })
    .scan((0, 0), |(x, y), (xd, yd)| {
        *x += xd;
        *y += yd;
        Some((*x, *y))
    })
    .chain([(0, 0)].iter().map(|c| *c))
}

fn part1(line: String) -> usize {
    trace_path(line.chars()).unique().count()
}

fn part2(line: String) -> usize {
    let santa: HashSet<Coord> = HashSet::from_iter(trace_path(line.chars().step_by(2)));
    let robo: HashSet<Coord> = HashSet::from_iter(trace_path(line.chars().skip(1).step_by(2)));
    santa.union(&robo).count()
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2, read_file};

    #[test]
    fn test_part1_examples() {
        assert_eq!(2, part1(">".to_string()));
        assert_eq!(4, part1("^>v<".to_string()));
        assert_eq!(2, part1("^v^v^v^v^v".to_string()));
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(3, part2("^v".to_string()));
        assert_eq!(3, part2("^>v<".to_string()));
        assert_eq!(11, part2("^v^v^v^v^v".to_string()));
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
    }
}
