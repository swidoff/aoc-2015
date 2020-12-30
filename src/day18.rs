use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_input(iter: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    iter.map(|line| {
        line.chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect_vec()
    })
    .collect_vec()
}

fn count_lit_neighbors(lights: &Vec<Vec<u8>>, row: usize, col: usize) -> u64 {
    let mut count = 0;
    let min_row = if row == 0 { 0 } else { row - 1 };
    let max_row = (row + 2).min(lights.len());
    let min_col = if col == 0 { 0 } else { col - 1 };
    let max_col = (col + 2).min(lights.len());

    for r in min_row..max_row {
        for c in min_col..max_col {
            if !(r == row && c == col) {
                count += lights[r][c] as u64;
            }
        }
    }
    count
}

fn animate(lights: &mut Vec<Vec<u8>>, iterations: usize, corners_on: bool) {
    let dim = lights.len();
    let mut changes = HashMap::new();
    if corners_on {
        lights[0][0] = 1;
        lights[0][dim - 1] = 1;
        lights[dim - 1][0] = 1;
        lights[dim - 1][dim - 1] = 1;
    }

    for _ in 0..iterations {
        for row in 0..dim {
            for col in 0..dim {
                let lit_neighbors = count_lit_neighbors(lights, row, col);
                match lights[row][col] {
                    0 if lit_neighbors == 3 => changes.insert((row, col), 1),
                    1 if lit_neighbors < 2 || lit_neighbors > 3 => changes.insert((row, col), 0),
                    _ => None,
                };
            }
        }

        for ((row, col), val) in changes.iter() {
            lights[*row][*col] = *val;
        }
        changes.clear();

        if corners_on {
            lights[0][0] = 1;
            lights[0][dim - 1] = 1;
            lights[dim - 1][0] = 1;
            lights[dim - 1][dim - 1] = 1;
        }
    }
}

fn count_lights(input: &mut Vec<Vec<u8>>) -> u64 {
    input
        .iter()
        .map(|v| v.iter().map(|v| *v as u64).sum::<u64>())
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::day18::{animate, count_lights, parse_input, read_file};

    const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..
";

    #[test]
    fn test_part1_example() {
        let mut input = parse_input(EXAMPLE.lines().map(|s| s.to_string()));
        animate(&mut input, 4, false);
        let res = count_lights(&mut input);
        assert_eq!(4, res);
    }

    #[test]
    fn test_part1() {
        let mut input = parse_input(read_file());
        animate(&mut input, 100, false);
        let res = count_lights(&mut input);
        println!("{}", res);
    }

    #[test]
    fn test_part2_example() {
        let mut input = parse_input(EXAMPLE.lines().map(|s| s.to_string()));
        animate(&mut input, 5, true);
        let res = count_lights(&mut input);
        assert_eq!(17, res);
    }

    #[test]
    fn test_part2() {
        let mut input = parse_input(read_file());
        animate(&mut input, 100, true);
        let res = count_lights(&mut input);
        println!("{}", res);
    }
}
