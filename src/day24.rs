use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> Vec<u64> {
    let file = File::open("input/day24.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| u64::from_str(s.unwrap().as_str()).unwrap())
        .collect_vec()
}

fn solve(packages: &mut Vec<u64>, groups: u64) -> u64 {
    packages.sort();
    packages.reverse();

    let mut res = Vec::new();
    let target = packages.iter().sum::<u64>() / groups;
    do_solve(packages, target, 0, &mut Vec::new(), &mut res);

    let min_len = res.iter().map(|v| v.len()).min().unwrap();

    let res = res
        .iter()
        .filter(|g| g.len() == min_len)
        .map(|g| g.iter().product::<u64>())
        .min()
        .unwrap();

    res
}

fn do_solve(
    packages: &mut Vec<u64>,
    target: u64,
    index: usize,
    group: &mut Vec<u64>,
    res: &mut Vec<Vec<u64>>,
) {
    if group.iter().sum::<u64>() == target {
        res.push(group.clone());
    } else if index < packages.len() {
        let package = packages[index];
        if group.iter().sum::<u64>() + package <= target {
            group.push(package);
            do_solve(packages, target, index + 1, group, res);
            group.pop();
        }

        do_solve(packages, target, index + 1, group, res);
    }
}

#[cfg(test)]
mod tests {
    use crate::day24::{read_file, solve};

    #[test]
    fn test_part1_example() {
        let mut packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let res = solve(&mut packages, 3);
        assert_eq!(99, res);
    }

    #[test]
    fn test_part1() {
        let mut packages = read_file();
        let res = solve(&mut packages, 3);
        // println!("{}", res);
        assert_eq!(10439961859, res);
    }

    #[test]
    fn test_part2_example() {
        let mut packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let res = solve(&mut packages, 4);
        assert_eq!(44, res);
    }

    #[test]
    fn test_part2() {
        let mut packages = read_file();
        let res = solve(&mut packages, 4);
        println!("{}", res);
        // assert_eq!(10439961859, res);
    }
}
