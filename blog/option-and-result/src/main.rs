/*
You need to run this script from the directory that it is in.
*/
#[allow(dead_code)]

fn main() {
    {
        use rustc_version_runtime::version;

        println!("This was compiled using {:?}", version());
        // This was compiled using Version { major: 1, minor: 53, patch: 0, pre: [], build: [] }
    }
    // Option block
    // https://doc.rust-lang.org/std/option/
    {
        // Create an example enum and implement Display:
        use strum_macros::Display;
        #[derive(Display)]
        enum Example {
            This,
            That,
        }

        let this = Example::This;
        let that = Example::That;

        println!("Example::This contains: {}", this);
        println!("Example::That contains: {}", that);
        /*
        Example::This contains: This
        Example::That contains: That
        */

        fn matcher(x: Example) {
            match x {
                Example::This => println!("We got This."),
                Example::That => println!("We got That."),
            }
        }
        matcher(Example::This);
        matcher(Example::That);
        /*
        We got This.
        We got That
        */
    }
    // illustrating an Option is generic over other types:
    {
        let a_str: Option<&str> = Some("a str");
        let a_string: Option<String> = Some(String::from("a String"));
        let a_float: Option<f64> = Some(1.1);
        let a_vec: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]);
        #[derive(Debug)]
        struct Person {
            name: String,
            age: i32,
        }
        let marie = Person {
            name: String::from("Marie"),
            age: 2,
        };
        let a_person: Option<Person> = Some(marie);
        let maybe_someone: Option<Person> = None;
        println!(
            "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
            a_str, a_string, a_float, a_vec, a_person, maybe_someone
        );
    }
    // Illustrate pattern matching on the Option:
    {
        let something: Option<&str> = Some("a String");
        let nothing: Option<&str> = None;
        dbg!(something);
        dbg!(nothing);

        match something {
            Some(text) => println!("We go something: {}", text),
            None => println!("We got nothing."),
        }
        match nothing {
            Some(something_else) => println!("We go something: {}", something_else),
            None => println!("We got nothing"),
        }
    }
    // unwrap
    {
        let something: Option<&str> = Some("Something");
        let unwrapped = something.unwrap(); // "Something"
        println!("{:?}\n{:?}", something, unwrapped);
        let nothing: Option<&str> = None;
        // uncommenting the next line will cause a panic:
        //nothing.unwrap();
    }
    // Example function that takes optional value
    {
        fn might_print(option: Option<&str>) {
            match option {
                Some(text) => println!("The argument contains the following value: '{}'", text),
                None => println!("The argument contains None."),
            }
        }
        let something: Option<&str> = Some("some str");
        let nothing: Option<&str> = None;
        might_print(something);
        might_print(nothing);
    }
    // Example function that returns an optional
    {
        // A function that returns the text if it contains target character, None otherwise:
        fn contains_char(text: &str, target_c: char) -> Option<&str> {
            if text.chars().any(|ch| ch == target_c) {
                return Some(text);
            } else {
                return None;
            }
        }
        let a = contains_char("Rust in action", 'a');
        let q = contains_char("Rust in action", 'q');
        println!("{:?}", a);
        println!("{:?}", q);
        let a_unwrapped = a.unwrap();
        println!("{:?}", a_unwrapped);
        match a {
            Some(a) => println!("contains_char returned something: {:?}!", a),
            None => println!("contains_char did not return something, so branch off here"),
        }
        if let Some(a) = contains_char("Rust in action", 'a') {
            println!("contains_char returned something: {:?}!", a);
        } else {
            println!("contains_char did not return something, so branch off here")
        }
    }
    // Struct option
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Option<i32>,
        }
        let marie = Person {
            name: String::from("Marie"),
            age: Some(2),
        };
        let jan = Person {
            name: String::from("Jan"),
            age: None,
        };
        println!("{:?}\n{:?}", marie, jan);
    }
    // Use of Option in vector:
    {
        let mut vec = vec![0, 1];
        let a = vec.pop();
        let b = vec.pop();
        let c = vec.pop();
        println!("{:?}\n{:?}\n{:?}\n", a, b, c);
    }
    // Check length fun example:
    {
        fn check_length(s: &str, min: usize) -> Result<&str, String> {
            if s.chars().count() >= min {
                return Ok(s);
            } else {
                return Err(format!("'{}' is not long enough!", s));
            }
        }
        let a = check_length("some str", 5);
        let b = check_length("another str", 300);
        dbg!(a); // Ok("some str",)
        dbg!(b); // Err("'another str' is not long enough!",)
    }
    {
        fn check_length(s: &str, min: usize) -> Result<&str, String> {
            if s.chars().count() >= min {
                Ok(s)
            } else {
                Err(format!("'{}' is not long enough!", s))
            }
        }
        let a = check_length("some str", 5);
        let b = check_length("another str", 300);
        dbg!(a); // Ok("some str",)
        dbg!(b); // Err("'a' is not long enough!",)

        let func_return = check_length("some string literal", 1);
        let a_str = match func_return {
            Ok(a_str) => a_str,
            Err(error) => panic!("Problem running 'check_length':\n {:?}", error),
        };
        dbg!(a_str);
    }
    // Serde example
    {
        use serde_json::json;
        let json_string = r#"
        {
            "key": "value",
            "another key": "another value",
            "key to a list" : [1 ,2]
        }"#;
        let json_serialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        println!("{:?}", &json_serialized);
    }
    // Example opening a file and reading the JSON
    {
        use serde::{Deserialize, Serialize};
        use std::error::Error;
        use std::fs;

        // Debug allows us to print the struct.
        // Deserialize and Serialize adds decoder and encoder capabilities to the struct.
        #[derive(Debug, Deserialize, Serialize)]
        struct Person {
            name: String,
            age: usize,
        }
        fn file_to_json(s: &str) -> Result<Person, Box<dyn Error>> {
            let text = fs::read_to_string(s)?;
            let marie: Person = serde_json::from_str(&text)?;
            Ok(marie)
        }
        let x = file_to_json("json.txt");
        let y = file_to_json("invalid_json.txt");
        let z = file_to_json("non_existing_file.txt");
        dbg!(x);
        dbg!(y);
        dbg!(z);
    }
    // Example using anyhow
    {
        use anyhow::{anyhow, Context, Result};
        use serde::{Deserialize, Serialize};
        use std::fs;

        #[derive(Debug, Deserialize, Serialize)]
        struct Secrets {
            username: String,
            password: String,
        }

        fn get_secrets(s: &str) -> Result<Secrets> {
            let text = fs::read_to_string(s).context("Secrets file is missing.")?;
            let secrets: Secrets =
                serde_json::from_str(&text).context("Problem serializing secrets.")?;
            if secrets.password.chars().count() < 2 {
                return Err(anyhow!("Password in secrets file is too short"));
            }
            Ok(secrets)
        }
        let a = get_secrets("secrets.json");
        dbg!(a);

        let b = get_secrets("secrets.jsonnn");
        dbg!(b);

        let c = get_secrets("invalid_json.txt");
        dbg!(c);

        let d = get_secrets("wrong_secrets.json");
        dbg!(d);
    }
    // Example using eyre
    {
        use eyre::{EyreHandler, Report, Result, WrapErr};
        use serde::{Deserialize, Serialize};
        use std::fs;

        #[derive(Debug, Deserialize, Serialize)]
        struct Secrets {
            username: String,
            password: String,
        }

        fn secrets(s: &str) -> Result<Secrets> {
            let text = fs::read_to_string(s).wrap_err("Secrets file is missing.")?;
            let secrets: Secrets =
                serde_json::from_str(&text).wrap_err("Problem serializing secrets.")?;
            if secrets.username.chars().count() < 2 {
                //return Err(eyre!("Username in secrets file is too short"));
            }
            Ok(secrets)
        }
        //let a = secrets("secrets.json");
        //dbg!(a);

        //let b = secrets("secrets.jsonnn");
        //dbg!(b);

        //let c = secrets("invalid_json.txt");
        //dbg!(c);
    }
    // Some other examples
    {
        use std::fs::File;
        use std::io::prelude::*;
        let result_1 = File::open("exists.txt");
        let result_2 = File::open("does_not_exist.txt");
        println!("{:?}", result_1);
        println!("{:?}", result_2);
        let mut file = result_1.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    // Example from The Book:
    {
        use std::fs;
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn file_opener(file: &str) -> Result<String, io::Error> {
            let f = File::open(file);

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                // Because the prelude imports the variants
                //  we can use Ok and Err in this way.
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        let x = file_opener("exists.txt");
        dbg!(x);
        let y = file_opener("non-existing.txt");
        dbg!(y);
        fn quick_opener(file: &str) -> Result<String, io::Error> {
            let mut f = File::open(file)?; // < notice the ?
            let mut s = String::new();
            f.read_to_string(&mut s)?; // another one here
            Ok(s) // Wrappoing the value in a variant of result
        }
        let a = quick_opener("exists.txt");
        dbg!(a);
        let b = quick_opener("non-existing.txt");
        dbg!(b);
        fn fastest_opener(file: &str) -> Result<String, io::Error> {
            fs::read_to_string(file)
        }
        let ax = fastest_opener("exists.txt");
        dbg!(ax);
        let bx = fastest_opener("non-existing.txt");
        dbg!(bx);
    }

    // malformed JSON, unwrap, expect and dbg!
    {
        use serde_json::json;
        use std::error::Error;
        let invalid_json = r#"
        {
            "key": "v
        }"#;

        //example showing unwrap vs expect:
        /*
        let json_serialized: serde_json::Value = serde_json::from_str(&invalid_json).unwrap();
        The above results in the following:

        thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("control character (\\u0000-\\u001F) found while parsing a string", line: 4, column: 0)'
        */
        /*
        let json_serialized: serde_json::Value =
            serde_json::from_str(&invalid_json).expect("unable to deserialize JSON");
            The above results in the following:

        thread 'main' panicked at 'unable to deserialize JSON: Error("control character (\\u0000-\\u001F) found while parsing a string", line: 4, column: 0)', src\main.rs:264:49
        */
        // Examples that use functions
        let valid_json = r#"
        {
            "key": "value"
        }"#;
        fn de_serialize(s: &str) -> Result<serde_json::Value, serde_json::Error> {
            serde_json::from_str(&s)
        }
        let a = de_serialize(invalid_json);
        let b = de_serialize(valid_json);
        dbg!(a);
        dbg!(b);
        // handle with a generic box
        fn de_serialize_generic_error(s: &str) -> Result<serde_json::Value, Box<Error>> {
            let ret = serde_json::from_str(&s)?;
            Ok(ret)
        }

        //
        fn de_serialize_w_generic_error(s: &str) -> Result<serde_json::Value, Box<dyn Error>> {
            let ret = serde_json::from_str(&s)?;
            Ok(ret)
        }
    }
}
