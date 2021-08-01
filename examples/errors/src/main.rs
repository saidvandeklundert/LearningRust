use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
const FILE_PATH: &str = r"C:\dev-container\LearningRust\errors\src\example.txt";

fn main() {
    /*

    Rust groups errors into 2 major categories:
    - recoverable
    - unrecoverable
    No exceptions in Rust, instead, there is the Result enum.
    It is available without having imported it.
    https://ibm-learning.udemy.com/course/rust-fundamentals/learn/lecture/20695780#questions

    */
    // The Result enum is used when handling recoverable errors. The enum is as follows:
    /*
    pub enum Result<T, E>{
        /// Contains the success value
        Ok(T),
        /// Contains the error value
        Err(E),
    }
    */
    // Triggering an error by opening a file that does not exist:
    let non_existing_file = File::open("I_do_not_exist.txt");
    dbg!(non_existing_file);
    // This gives us an Error:
    /*
        [src\main.rs:236] file = Err(
            Os {
                code: 2,
                kind: NotFound,
                message: "The system cannot find the file specified.",
            },
        )
    */
    // Now we open a file that DOES exist:
    let existing_file = File::open(FILE_PATH);
    dbg!(existing_file);
    // This gives us the following:
    /*
        [src\main.rs:247] existing = Ok(
            File {
                handle: 0x00000000000000c0,
                path: "\\\\?\\C:\\dev-container\\LearningRust\\exploring\\csv\\src\\example.sv\\src\\example.csv",
            },
        )
    */

    // Now we open a file and use match to 'handle' the possible error:
    let f = File::open(FILE_PATH);
    let f = match f {
        Ok(file) => {
            println!("Filen Opened succesfully!");
        }
        Err(e) => {
            println!("Error opening file: {}", e)
        }
    };
    // Same thing again, except this time we read and print the content:
    let f2 = File::open(FILE_PATH);
    let f2 = match f2 {
        Ok(mut file) => {
            println!("Filen Opened succesfully!");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("Content of the file:\n {}", contents)
        }
        Err(e) => {
            println!("Error opening file: {}", e)
        }
    };

    // We can also use unwrap() to unwrap the Result value.
    // If the Result is Ok, unwrap will give you the value inside the Ok.
    // If the Result contains an error, unwrap will call the panic! macro.
    let mut f3 = File::open(FILE_PATH).unwrap();
    let mut contents3 = String::new();
    f3.read_to_string(&mut contents3).unwrap();
    println!("Content of the file f3:\n {}", contents3);
    // Instead of unwrap, we can also use expect(). It does the same, but it allows us to specify a panic message:
    let mut f4 = File::open(FILE_PATH).expect("Some informative message to accompany the panic");
    let mut contents4 = String::new();
    f4.read_to_string(&mut contents4).unwrap();
    println!("Content of the file f4:\n {}", contents4);

    // We can also open a file in a function and let the error propagate in case we run into one:

    fn read_file_func() -> Result<String, io::Error> {
        let f = File::open(FILE_PATH);

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    let read_file_funcs = read_file_func().unwrap();
    println!("read_file_funcs: {}", read_file_funcs);
    // Shortening the previous func:
    fn read_file_func_shorter() -> Result<String, io::Error> {
        let mut f = File::open(FILE_PATH)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    let read_file_func_shorters = read_file_func_shorter().unwrap();
    println!("read_file_func_shorters: {}", read_file_func_shorters);
    // We can shorten this even further:
    fn open_file() -> Result<String, io::Error> {
        fs::read_to_string(FILE_PATH)
    }
    let string_we_read = open_file().unwrap();
    println!("Content of the file: {}", string_we_read);
}
