use itertools::Itertools;

const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn encode(str: &str) -> Vec<usize> {
    str.chars()
        .map(|c| LETTERS.binary_search(&c).unwrap())
        .collect_vec()
}

fn decode(password: &Vec<usize>) -> String {
    password.iter().map(|i| LETTERS[*i]).join("")
}

fn increment(password: &mut Vec<usize>) {
    let mut i = password.len() - 1;
    let mut carry = true;

    while i != 0 && carry {
        let index = password[i];
        let new_index = (index + 1) % LETTERS.len();
        password[i] = new_index;
        carry = new_index == 0;
        i -= 1;
    }
}

fn has_pairs(password: &Vec<usize>) -> bool {
    let group_by = password.into_iter().group_by(|v| **v);
    let num_pairs = group_by
        .into_iter()
        .filter_map(|(key, group)| if group.count() == 2 { Some(key) } else { None })
        .unique()
        .count();
    num_pairs >= 2
}

fn has_straight(password: &Vec<usize>) -> bool {
    password
        .iter()
        .tuple_windows()
        .any(|(i1, i2, i3)| *i2 == *i1 + 1 && *i3 == *i2 + 1)
}

fn has_legal_characters(password: &Vec<usize>) -> bool {
    password.iter().all(|c| *c != 8 && *c != 11 && *c != 14)
}

fn part1(password: &str) -> String {
    let mut password = encode(password);
    increment(&mut password);

    while !(has_legal_characters(&password) && has_straight(&password) && has_pairs(&password)) {
        increment(&mut password);
    }

    decode(&password)
}

#[cfg(test)]
mod tests {
    use crate::day11::part1;

    #[test]
    fn test_part1_example() {
        assert_eq!("abcdffaa", part1("abcdefgh"));
        assert_eq!("ghjaabcc", part1("ghijklmn"));
    }

    #[test]
    fn test_part1() {
        println!("{}", part1("vzbxkghb"));
    }

    #[test]
    fn test_part2() {
        println!("{}", part1("vzbxxyzz"));
    }
}
