fn part1(target_row: usize, target_col: usize) -> u64 {
    let mut row = 1;
    let mut col = 1;
    let mut code = 20151125;

    while row != target_row || col != target_col {
        code = code * 252533 % 33554393;
        if row > 1 {
            col += 1;
            row -= 1;
        } else {
            row = col + 1;
            col = 1;
        }
    }

    code
}

#[cfg(test)]
mod tests {
    use crate::day25::part1;

    #[test]
    fn test_part1_example() {
        assert_eq!(31916031, part1(2, 1));
        assert_eq!(9250759, part1(5, 5));
    }

    #[test]
    fn test_part1() {
        println!("{}", part1(2978, 3083));
    }
}
