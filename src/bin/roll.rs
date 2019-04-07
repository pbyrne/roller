use roller::dice::{Die,Roller};

fn main() {
    let roller = Roller::new(
        vec![
          Die::new(10),
          Die::new(10),
        ],
        4,
    );
    let result = roller.roll();

    println!("Rolled {} (from {:?})", result.total(), result.rolls());
}
