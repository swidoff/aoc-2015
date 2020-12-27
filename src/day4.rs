use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::FromIterator;

fn find_hash_with_prefix(key: &str, start: u64, prefix: &str) -> u64 {
    (start..u64::max_value())
        .find(|d| {
            let mut input = String::from(key);
            let number = d.to_string();
            input.push_str(&number[0..]);

            let hash = md5::compute(input);
            format!("{:x}", hash).starts_with(prefix)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day4::find_hash_with_prefix;

    #[test]
    fn test_part1_example() {
        assert_eq!(609043, find_hash_with_prefix("abcdef", 0, "00000"));
        assert_eq!(1048970, find_hash_with_prefix("pqrstuv", 0, "00000"));
    }

    #[test]
    fn test_part1() {
        println!("{}", find_hash_with_prefix("bgvyzdsv", 0, "00000"));
    }

    #[test]
    fn test_part2() {
        println!("{}", find_hash_with_prefix("bgvyzdsv", 254576, "000000"));
    }
}
