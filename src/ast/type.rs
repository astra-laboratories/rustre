use crate::parser::{Pair, Rule};

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    String,
    Tuple(Vec<Type>),
}

impl FromStr for Type {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unit" => Ok(Self::Unit),
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            invalid => anyhow::bail!("unimplemented type {invalid}"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "i32"),
            Self::Float => write!(f, "f32"),
            Self::String => write!(f, "String"),
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

impl TryFrom<Pair<'_, Rule>> for Type {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Self::from_str(pair.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Type::Unit.to_string(), "()");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::Int.to_string(), "i32");
        assert_eq!(Type::Float.to_string(), "f32");
        assert_eq!(Type::String.to_string(), "String");
        assert_eq!(
            Type::Tuple(vec![
                Type::Unit,
                Type::String,
                Type::Tuple(vec![Type::Float, Type::Int])
            ])
            .to_string(),
            "((),String,(f32,i32))"
        );
    }
}
