use crate::ast::{Binop, Const, Unop};

#[derive(Debug, Clone)]
pub enum Expr {
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Const(Const),
    Unop(Unop, Box<Expr>),
    Binop(Binop, Box<Expr>, Box<Expr>),
    /// Yields an initial value followed by an expression
    Fby(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// Reference to the result of another equation.
    Ident(String),
    Tuple(Vec<Expr>),
}
