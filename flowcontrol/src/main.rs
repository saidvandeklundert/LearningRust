use rand::{thread_rng, Rng};
fn main() {
    let two: i32 = 2;
    let five: i32 = 5;
    let value = thread_rng().gen_range(0, 10);
    println!("Hello, world!\nGenerated number {}.", value);

    if value < two {
        print!("{} is less then 2", value);
    } else if value < five {
        print!("{} is less then 5", value);
    } else {
        print!("no prior condition was matched.");
    };
    // looping over an array:
    let array_i32: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for x in array_i32.iter() {
        print!("\n{} is in the array", x)
    }
    // using range does not require range:
    for num in 0..3 {
        println!("Ranging over numbers, currently at {}", num)
    }
    // now INCLUDING the last number:
    for num in 0..=3 {
        println!("Ranging over numbers, currently at {}", num)
    }
}
