use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

/*

Example searching the text string:

    cargo run -- ^A

Example searching the text in a file"


*/
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("search")
        .version("0.1")
        .about("search for a pattern")
        .arg(
            Arg::with_name("pattern")
                .help("the pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("Target file to open and search")
                .takes_value(true)
                .require_delimiter(false),
        )
        .get_matches();
    let text = "\
Some text. Some words.
Some more words.😎
A haystack. Some other things. And also, a needle.
Can we find it?";

    let target_word = "needle";
    let input = args.value_of("input").unwrap_or("-");
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    if input == "-" {
        for line in text.lines() {
            if line.contains(target_word) {
                println!("{}", line);
            }
        }
        // with line number:
        for (i, line) in text.lines().enumerate() {
            if line.contains(target_word) {
                println!("{} {}", i, line);
            }
        }

        for line in text.lines() {
            let contains_re = re.find(line);
            match contains_re {
                Some(_) => {
                    println!("re found this: {}", line);
                }
                None => (),
            }
        }
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
