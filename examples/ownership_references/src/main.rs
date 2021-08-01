// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut)]

fn main() {
    // get the first argument as a String, or print usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&mut arg);

    // take a *mutable* reference to a String and add an "s" to
    // the String if it doesn't already end with "s".
    change(&mut arg);
    println!("{}", arg);

    if eat(arg.clone()) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    if eat_short(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Function that takes references to two integer arguments, dereferences them and adds them together
    add(&1, &2)
}

fn add(int1: &i32, int2: &i32) {
    let sum = *int1 + *int2;
    println!("{}", sum);
}

// check if a string might be bananas
fn eat(s: String) -> bool {
    if s.contains("a") && s.starts_with("b") {
        return true;
    } else {
        return false;
    }
}

fn eat_short(s: String) -> bool {
    s.contains("a") && s.starts_with("b")
}
fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("PLURAL")
    } else {
        println!("NOT PLURAL")
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    } else {
        println!("nothing to add")
    }
}
