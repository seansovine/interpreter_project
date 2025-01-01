/// Implementing Nystrom's basic parser.
///
///
use crate::parser::grammar::*;
use crate::parser::scanner::Token;

// ----------------------
// Parser implementation.

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,

    // Root of AST.
    pub root: Option<Expression>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser { tokens, cursor: 0, root: None };
        parser.root = Some(parser.expression());

        parser
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::EqualEqual | Token::BangEqual => true,
            _ => false,
        };

        let mut expr = self.comparison();

        while self.match_token(pred) {
            let operator = match self.previous() {
                Token::EqualEqual => BinaryOp::EqualEqual,
                Token::BangEqual => BinaryOp::BangEqual,
                _ => unreachable!(),
            };
            let right = self.comparison();
            expr = Expression::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => true,
            _ => false,
        };

        let mut expr = self.term();

        while self.match_token(pred) {
            let operator = match self.previous() {
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term();
            expr = Expression::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::Plus | Token::Minus => true,
            _ => false,
        };

        let mut expr = self.factor();

        while self.match_token(pred) {
            let operator = match self.previous() {
                Token::Plus => BinaryOp::Plus,
                Token::Minus => BinaryOp::Minus,
                _ => unreachable!(),
            };
            let right = self.factor();
            expr = Expression::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::Slash | Token::Star => true,
            _ => false,
        };

        let mut expr = self.unary();

        while self.match_token(pred) {
            let operator = match self.previous() {
                Token::Slash => BinaryOp::Slash,
                Token::Star => BinaryOp::Star,
                _ => unreachable!(),
            };
            let right = self.unary();
            expr = Expression::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::Bang | Token::Minus => true,
            _ => false,
        };

        if self.match_token(pred) {
            let operator = match self.previous() {
                Token::Bang => UnaryOp::Bang,
                Token::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            let right = self.unary();
            return Expression::Unary(Unary {
                operator,
                expr: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        if self.cursor >= self.tokens.len() {
            panic!("Out of tokens while parsing expression.");
        }

        // Consume current token.
        let token = &self.tokens[self.cursor];
        self.cursor += 1;

        match token {
            Token::False => Expression::Literal(Literal::False),
            Token::True => Expression::Literal(Literal::True),
            Token::Nil => Expression::Literal(Literal::Nil),

            Token::Number(val) => Expression::Literal(Literal::Number(val.to_owned())),
            Token::String(val) => Expression::Literal(Literal::String(val.to_owned())),

            Token::LeftParen => {
                let expr = self.expression();

                // Next token should be right paren; consume it.
                if self.tokens[self.cursor] != Token::RightParen {
                    panic!("Expected ')'.");
                }
                self.cursor += 1;

                Expression::Grouping(Box::new(expr))
            }

            _ => unreachable!(),
        }
    }

    fn previous(&self) -> &Token {
        if self.cursor < 1 {
            panic!("Previous called at index 0.");
        }

        &self.tokens[self.cursor - 1]
    }

    fn match_token<F>(&mut self, pred: F) -> bool
    where
        F: Fn(&Token) -> bool,
    {
        if self.cursor == self.tokens.len() {
            return false;
        }
        if pred(&self.tokens[self.cursor]) {
            self.cursor += 1;

            true
        } else {
            false
        }
    }
}
