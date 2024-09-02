use crate::ast::{Binop, Const, Unop};
use crate::parser::{try_inner_pair, Pair, Rule};

use anyhow::{anyhow, bail};

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

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::call => {
                todo!()
            }
            Rule::constant => Ok(Self::Const(Const::try_from(try_inner_pair(pair)?)?)),
            Rule::unop_expr => {
                todo!()
            }
            Rule::ifrule => {
                todo!()
            }
            Rule::ident => Ok(Self::Ident(pair.as_str().to_string())),
            Rule::pexpr => Self::try_from(try_inner_pair(pair)?),
            Rule::expr_tuple => {
                let mut exprs = Vec::new();
                for pair in pair.into_inner() {
                    exprs.push(Self::try_from(pair)?);
                }
                Ok(Self::Tuple(exprs))
            }
            Rule::expr => {
                todo!()
            }
            invalid => bail!("invalid expr pair {invalid:?}"),
        }
    }
}
