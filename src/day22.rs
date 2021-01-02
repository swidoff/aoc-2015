use itertools::Itertools;
use std::collections::HashMap;

struct Spell {
    id: &'static str,
    cost: i64,
    damage: i64,
    heal: i64,
    armor: i64,
    mana: i64,
    turns: u8,
}

const SPELLS: [Spell; 5] = [
    Spell {
        id: "Magic Missile",
        cost: 53,
        damage: 4,
        heal: 0,
        armor: 0,
        mana: 0,
        turns: 0,
    },
    Spell {
        id: "Drain",
        cost: 73,
        damage: 2,
        heal: 2,
        armor: 0,
        mana: 0,
        turns: 0,
    },
    Spell {
        id: "Shield",
        cost: 113,
        damage: 0,
        heal: 0,
        armor: 7,
        mana: 0,
        turns: 6,
    },
    Spell {
        id: "Poison",
        cost: 173,
        damage: 3,
        heal: 0,
        armor: 0,
        mana: 0,
        turns: 6,
    },
    Spell {
        id: "Recharge",
        cost: 229,
        damage: 0,
        heal: 0,
        armor: 0,
        mana: 101,
        turns: 5,
    },
];

fn win(
    hp1_initial: i64,
    mana1_initial: i64,
    hp2_initial: i64,
    dam2: i64,
    spells: &Vec<usize>,
) -> (bool, i64) {
    let mut effects: [u8; SPELLS.len()] = [0; SPELLS.len()];
    let mut hp1 = hp1_initial;
    let mut mana1 = mana1_initial;
    let mut hp2 = hp2_initial;
    let mut turn = 0;
    let mut cost = 0;
    loop {
        let mut arm1 = 0;
        for i in 0..SPELLS.len() {
            if effects[i] > 0 {
                let spell = &SPELLS[i];
                println!("EFFECT {} {}", spell.id, effects[i]);
                hp2 -= spell.damage;
                mana1 += spell.mana;
                hp1 += spell.heal;
                arm1 += spell.armor;
                effects[i] -= 1;
            }
        }

        if hp2 <= 0 {
            return (true, cost);
        }

        if turn % 2 == 0 {
            let i = turn / 2;
            if i < spells.len() {
                let spell_index = spells[i];
                let spell = &SPELLS[spell_index];
                if mana1 >= spell.cost {
                    println!("CAST {}", spell.id);
                    mana1 -= spell.cost;
                    cost += spell.cost;
                    if spell.turns == 0 {
                        hp2 -= spell.damage;
                        hp1 += spell.heal
                    } else if effects[spell_index] == 0 {
                        effects[spell_index] = spell.turns;
                    } else {
                        panic!("Spell still in effect!")
                    }
                } else {
                    panic!("Not enough mana!")
                }
            }
            if hp2 <= 0 {
                return (true, cost);
            }
        } else {
            hp1 -= if dam2 > arm1 { dam2 - arm1 } else { 1 };

            if hp1 <= 0 {
                return (false, cost);
            }
        }
        println!(
            "turn: {}, hp1: {}, hp2: {}, mana1: {}",
            turn, hp1, hp2, mana1
        );
        turn += 1;
    }
}

fn spell_combinations(max_spells: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();

    let combinations = (0..SPELLS.len())
        .combinations_with_replacement(max_spells)
        .flat_map(|s| s.into_iter().permutations(max_spells).unique());
    for c in combinations {
        res.push(c);
    }

    res.sort_by_key(|v| v.iter().map(|i| SPELLS[*i].cost).sum::<i64>());
    res
}

#[cfg(test)]
mod tests {
    use crate::day22::{spell_combinations, win};

    #[test]
    fn test_part1_example() {
        assert_eq!(true, win(10, 250, 13, 8, &vec![3, 0]).0);
        assert_eq!(true, win(10, 250, 14, 8, &vec![4, 2, 1, 3, 0]).0);
    }

    #[test]
    fn test_part1() {
        // let spells = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2]; 2173
        // let spells = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 0]; // 2113
        // let spells = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 0, 0, 3]; // 1937
        // let spells = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 0, 3]; // 1884
        let spells = vec![2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 0, 3];
        let (win, cost) = win(50, 500, 71, 10, &spells);
        println!("win: {}, cost: {}", win, cost);
    }
}
