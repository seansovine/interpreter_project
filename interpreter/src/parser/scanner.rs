/// First we will implement a version of the lexer from Robert
/// Nystrom's *Crafting An Interpreter*. Then we'll modify that
/// to suit our own language, which is a work in progress.
///
/// Created by sean on 12/22/2024.
///
use crate::parser::FileUtf8Reader;

use std::fs::File;

#[derive(Debug)]
pub enum Token {
    // Single-char tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Star,
    // Special token to aid parser.
    EOF,
}

pub struct Scanner {
    pub tokens: Vec<Token>,

    reader: FileUtf8Reader,

    current_char: Option<char>,
    next_char: Option<char>,
    third_char: Option<char>,
}

impl Scanner {
    pub fn new(file: File) -> Scanner {
        let reader = FileUtf8Reader::new(file);
        let mut scanner = Scanner {
            reader,
            tokens: vec![],
            current_char: None,
            next_char: None,
            third_char: None,
        };

        scanner.current_char = scanner.reader.next();
        scanner.next_char = scanner.reader.next();
        scanner.third_char = scanner.reader.next();

        scanner
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token()
        }

        self.add_token(Token::EOF);
    }

    fn scan_token(&mut self) {
        match self.current_char {
            None => panic!("Should not attempt token scan with no remaining characters."),
            Some(c) => match c {
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                '{' => self.add_token(Token::LeftBrace),
                '}' => self.add_token(Token::RightBrace),
                ',' => self.add_token(Token::Comma),
                '.' => self.add_token(Token::Period),
                '-' => self.add_token(Token::Minus),
                '+' => self.add_token(Token::Plus),
                ';' => self.add_token(Token::Semicolon),
                '*' => self.add_token(Token::Star),

                // We just ignore whitespace.
                ' ' | '\t' => {}
                // Later we may want to track line numbers.
                '\n' => {}

                _ => {
                    // TODO: Handle unrecognized tokens.
                }
            },
        }

        self.advance();
    }

    /// This fn exists to avoid duplicating the push call for each token,
    /// And it will be useful if we start recording additional information.
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn advance(&mut self) {
        self.current_char = self.next_char;
        self.next_char = self.third_char;
        self.third_char = self.reader.next();
    }

    fn is_at_end(&self) -> bool {
        self.current_char.is_none()
    }
}
