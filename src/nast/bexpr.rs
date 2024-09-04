use crate::ast::{Binop, Unop};
use crate::nast::Atom;

use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Bexpr {
    Atom(Atom),
    Unop(Unop, Box<Bexpr>),
    Binop(Binop, Box<Bexpr>, Box<Bexpr>),
    If {
        condition: Box<Bexpr>,
        if_body: Box<Bexpr>,
        else_body: Box<Bexpr>,
    },
    Tuple(Vec<Bexpr>),
}

impl Display for Bexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(atom) => write!(f, "{atom}"),
            Self::Unop(op, expr) => write!(f, "{op} {expr}"),
            Self::Binop(op, lhs, rhs) => write!(f, "{lhs} {op} {rhs}"),
            Self::If {
                condition,
                if_body,
                else_body,
            } => {
                write!(f, "if {condition} {{ {if_body} }} else {{ {else_body} }}")
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
