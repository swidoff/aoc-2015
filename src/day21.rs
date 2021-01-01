use itertools::Itertools;

const WEAPONS: [(usize, usize, usize); 5] =
    [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

const ARMOR: [(usize, usize, usize); 6] = [
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RINGS: [(usize, usize, usize); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

const NO_RINGS: (usize, usize, usize) = (0, 0, 0);

fn items() -> Vec<(usize, usize, usize)> {
    let mut ring_combos = Vec::new();
    for combo in RINGS.iter().combinations(2) {
        ring_combos.push(combo);
    }
    for combo in RINGS.iter().combinations(1) {
        ring_combos.push(combo);
    }
    ring_combos.push(vec![&NO_RINGS]);

    let mut res = Vec::new();
    for (weapon_cost, weapon_dam, weapon_arm) in WEAPONS.iter() {
        for (arm_cost, arm_dam, arm_arm) in ARMOR.iter() {
            for rings in ring_combos.iter() {
                let (ring_cost, ring_dam, ring_arm) =
                    rings.iter().fold((0, 0, 0), |(tc, td, ta), (c, d, a)| {
                        (tc + c, td + d, ta + a)
                    });

                res.push((
                    weapon_cost + arm_cost + ring_cost,
                    weapon_dam + arm_dam + ring_dam,
                    weapon_arm + arm_arm + ring_arm,
                ));
            }
        }
    }

    res
}

fn win(hp1: usize, dam1: usize, arm1: usize, hp2: usize, dam2: usize, arm2: usize) -> bool {
    let hit1 = if dam1 > arm2 { dam1 - arm2 } else { 1 };
    let hit2 = if dam2 > arm1 { dam2 - arm1 } else { 1 };
    let rounds1 = hp2 / hit1 + if hp2 % hit1 > 0 { 1 } else { 0 };
    let rounds2 = hp1 / hit2 + if hp1 % hit2 > 0 { 1 } else { 0 };
    rounds1 <= rounds2
}

fn part1() -> usize {
    *items()
        .iter()
        .sorted_by_key(|t| t.0)
        .find_map(|(cost, dam, arm)| {
            if win(100, *dam, *arm, 103, 9, 2) {
                Some(cost)
            } else {
                None
            }
        })
        .unwrap()
}

fn part2() -> usize {
    *items()
        .iter()
        .sorted_by_key(|t| -(t.0 as i64))
        .find_map(|(cost, dam, arm)| {
            if !win(100, *dam, *arm, 103, 9, 2) {
                Some(cost)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day21::{part1, part2};

    #[test]
    fn test_part1() {
        println!("{}", part1())
    }

    #[test]
    fn test_part2() {
        println!("{}", part2())
    }
}
