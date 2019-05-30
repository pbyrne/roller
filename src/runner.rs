use super::dice::{RollResult, Roller};

pub struct Runner {
    definition: String,
    roller: Roller,
    verbose: bool,
}

impl Runner {
    pub fn new(definition: String, verbose: bool) -> Runner {
        let roller = Roller::parse(&definition);

        Runner {
            definition,
            roller,
            verbose,
        }
    }

    pub fn result(&self) -> RollResult {
        self.roller.roll()
    }

    pub fn run(&self) {
        let result = self.result();

        println!("{}", result.total());

        if self.verbose {
            eprintln!("Definition: {}", self.definition);
            eprintln!("Rolls: {:?}", result.rolls());
        }
    }
}

#[cfg(test)]
mod test_runner {
    use super::*;

    #[test]
    fn runner_new_interprets_input() {
        let definition = "2d4+5".to_string();
        let verbose = false;
        let runner = Runner::new(definition, verbose);

        assert_eq!("2d4+5", runner.definition);
        assert_eq!(false, runner.verbose);
    }
}
