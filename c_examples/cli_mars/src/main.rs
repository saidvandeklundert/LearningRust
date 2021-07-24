use std::io;

fn main() {
    println!("Enter your weight in kg:");
    // input 'own' the String created by String::new().
    // When input goes out of scope, the value is deallocated.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);

    // convert string to f32 after removing whitespace:
    //      unwrap handles the fact that parse returns a result
    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);

    let mars_weight = calculate_weight_on_mars(weight);
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
