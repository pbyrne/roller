use roller::config::Config;
use std::env;

fn main() {
    let config = Config::new(&env::args().collect());
    let result = config.result();

    println!("Rolled {} (from {:?})", result.total(), result.rolls());
}
