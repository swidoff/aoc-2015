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
    // Spell {
    //     id: "Nothing",
    //     cost: 0,
    //     damage: 0,
    //     heal: 0,
    //     armor: 0,
    //     mana: 0,
    //     turns: 0,
    // },
];

#[derive(Clone, Debug)]
struct State {
    hp1: i64,
    mana1: i64,
    hp2: i64,
    dam2: i64,
    effects: [u8; SPELLS.len()],
    cost: i64,
    spells: Vec<usize>,
}

impl State {
    fn new(hp1: i64, mana1: i64, hp2: i64, dam2: i64) -> State {
        State {
            hp1,
            mana1,
            hp2,
            dam2,
            effects: [0; SPELLS.len()],
            cost: 0,
            spells: Vec::new(),
        }
    }

    fn player_won(&self) -> bool {
        self.hp2 <= 0
    }

    fn boss_won(&self) -> bool {
        self.hp1 <= 0
    }

    fn play(&self, spell_index: usize, hard: bool) -> State {
        let mut s = self.clone();
        for turn in 0..2 {
            let mut arm1 = 0;
            if hard && turn == 0 {
                s.hp1 -= 1;
            }

            if s.hp1 > 0 && s.hp2 > 0 {
                for i in 0..SPELLS.len() {
                    if s.effects[i] > 0 {
                        let spell = &SPELLS[i];
                        arm1 += spell.armor;
                        s.hp2 -= spell.damage;
                        s.mana1 += spell.mana;
                        s.hp1 += spell.heal;
                        s.effects[i] -= 1;
                    }
                }
            }

            if s.hp1 > 0 && s.hp2 > 0 {
                if turn % 2 == 0 {
                    let spell = &SPELLS[spell_index];
                    if s.mana1 >= spell.cost {
                        s.mana1 -= spell.cost;
                        s.cost += spell.cost;
                        s.spells.push(spell_index);
                        if spell.turns == 0 {
                            s.hp2 -= spell.damage;
                            s.hp1 += spell.heal
                        } else if s.effects[spell_index] == 0 {
                            s.effects[spell_index] = spell.turns;
                        } else {
                            panic!("Spell still in effect!")
                        }
                    } else {
                        panic!("Not enough mana!")
                    }
                } else {
                    s.hp1 -= if s.dam2 > arm1 { s.dam2 - arm1 } else { 1 };
                }
            }
        }

        s
    }
}

fn optimize(state: State, hard: bool) -> Option<State> {
    if state.player_won() {
        Some(state)
    } else if state.boss_won() {
        None
    } else {
        let mut res: Option<State> = None;
        for spell_index in 0..SPELLS.len() {
            let spell = &SPELLS[spell_index];
            if state.mana1 >= spell.cost && state.effects[spell_index] <= 1 {
                let new_state = state.play(spell_index, hard);
                match optimize(new_state, hard) {
                    Some(optimal) => match res.as_ref() {
                        Some(prev_optimal) if optimal.cost < prev_optimal.cost => {
                            res.replace(optimal);
                        }
                        None => {
                            res.replace(optimal);
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::day22::{optimize, State, SPELLS};

    #[test]
    fn test_part1_example1() {
        let mut state = State::new(10, 250, 13, 8);
        state = state.play(3, false);
        assert_eq!(2, state.hp1);
        assert_eq!(10, state.hp2);
        assert_eq!(5, state.effects[3]);
        assert_eq!(77, state.mana1);

        state = state.play(0, false);
        assert_eq!(2, state.hp1);
        assert_eq!(0, state.hp2);
        assert_eq!(3, state.effects[3]);
        assert_eq!(24, state.mana1);
        assert_eq!(true, state.player_won());
    }

    #[test]
    fn test_part1_example2() {
        let mut state = State::new(10, 250, 14, 8);
        state = state.play(4, false);
        assert_eq!(2, state.hp1);
        assert_eq!(14, state.hp2);
        assert_eq!(4, state.effects[4]);
        assert_eq!(122, state.mana1);

        state = state.play(2, false);
        assert_eq!(1, state.hp1);
        assert_eq!(14, state.hp2);
        assert_eq!(2, state.effects[4]);
        assert_eq!(211, state.mana1);

        state = state.play(1, false);
        assert_eq!(2, state.hp1);
        assert_eq!(12, state.hp2);
        assert_eq!(3, state.effects[2]);
        assert_eq!(0, state.effects[4]);
        assert_eq!(340, state.mana1);

        state = state.play(3, false);
        assert_eq!(1, state.hp1);
        assert_eq!(9, state.hp2);
        assert_eq!(1, state.effects[2]);
        assert_eq!(5, state.effects[3]);
        assert_eq!(167, state.mana1);

        state = state.play(0, false);
        assert_eq!(1, state.hp1);
        assert_eq!(-1, state.hp2);
        assert_eq!(0, state.effects[2]);
        assert_eq!(3, state.effects[3]);
        assert_eq!(114, state.mana1);
        assert_eq!(true, state.player_won());
    }

    #[test]
    fn test_part1() {
        let mut state = State::new(50, 500, 71, 10);
        let res = optimize(state, false);
        // println!("{:?}", res.unwrap());
        assert_eq!(1824, res.unwrap().cost);
    }

    #[test]
    fn test_part2() {
        let mut state = State::new(50, 500, 71, 10);
        let res = optimize(state, true);
        println!("{:?}", res.unwrap());
        // assert_eq!(1824, res.unwrap().cost);
    }
}
