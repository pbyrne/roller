use super::dice;

pub struct Config {
    definition: String,
    roller: dice::Roller,
    program: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        // TODO use something like clap or getopts to make this more robust
        let program = &args[0];
        let definition = &args[1];
        // TODO make this dynamic from the definition
        let roller = dice::Roller::new(
            vec![dice::Die::new(10), dice::Die::new(10)],
            2
        );

        Config {
            definition: definition.to_string(),
            roller,
            program: program.to_string(),
        }
    }

    pub fn result(&self) -> dice::Result {
        self.roller.roll()
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
