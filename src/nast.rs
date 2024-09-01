// Normalized AST
//
// This is a restriction of the raw AST. Expressions are flattened into basic expressions that
// cannot contain calls or `fby` operators. Instead they can contain `Atom::Ident` that reference
// the result of other equations.

use crate::ast::{Binop, Const, Type, Unop};

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Atom {
    Ident(String),
    Const(Const),
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => write!(f, "{c}"),
            Self::Ident(ident) => write!(f, "{ident}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Bexpr {
    Atom(Atom),
    Unop(Unop, Box<Bexpr>),
    Binop(Binop, Box<Bexpr>, Box<Bexpr>),
    IfElse(Box<Bexpr>, Box<Bexpr>, Option<Box<Bexpr>>),
    Tuple(Vec<Bexpr>),
}

impl Display for Bexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{atom}"),
            Self::Unop(op, expr) => write!(f, "{op} {expr}"),
            Self::Binop(op, lhs, rhs) => write!(f, "{lhs} {op} {rhs}"),
            Self::IfElse(condition, if_body, maybe_else_body) => {
                write!(f, "if {condition} {{ {if_body} }}")?;
                if let Some(else_body) = maybe_else_body {
                    write!(f, "else {{ {if_body} }}")?;
                }
                Ok(())
            }
            Self::Tuple(elems) => write!(
                f,
                "({})",
                elems
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Bexpr(Bexpr),
    Call { name: String, args: Vec<Bexpr> },
    Fby(Vec<Atom>, Vec<Bexpr>),
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub names: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args_in: HashMap<String, Type>,
    pub args_out: HashMap<String, Type>,
    pub locals: HashMap<String, Type>,
    pub body: Vec<Equation>,
}
