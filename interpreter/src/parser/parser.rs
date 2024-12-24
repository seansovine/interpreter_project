/// Implementing Nystrom's basic parser.
///
///
use crate::parser::scanner::Token;

// -------------------
// Grammar definition.

// expression → literal | unary | binary | grouping ;
enum Expression {
    Literal(Literal),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    // grouping → "(" expression ")" ;
    Grouping(Box<Expression>),
}

// literal → NUMBER | STRING | "true" | "false" | "nil" ;
enum Literal {
    Number(String),
    String(String),
    //
    True,
    False,
    Nil,
}

// unary → ( "-" | "!" ) expression ;
struct Unary {
    operator: UnaryOp,
    expr: Box<Expression>,
}

enum UnaryOp {
    Minus,
    Bang,
}

// binary → expression operator expression ;
struct Binary {
    left: Box<Expression>,
    operator: BinaryOp,
    right: Box<Expression>,
}

enum BinaryOp {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

// ----------------------
// Parser implementation.

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

/// BIG NOTE: This is Work in Progress!
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let pred = |token: &Token| match token {
            Token::EqualEqual => true,
            Token::BangEqual => true,
            _ => false,
        };

        // TODO: Call comparison here instead.
        let expr = Expression::Literal(Literal::Nil);

        while self.match_token(pred) {
            let operator = self.previous();
            // TODO: Call comparison again for RHS.
            // Update expression to new binary from current.
        }

        expr
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
