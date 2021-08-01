use rand::{thread_rng, Rng};
use scope::printer;

fn main() {
    println!("Hello, world!");
    printer();
    let rng = thread_rng().gen_range(0, 10);
    println!("{}", rng);
}
