fn main() {
    println!("Hello, world!");

    // Create and call a function called 'multiply_result'.
    // The func should takes 2 i32's and return their product.
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    let muliply_result = multiply(2, 2);
    dbg!(muliply_result);

    // Create a function that takes a vector of strings
    // The function should return a vector with strings that start with an 'a'
    // If there is no string in the vector that starts with an 'a', return an error.
    let mut v1 = vec!["word", "abnormal", "normal", "special", "anti"];

    fn starts_with_a(mut v: Vec<&str>) {
        for i in (0..v.len()).rev() {
            let first_char = v[i].chars().nth(0).unwrap();
            if first_char == 'a' {
                v.swap_remove(i);
            } else {
                println!("{}", v[i]);
            }
        }
    }
    starts_with_a(v1);
}
