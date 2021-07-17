#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // collect command line args:
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Loop over args
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        };
    }
}

fn sum() {
    let mut sum = 0;
    for num in 7..=23 {
        sum += num
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x < 500 {
        x = x * 2;
        count += 1;
    }
    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut n = 0;
    while n < 8 {
        print!("{} ", arg);

        // Increment counter
        n += 1;
    }
    println!();
}
