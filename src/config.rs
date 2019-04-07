use super::dice::{Die,Roller,RollResult};

pub struct Config {
    definition: String,
    roller: Roller,
    program: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        // TODO use something like clap or getopts to make this more robust
        let program = &args.get(0).unwrap();
        let definition = &args.get(1).unwrap_or(&"1d6".to_string()).to_string();
        // TODO make this dynamic from the definition
        let roller = Roller::new(
            vec![Die::new(10), Die::new(10)],
            2
        );

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
    // let roller = Roller::new(
    //     vec![
    //       Die::new(10),
    //       Die::new(10),
    //     ],
    //     4,
    // );
    // let result = roller.roll();

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn config_parse_interprets_input() {
        let args: Vec<String> = vec!["roll".to_string(), "2d10".to_string()];
        let config = Config::new(&args);

        assert_eq!("roll", config.program);
        assert_eq!("2d10", config.definition);
    }
}
