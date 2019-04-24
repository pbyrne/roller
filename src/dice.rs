use rand::{thread_rng,Rng};
use regex::Regex;

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

    fn generate_dice(count: u64, sides: u64) -> Vec<Die> {
        let mut result: Vec<Die> = vec![];
        // vec![Die::new(sides)]
        for _ in 0..count {
            result.push(Die::new(sides))
        };

        result
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

pub struct RollResult {
    modifier: i64,
    rolls: Vec<u64>,
}

impl RollResult {
    pub fn total(&self) -> i64 {
        self.modifier + self.rolls.iter().sum::<u64>() as i64
    }

    pub fn rolls(&self) -> &Vec<u64> { &self.rolls }
}

pub struct Roller {
    dice: Vec<Die>,
    modifiers: Vec<i64>,
}

impl Roller {
    pub fn new(dice: Vec<Die>, modofiers: Vec<i64>) -> Roller {
        Roller {
            dice,
            modifiers,
        }
    }

    pub fn parse(definition: &str) -> Roller {
        let mut dice = vec!<Die>;
        let mut modifiers = vec!<i64>

        definition.split(char::is_whitespace).each 
        let regex = Regex::new(r"(?P<num>\d+)?d(?P<sides>\d+)(?P<modifier>[+-]\d+)?").unwrap();
        let capture = regex.captures(definition).unwrap();
        let num: u64 = match capture.name("num") {
            Some(value) => value.as_str().parse().unwrap(),
            None => 1,
        };
        let sides: u64 = match capture.name("sides") {
            Some(value) => value.as_str().parse().unwrap(),
            None => 6,
        };
        let modifier: i64 = match capture.name("modifier") {
            Some(value) => value.as_str().parse().unwrap(),
            None => 0,
        };

        let dice = Die::generate_dice(num, sides);

        Roller::new(dice, modifier)
    }

    pub fn roll(&self) -> RollResult {
        let rolls = self.dice.iter().map(Die::roll).collect();

        RollResult {
            modifiers: self.modifiers,
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
        let modifiers = vec![6];
        let roller = Roller::new(dice, modifiers);

        assert_eq!(2, roller.dice.len());
        assert_eq!(6, roller.dice[0].sides);
        assert_eq!(4, roller.dice[1].sides);
        assert_eq!([6], roller.modifiers);
    }

    #[test]
    fn roller_roll_returns_result() {
        let roller = Roller::new(vec![Die::new(6), Die::new(4)], 4);
        let result = roller.roll();

        assert_eq!(result.rolls.len(), roller.dice.len());
        assert_eq!(result.modifier, roller.modifier);
    }

    #[test]
    fn roller_parse_accepts_d_n() {
        let definition = "d4";
        let roller = Roller::parse(definition);

        assert_eq!(1, roller.dice.len());
        assert_eq!(4, roller.dice[0].sides);
        assert_eq!(0, roller.modifier)
    }

    #[test]
    fn roller_parse_accepts_n_d_m() {
        let definition = "2d8";
        let roller = Roller::parse(definition);

        assert_eq!(2, roller.dice.len());
        assert_eq!(8, roller.dice[0].sides);
        assert_eq!(8, roller.dice[1].sides);
        assert_eq!(0, roller.modifier);
    }

    #[test]
    fn roller_parse_accepts_n_d_m_plus_x() {
        let definition = "3d4+5";
        let roller = Roller::parse(definition);

        assert_eq!(3, roller.dice.len());
        assert_eq!(4, roller.dice[0].sides);
        assert_eq!(5, roller.modifier);
    }

    #[test]
    fn roller_parse_accepts_n_d_m_minux_x() {
        let definition = "1d20-5";
        let roller = Roller::parse(definition);

        assert_eq!(1, roller.dice.len());
        assert_eq!(20, roller.dice[0].sides);
        assert_eq!(-5, roller.modifier);
    }

    #[test]
    fn roller_parse_accepts_n_d_m_plus_x_d_y() {
        let definition = "2d4 1d8";
        let roller = Roller::parse(definition);

        assert_eq!(3, roller.dice.len());
        assert_eq!(4, roller.dice[0].sides);
        assert_eq!(4, roller.dice[1].sides);
        assert_eq!(8, roller.dice[2].sides);
    }

    // fn roller_parse_accepts_n_d_m_plus_x_d_y_plus_z()
    // fn roller_parse_accepts_arbitrary_whitespace()
}

#[cfg(test)]
mod test_result {
    use super::*;

    #[test]
    fn result_total_sums_rolls() {
        let result = RollResult {
            modifier: 0,
            rolls: vec![1, 2, 3, 4],
        };

        assert_eq!(10, result.total())
    }

    #[test]
    fn result_total_adds_modifier() {
        let result = RollResult {
            modifier: 2,
            rolls: vec![1, 2],
        };

        assert_eq!(5, result.total())
    }

    #[test]
    fn result_total_subtracts_modifier() {
        let result = RollResult {
            modifier: -4,
            rolls: vec![6, 7],
        };

        assert_eq!(9, result.total())
    }
}
