use roller::Die;

fn main() {
    let die = Die::new(6);

    println!("Rolling 1d6: {}", die.roll())
}
