use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day14.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

struct Reindeer {
    name: String,
    speed: u64,
    duration: u64,
    rest: u64,
}

fn parse_input(iter: impl Iterator<Item = String>) -> Vec<Reindeer> {
    iter.map(|line| {
        let tokens = line.split_whitespace().collect_vec();
        let name = tokens[0].to_string();
        let speed = u64::from_str(tokens[3]).unwrap();
        let duration = u64::from_str(tokens[6]).unwrap();
        let rest = u64::from_str(tokens[tokens.len() - 2]).unwrap();
        Reindeer {
            name,
            speed,
            duration,
            rest,
        }
    })
    .collect_vec()
}

fn part1(reindeer: &Vec<Reindeer>, seconds: u64) -> u64 {
    reindeer
        .iter()
        .map(|d| {
            let period_len = d.duration + d.rest;
            let periods = seconds / period_len;
            let remainder = seconds % period_len;
            d.speed * d.duration * periods + d.speed * d.duration.min(remainder)
        })
        .max()
        .unwrap()
}

fn part2(reindeer: &Vec<Reindeer>, seconds: u64) -> u64 {
    let mut running = vec![true; reindeer.len()];
    let mut timer = reindeer.iter().map(|d| d.duration).collect_vec();
    let mut scores: Vec<u64> = vec![0; reindeer.len()];
    let mut distances = vec![0; reindeer.len()];

    for _ in 0..seconds {
        for (i, reindeer) in reindeer.iter().enumerate() {
            if running[i] {
                distances[i] += reindeer.speed;
            }
            timer[i] -= 1;
            if timer[i] == 0 {
                if running[i] {
                    timer[i] = reindeer.rest;
                } else {
                    timer[i] = reindeer.duration;
                }
                running[i] = !running[i];
            }
        }

        let winning_distance = *distances.iter().max().unwrap();
        for (i, distance) in distances.iter().enumerate() {
            if *distance == winning_distance {
                scores[i] += 1;
            }
        }
    }

    *scores.iter().max().unwrap()
}

mod tests {
    use crate::day14::{parse_input, part1, part2, read_file};
    use itertools::Itertools;

    const EXAMPLE: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn test_part1_example() {
        let reindeer = parse_input(EXAMPLE.lines().map(|v| v.to_string()));
        assert_eq!(1120, part1(&reindeer, 1000));
    }

    #[test]
    fn test_part1() {
        let reindeer = parse_input(read_file());
        println!("{}", part1(&reindeer, 2503));
    }

    #[test]
    fn test_part2_example() {
        let reindeer = parse_input(EXAMPLE.lines().map(|v| v.to_string()));
        assert_eq!(689, part2(&reindeer, 1000));
    }

    #[test]
    fn test_part2() {
        let reindeer = parse_input(read_file());
        println!("{}", part2(&reindeer, 2503));
    }
}
