/// ----------------------------------------------
/// Grammar definition for Nystrom's Lox language.

// expression → literal | unary | binary | grouping ;
pub(crate)  enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    // grouping → "(" expression ")" ;
    Grouping(Box<Expression>),
}

// literal → NUMBER | STRING | "true" | "false" | "nil" ;
pub(crate) enum Literal {
    Number(String),
    String(String),
    //
    True,
    False,
    Nil,
}

// unary → ( "-" | "!" ) expression ;
pub(crate) struct Unary {
    pub operator: UnaryOp,
    pub expr: Box<Expression>,
}

pub(crate) enum UnaryOp {
    Minus,
    Bang,
}

// binary → expression operator expression ;
pub(crate) struct Binary {
    pub left: Box<Expression>,
    pub operator: BinaryOp,
    pub right: Box<Expression>,
}

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

// TODO: Implement.
