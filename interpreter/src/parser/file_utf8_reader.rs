use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

/// This just needs to be long enough to
/// hold the longest utf-8 codepoint.
const BUFFER_SIZE: usize = 12;

/// Wraps BufReader<File> and allows us to read a file
/// one utf-8 character at a time, without loading the
/// entire file into memory at once.
pub struct FileUtf8Reader {
    reader: BufReader<File>,
}

impl FileUtf8Reader {
    pub fn new(file: File) -> FileUtf8Reader {
        FileUtf8Reader {
            reader: BufReader::with_capacity(BUFFER_SIZE, file),
        }
    }
}

impl Iterator for FileUtf8Reader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // The idea to use fill_buf for this comes from:
        //  https://stackoverflow.com/questions/37079342/

        let buffer = self.reader.fill_buf().unwrap();

        if buffer.is_empty() {
            return None;
        }

        let char_len = UTF8_CHAR_WIDTH[buffer[0] as usize];
        let char_bytes = &buffer[0..char_len];

        let char_str = str::from_utf8(char_bytes).unwrap();
        let the_char = char_str.chars().nth(0).unwrap();

        self.reader.consume(char_len);

        Some(the_char)
    }
}

// Copied from src/core/str/validations.rs in stdlib.
// https://tools.ietf.org/html/rfc3629
const UTF8_CHAR_WIDTH: &[usize; 256] = &[
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];
