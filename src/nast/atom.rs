use crate::ast::Const;

use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
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
