use super::dice::{RollResult, Roller};

pub struct Config {
    definition: String,
    roller: Roller,
    program: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        // TODO use something like clap or getopts to make this more robust
        let program = &args[0];
        let definition = &args.get(1).unwrap_or(&"1d6".to_string()).to_string();
        let roller = Roller::parse(definition);

        Config {
            definition: definition.to_string(),
            roller,
            program: program.to_string(),
        }
    }

    pub fn result(&self) -> RollResult {
        self.roller.roll()
    }

    pub fn run(&self) {
        // TODO do different things based on input (`--help`, `--version`)
        let result = self.result();

        println!("Rolled {} (from {:?})", result.total(), result.rolls());
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn config_new_interprets_input() {
        let args: Vec<String> = vec!["roll".to_string(), "2d10".to_string()];
        let config = Config::new(&args);

        assert_eq!("roll", config.program);
        assert_eq!("2d10", config.definition);
    }
}
