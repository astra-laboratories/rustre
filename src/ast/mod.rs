//! Raw AST
//!
//! This is a 1:1 representation of Lustre source files.
//!
//! Dot operators can be applied to floats (non-dot operators can be applied to integers).

mod arg;
mod binop;
mod r#const;
mod equation;
mod expr;
mod node;
mod r#type;
mod unop;

pub use arg::{Arg, List as ArgList, Local};
pub use binop::Binop;
pub use equation::{Equation, List as EquationList};
pub use expr::Expr;
pub use node::{List as NodeList, Node};
pub use r#const::Const;
pub use r#type::Type;
pub use unop::Unop;

use crate::parser::{Lustre, Parser, Rule};

use anyhow::anyhow;

use std::str::FromStr;

pub struct Ast(pub Vec<Node>);

impl FromStr for Ast {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = Lustre::parse(Rule::file, s)?;
        let nodes = NodeList::try_from(pair.next().ok_or(anyhow!("expected next pair"))?)?;
        Ok(Self(nodes.into_inner()))
    }
}
