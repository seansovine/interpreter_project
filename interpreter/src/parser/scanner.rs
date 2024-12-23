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
    // Operators, potentially two-token.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    // Division operator
    Slash,
    // String containing text value.
    String(String),
    // Special token to aid parser.
    EOF,
}

pub struct Scanner {
    pub tokens: Vec<Token>,

    reader: FileUtf8Reader,

    current_char: Option<char>,
    next_char: Option<char>,
    third_char: Option<char>,

    current_line: usize,
}

impl Scanner {
    pub fn new(file: File) -> Scanner {
        let reader = FileUtf8Reader::new(file);
        let mut scanner = Scanner {
            tokens: vec![],
            reader,
            current_char: None,
            next_char: None,
            third_char: None,
            current_line: 1,
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

                // Operators requiring one lookahead.
                '!' => {
                    if self.next_char == Some('=') {
                        self.add_token(Token::BangEqual);
                    } else {
                        self.add_token(Token::Bang);
                    }
                }
                '=' => {
                    if self.next_char == Some('=') {
                        self.add_token(Token::EqualEqual);
                    } else {
                        self.add_token(Token::Equal);
                    }
                }
                '<' => {
                    if self.next_char == Some('=') {
                        self.add_token(Token::LessEqual);
                    } else {
                        self.add_token(Token::Less);
                    }
                }
                '>' => {
                    if self.next_char == Some('=') {
                        self.add_token(Token::GreaterEqual);
                    } else {
                        self.add_token(Token::Greater);
                    }
                }

                // Either start of comment or division operator.
                '/' => {
                    if self.next_char == Some('/') {
                        self.consume_rest_of_line();
                    } else {
                        self.add_token(Token::Slash);
                    }
                }

                // String literal.
                '"' => {
                    if let Some(text) = self.get_string_literal() {
                        self.add_token(Token::String(text));
                    } else {
                        // TODO: Report unterminated string literal.
                    }
                }

                // We just ignore whitespace.
                ' ' | '\t' | '\r' => {}
                // Later we may want to track line numbers.
                '\n' => {
                    self.current_line += 1;
                }

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

    fn consume_rest_of_line(&mut self) {
        while !self.is_at_end() && self.current_char != Some('\n') {
            self.advance();
        }
    }

    fn get_string_literal(&mut self) -> Option<String> {
        // We are still on the starting '"'.
        self.advance();

        let mut string = String::new();
        while self.current_char != Some('"') && !self.is_at_end() {
            string.push(self.current_char.unwrap());
            self.advance();
        }

        if self.current_char == Some('"') {
            Some(string)
        } else {
            None
        }

        // NOTE: We are still on the last quote now.
    }
}
