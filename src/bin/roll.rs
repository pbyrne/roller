use roller::config::Config;
use std::env;

fn main() {
    let config = Config::new(&env::args().collect());

    config.run()
}
