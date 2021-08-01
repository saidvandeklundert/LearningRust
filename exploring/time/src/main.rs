use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(2, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        println!("Hello, world!");
        count += 1;
    }
    println!("Printed \"Hello, world!\" {} times", count);
    let elapsed = start.elapsed();
    println!("Script ran for {:.4?} seconds", elapsed);
}
