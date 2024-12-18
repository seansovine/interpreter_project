/// Interpreter project.
///
/// This will be a basic interpreter for a
/// simple language, for the purpose of
/// learning, mostly about parsers.
///
/// Created by sean on 12/18/2024.
///
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn read_file(file: File) -> Result<(), Box<dyn Error>> {
    // Read chars of the file.
    // The main idea for this approach comes from:
    //  https://stackoverflow.com/questions/37079342/

    const CAP: usize = 1024 * 128;
    let mut reader = BufReader::with_capacity(CAP, file);

    loop {
        let buffer_bytes = reader.fill_buf()?;
        let buffer = str::from_utf8(buffer_bytes)?;
        let length = buffer.len();

        if length == 0 {
            break;
        }

        for c in buffer.chars() {
            print!("'{}'\n", c);
        }

        reader.consume(length);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world from the future parser!");

    //
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.into_os_string().into_string().unwrap();
    println!("Current directory is: {current_dir_str}");
    //

    let args: Vec<String> = env::args().collect();
    println!("Command line args are: {}\n", format!("{:?}", args));

    let file = File::open(&args[1]).unwrap();

    read_file(file)
}