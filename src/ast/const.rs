use crate::ast::Type;
use crate::parser::{Pair, Rule};

use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Const {
    #[must_use]
    pub fn as_type(&self) -> Type {
        match self {
            Self::Unit => Type::Unit,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Const {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let pair_str = pair.as_str();
        match pair.as_rule() {
            Rule::unit => Ok(Self::Unit),
            Rule::bool => Ok(Self::Bool(pair_str.parse()?)),
            Rule::int => Ok(Self::Int(pair_str.parse()?)),
            Rule::float => Ok(Self::Float(pair_str.parse()?)),
            Rule::string => Ok(Self::String(pair_str.to_string())),
            _ => unimplemented!(),
        }
    }
}

impl Display for Const {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            // Need to always use a dot for Rust to understand it's a float constant
            Self::Float(float) => write!(f, "{float:?}"),
            // use debug to format the string with quotation marks TODO: escaping
            Self::String(s) => write!(f, "{s:?}"),
        }
    }
}
