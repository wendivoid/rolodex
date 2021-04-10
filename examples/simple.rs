extern crate rolodex;

use rolodex::*;
use std::io::Read;
use std::fs::File;
use std::env::args;

fn print_help_message() {
    println!("Parse VCard Example:\n\tvalidate files...\n")
}

fn panic_error(input: &str, err: nom::Err<ParseError>) {
    match err {
        nom::Err::Incomplete(size) => panic!("Expected More Data: {:?}", size),
        nom::Err::Failure(err) => panic!("{}", err.display(input)),
        nom::Err::Error(err) => panic!("{}", err.display(input))
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        for arg in &args[1..] {
            let mut file = File::open(arg).expect(&format!("Failed to open `{}`", arg));
            let mut raw_data = vec![];
            file.read_to_end(&mut raw_data).expect(&format!("Failed to read file `{}`", arg));
            let data = String::from_utf8_lossy(&raw_data);
            match VCard::parse(&data) {
                Err(err) => panic_error(&data, err),
                Ok(item) => println!("{}", item)
            }
        }
    } else {
        print_help_message()
    }
}
