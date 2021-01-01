use std::collections::HashSet;

fn part1(houses: usize) -> Vec<usize> {
    let mut res = vec![10; houses];
    for elf in 2..houses + 1 {
        let mut house = elf;
        while house < houses {
            res[house - 1] += elf * 10;
            house += elf;
        }
    }
    res
}

fn part2(houses: usize) -> Vec<usize> {
    let mut res = vec![10; houses];
    for elf in 2..houses + 1 {
        let mut house = elf;
        let mut i = 0;
        while house < houses && i < 50 {
            res[house - 1] += elf * 11;
            house += elf;
            i += 1
        }
    }
    res
}

fn min_house(presents: Vec<usize>, min: usize) -> usize {
    let res = presents
        .iter()
        .enumerate()
        .find_map(
            |(i, presents)| {
                if *presents >= min {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .unwrap();
    res
}

#[cfg(test)]
mod tests {
    use crate::day20::{min_house, part1, part2};

    #[test]
    fn test_part1() {
        let presents = part1(1500000);
        let res = min_house(presents, 33100000);
        println!("{}", res);
        assert_eq!(776160, res);
    }

    #[test]
    fn test_part2() {
        let presents = part2(1500000);
        let res = min_house(presents, 33100000);
        println!("{}", res);
        // assert_eq!(776160, res);
    }
}
