use itertools::Itertools;

fn look_and_say(input: &Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();

    for (key, group) in &input.into_iter().group_by(|v| **v) {
        res.push(group.count() as u8);
        res.push(key);
    }

    res
}

fn part1(input: Vec<u8>, iterations: usize) -> Vec<u8> {
    (0..iterations).fold(input, |prev, _| look_and_say(&prev))
}

#[cfg(test)]
mod tests {
    use crate::day10::part1;
    use itertools::Itertools;

    #[test]
    fn test_part1_example() {
        let res = part1(vec![1], 5);
        assert_eq!(
            res.iter().map(|d| d.to_string()).collect_vec().join(""),
            "312211"
        );
    }

    #[test]
    fn test_part1() {
        let res = part1(vec![1, 1, 1, 3, 1, 2, 2, 1, 1, 3], 40);
        println!("{}", res.len());
    }

    #[test]
    fn test_part2() {
        let res = part1(vec![1, 1, 1, 3, 1, 2, 2, 1, 1, 3], 50);
        println!("{}", res.len());
    }
}
