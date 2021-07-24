use std::env;
use std::error::Error;
use std::ffi::OsString;

fn first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected an arg, but got none")),
        Some(arg_1) => Ok(arg_1),
    }
}

fn main() {
    // Prints each argument on a separate line
    for argument in env::args_os() {
        dbg!(argument);
    }
    let arg_1 = first_arg();
    let figuring_out = arg_1.unwrap();
    let string = figuring_out.into_string().unwrap();
    println!("{}", string);
}
// Example file location:
// C:\dev-container\LearningRust\exploring\csv\src\example.csv
// inspiration: https://docs.rs/csv/1.0.0/csv/tutorial/index.html#reading-csv

// Learn by building real world applications The Result Enum: 32
