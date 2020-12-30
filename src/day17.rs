use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day17.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_input(iter: impl Iterator<Item = String>) -> Vec<u64> {
    iter.map(|v| u64::from_str(v.as_str()).unwrap())
        .collect_vec()
}

fn summing_combinations(
    containers: &Vec<u64>,
    i: usize,
    sum: u64,
    container_count: usize,
    res: &mut HashMap<usize, u64>,
) {
    if sum == 0 {
        let total = res.remove(&container_count).unwrap_or(0) + 1;
        res.insert(container_count, total);
    } else if i < containers.len() {
        summing_combinations(containers, i + 1, sum, container_count, res);
        if containers[i] <= sum {
            summing_combinations(
                containers,
                i + 1,
                sum - containers[i],
                container_count + 1,
                res,
            );
        }
    }
}

fn part1(containers: &Vec<u64>, sum: u64) -> u64 {
    let mut res = HashMap::new();
    summing_combinations(containers, 0, sum, 0, &mut res);
    res.values().sum()
}

fn part2(containers: &Vec<u64>, sum: u64) -> u64 {
    let mut res = HashMap::new();
    summing_combinations(containers, 0, sum, 0, &mut res);
    let min_containers = res.keys().min().unwrap();
    *res.get(min_containers).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day17::{parse_input, part1, part2, read_file, summing_combinations};

    #[test]
    fn test_part1_example() {
        let containers = vec![20, 15, 10, 5, 5];
        let res = part1(&containers, 25);
        assert_eq!(4, res);
    }

    #[test]
    fn test_part1() {
        let containers = parse_input(read_file());
        let res = part1(&containers, 150);
        println!("{}", res);
        assert_eq!(1304, res);
    }

    #[test]
    fn test_part2_example() {
        let containers = vec![20, 15, 10, 5, 5];
        let res = part2(&containers, 25);
        assert_eq!(3, res);
    }

    #[test]
    fn test_part2() {
        let containers = parse_input(read_file());
        let res = part2(&containers, 150);
        println!("{}", res);
        assert_eq!(18, res);
    }
}
