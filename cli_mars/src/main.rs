use std::io;

fn main() {
    // input 'own' the String created by String::new().
    // When input goes out of scope, the value is deallocated.
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("Input: {}", input);
    let mars_weight = calculate_weight_on_mars(92.0);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

/*
1. Each value in Rust is owned by a variable.

2. When the owner goes out of scope, the value will be deallocated.

3. There can only be ONE owner at a time.
*/
