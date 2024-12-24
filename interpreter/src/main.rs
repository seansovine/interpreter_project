/// Interpreter project.
///
/// This will be a basic interpreter for a
/// simple language, for the purpose of
/// learning, mostly about parsers.
///
/// Created by sean on 12/18/2024.
///
mod parser;

use std::env;
use std::error::Error;
use std::fs::File;

use crate::parser::scanner::Scanner;
use crate::parser::FileUtf8Reader;

fn read_file(file: File) -> Result<(), Box<dyn Error>> {
    println!("Reading file one char at a time:");
    let reader = FileUtf8Reader::new(file);

    for c in reader {
        println!("'{c}'");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world from the future parser!");

    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.into_os_string().into_string().unwrap();
    println!("Current directory is: {current_dir_str}");

    let args: Vec<String> = env::args().collect();
    println!("Command line args are: {}\n", format!("{:?}", args));

    if cfg!(feature = "test_file_reader") {
        // Test our FileUtf8Reader.
        let file = File::open(&args[1]).unwrap();
        read_file(file).unwrap();
    }

    if cfg!(feature = "test_scanner") {
        // Test our scanner.
        let file = File::open(&args[1]).unwrap();
        let mut scanner = Scanner::new(file);

        scanner.scan_tokens();
        let tokens = scanner.tokens;

        println!("Recognized tokens are: {:?}", tokens);
    }

    // TODO: Add code to test parser here.

    Ok(())
}
