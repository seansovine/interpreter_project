/// Implementing Nystrom's Lox expression grammar.
///
///
use crate::parser::Parser;

/// ----------------------------------------------
/// Grammar definition for Nystrom's Lox language.

#[derive(Debug)]
// expression → literal | unary | binary | grouping ;
pub(crate) enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    // grouping → "(" expression ")" ;
    Grouping(Box<Expression>),
}

#[derive(Debug)]
// literal → NUMBER | STRING | "true" | "false" | "nil" ;
pub(crate) enum Literal {
    Number(String),
    String(String),
    //
    True,
    False,
    Nil,
}

#[derive(Debug)]
// unary → ( "-" | "!" ) expression ;
pub(crate) struct Unary {
    pub operator: UnaryOp,
    pub expr: Box<Expression>,
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug)]
// binary → expression operator expression ;
pub(crate) struct Binary {
    pub left: Box<Expression>,
    pub operator: BinaryOp,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
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

// -----------------------------
// Visitor to pretty print ASTs.

// We use the visitor pattern as in Nystrom's ch. 5.

trait Visitor<T> {
    fn visit_literal(&mut self, expression: &Literal) -> T;
    fn visit_unary(&mut self, expression: &Unary) -> T;
    fn visit_binary(&mut self, expression: &Binary) -> T;

    /// NOTE: Maybe we can find a better way to do this.
    fn visit_grouping(&mut self, expression: &Expression) -> T {
        match expression {
            Expression::Grouping(inner) => self._visit_grouping_impl(inner),

            _ => panic!("Not a grouping expression."),
        }
    }
    fn _visit_grouping_impl(&mut self, expression: &Expression) -> T;
}

// Define accept methods on types.

impl Literal {
    fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

impl Unary {
    fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}

impl Binary {
    fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

impl Expression {
    fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Binary(binary) => binary.accept(visitor),

            // NOTE: This is different because we made the Grouping variant
            // directly hold its inner expression, rather than a struct.
            Expression::Grouping(_) => visitor.visit_grouping(self),

            // NOTE: Currently unreachable, but it is useful for
            // debugging when we add new expression types.
            _ => panic!(
                "Accept not implemented for this expression variant: {:?}.",
                self
            ),
        }
    }
}

// Define a visitor for pretty printing.

struct PrettyPrintVisitor;

impl Visitor<String> for PrettyPrintVisitor {
    fn visit_literal(&mut self, expression: &Literal) -> String {
        let rep: String;
        match expression {
            Literal::Number(string) => rep = string.clone(),
            Literal::String(string) => rep = string.clone(),

            Literal::True => rep = String::from("true"),
            Literal::False => rep = String::from("false"),
            Literal::Nil => rep = String::from("nil"),
        }

        rep
    }

    fn visit_unary(&mut self, expression: &Unary) -> String {
        let op: String = match expression.operator {
            UnaryOp::Bang => String::from("!"),
            UnaryOp::Minus => String::from("-"),
        };

        format!("({} {})", op, expression.accept(self))
    }

    fn visit_binary(&mut self, expression: &Binary) -> String {
        let op: String = match expression.operator {
            BinaryOp::EqualEqual => String::from("=="),
            BinaryOp::BangEqual => String::from("!="),
            BinaryOp::Less => String::from("<"),
            BinaryOp::LessEqual => String::from("<="),
            BinaryOp::Greater => String::from(">"),
            BinaryOp::GreaterEqual => String::from(">="),
            BinaryOp::Plus => String::from("+"),
            BinaryOp::Minus => String::from("-"),
            BinaryOp::Star => String::from("*"),
            BinaryOp::Slash => String::from("/"),
        };

        format!(
            "({op} {} {})",
            expression.left.accept(self),
            expression.right.accept(self)
        )
    }

    fn _visit_grouping_impl(&mut self, expression: &Expression) -> String {
        format!("(group {})", expression.accept(self))
    }
}

// Add pretty print method to parser.

impl Parser {
    pub fn pretty_print(&self) -> String {
        let mut printer = PrettyPrintVisitor {};
        if let Some(expression) = self.root.as_ref() {
            expression.accept(&mut printer)
        } else {
            String::from("Null Expression")
        }
    }
}
