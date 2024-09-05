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

pub use arg::{Arg, List as ArgList, Locals};
pub use binop::Binop;
pub use equation::{Equation, List as EquationList};
pub use expr::Expr;
pub use node::{List as NodeList, Node};
pub use r#const::Const;
pub use r#type::Type;
pub use unop::Unop;

use crate::next;
use crate::parser::{Lustre, Parser, Rule};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Node>);

impl FromStr for Ast {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = Lustre::parse(Rule::file, s)?;
        let nodes = NodeList::try_from(next!(pair))?;
        Ok(Self(nodes.into_inner()))
    }
}

impl Ast {
    /// Attempts to read the contents of a Lustre file into an AST.
    ///
    /// # Errors
    ///
    /// Throws an error if the file cannot be opened or its contents cannot be read.
    pub fn read(path: PathBuf) -> Result<Self, anyhow::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::from_str(&contents)
    }

    pub fn normalize(&mut self) {
        self.0.iter_mut().for_each(Node::normalize);
    }

    pub fn order(&mut self) {
        self.0.iter_mut().for_each(Node::order);
    }
}
