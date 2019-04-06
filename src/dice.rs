use rand::{thread_rng,Rng};

pub struct Die {
    pub sides: u32,
}

impl Die {
    pub fn new(sides: u32) -> Die {
        Die {
            sides: sides,
        }
    }

    pub fn roll(&self) -> u32 {
        let mut rng = thread_rng();
        rng.gen_range(1, self.sides + 1)
    }
}

#[cfg(test)]
mod tests {
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

