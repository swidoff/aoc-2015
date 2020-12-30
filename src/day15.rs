use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day15.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Vec<i64>> {
    lines
        .map(|line| {
            let (_, properties) = line.split(": ").collect_tuple().unwrap();
            let properties = properties.split(", ").collect_vec();
            properties
                .iter()
                .map(|p| {
                    let (_, value) = p.split_whitespace().collect_tuple().unwrap();
                    i64::from_str(value).unwrap()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn maximize_score(ingredients: &Vec<Vec<i64>>, teaspoons: i64, calories: Option<i64>) -> i64 {
    let allocations = all_allocations(teaspoons, ingredients.len());
    let num_properties = ingredients[0].len();

    allocations
        .iter()
        .map(|allocation| {
            ingredients
                .iter()
                .zip(allocation)
                .map(|(v, a)| v.iter().map(|n| *n * *a).collect_vec())
                .fold(vec![0; num_properties], |totals, i| {
                    totals
                        .iter()
                        .zip(i.iter())
                        .map(|(t, v)| *t + *v)
                        .collect_vec()
                })
        })
        .filter(|v| calories.map(|c| v[num_properties - 1] == c).unwrap_or(true))
        .map(|v| {
            v.into_iter()
                .take(num_properties - 1)
                .map(|n| n.max(0))
                .product::<i64>()
        })
        .max()
        .unwrap()
}

fn all_allocations(teaspoons: i64, num_ingredients: usize) -> VecDeque<Vec<i64>> {
    let mut allocations: VecDeque<Vec<i64>> = VecDeque::new();
    allocations.push_back(Vec::new());
    while let Some(allocation) = allocations.front() {
        if allocation.len() == num_ingredients {
            break;
        } else {
            let allocation = allocations.pop_front().unwrap();
            let remaining: i64 = teaspoons - allocation.iter().sum::<i64>();
            let min = if allocation.len() == num_ingredients - 1 {
                remaining
            } else {
                0
            };
            for i in min..(remaining + 1) {
                let mut new_allocation = allocation.clone();
                new_allocation.push(i);
                allocations.push_back(new_allocation);
            }
        }
    }
    allocations
}

mod tests {
    use crate::day15::{maximize_score, parse_input, read_file};

    const EXAMPLE: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";

    #[test]
    fn test_part1_example() {
        let ingredients = parse_input(EXAMPLE.lines().map(|s| s.to_string()));
        let res = maximize_score(&ingredients, 100, None);
        assert_eq!(62842880, res);
    }

    #[test]
    fn test_part1() {
        let ingredients = parse_input(read_file());
        let res = maximize_score(&ingredients, 100, None);
        println!("{}", res);
        assert_eq!(13882464, res);
    }

    #[test]
    fn test_part2_example() {
        let ingredients = parse_input(EXAMPLE.lines().map(|s| s.to_string()));
        let res = maximize_score(&ingredients, 100, Some(500));
        assert_eq!(57600000, res);
    }

    #[test]
    fn test_part2() {
        let ingredients = parse_input(read_file());
        let res = maximize_score(&ingredients, 100, Some(500));
        println!("{}", res);
        // assert_eq!(13882464, res);
    }
}
