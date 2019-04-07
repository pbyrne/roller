use rand::{thread_rng,Rng};

#[derive(Debug)]
pub struct Die {
    sides: u64,
}

impl Die {
    pub fn new(sides: u64) -> Die {
        Die {
            sides,
        }
    }

    pub fn roll(&self) -> u64 {
        let mut rng = thread_rng();
        rng.gen_range(1, self.sides + 1)
    }
}

impl PartialEq for Die {
    fn eq(&self, other: &Die) -> bool {
        self.sides == other.sides
    }
}

pub struct Result {
    modifier: i64,
    rolls: Vec<u64>,
}

impl Result {
    pub fn total(&self) -> i64 {
        self.modifier + self.rolls.iter().sum::<u64>() as i64
    }

    pub fn rolls(&self) -> &Vec<u64> { &self.rolls }
}

pub struct Roller {
    dice: Vec<Die>,
    modifier: i64,
}

impl Roller {
    pub fn new(dice: Vec<Die>, modifier: i64) -> Roller {
        Roller {
            dice,
            modifier,
        }
    }

    pub fn roll(&self) -> Result {
        let rolls = self.dice.iter().map(|d| d.roll() ).collect();

        Result {
            modifier: self.modifier,
            rolls,
        }
    }
}

#[cfg(test)]
mod test_die {
    use super::*;

    #[test]
    fn die_new_accepts_sides() {
        let sides = 6;
        let die = Die::new(sides);

        assert_eq!(sides, die.sides);
    }

    #[test]
    fn die_roll_returns_random() {
        let sides = 6;
        let die = Die::new(sides);

        let result = die.roll();
        assert!(result <= die.sides);
    }
}

#[cfg(test)]
mod test_roller {
    use super::*;

    #[test]
    fn roller_new_accepts_input() {
        let dice = vec![Die::new(6), Die::new(4)];
        let modifier = 6;
        let roller = Roller::new(dice, modifier);

        assert_eq!(2, roller.dice.len());
        assert_eq!(6, roller.dice[0].sides);
        assert_eq!(4, roller.dice[1].sides);
        assert_eq!(modifier, roller.modifier);
    }

    #[test]
    fn roller_roll_returns_result() {
        let roller = Roller::new(vec![Die::new(6), Die::new(4)], 4);
        let result = roller.roll();

        assert_eq!(result.rolls.len(), roller.dice.len());
        assert_eq!(result.modifier, roller.modifier);
    }
}

#[cfg(test)]
mod test_result {
    use super::*;

    #[test]
    fn result_total_sums_rolls() {
        let result = Result {
            modifier: 0,
            rolls: vec![1, 2, 3, 4],
        };

        assert_eq!(10, result.total())
    }

    #[test]
    fn result_total_adds_modifier() {
        let result = Result {
            modifier: 2,
            rolls: vec![1, 2],
        };

        assert_eq!(5, result.total())
    }

    #[test]
    fn result_total_subtracts_modifier() {
        let result = Result {
            modifier: -4,
            rolls: vec![6, 7],
        };

        assert_eq!(9, result.total())
    }
}
