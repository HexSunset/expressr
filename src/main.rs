use nom::{IResult, };

fn main() {}

enum Expression {
    Const(f64),

    Var(char),

    Plus {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

enum Token {
    Const(f64),
    Var(char),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    OpenParen,
    CloseParen,
}
