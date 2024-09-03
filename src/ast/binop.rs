use crate::parser::{Pair, Rule};

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binop {
    Plus,
    Minus,
    Mult,
    Div,
    PlusDot,
    MinusDot,
    MultDot,
    DivDot,
    Lt,
    Gt,
    Leq,
    Geq,
    Eq,
    And,
    Or,
    Fby,
}

impl Binop {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Plus | Self::PlusDot => "+",
            Self::Minus | Self::MinusDot => "-",
            Self::Mult | Self::MultDot => "*",
            Self::Div | Self::DivDot => "/",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Leq => "<=",
            Self::Geq => ">=",
            Self::Eq => "==",
            Self::And => "&&",
            Self::Or => "||",
            Self::Fby => "",
        }
    }

    /*
    pub fn as_rs_str(&self) -> &str {
        // TODO as_str should return from_str
        todo!("do we need this")
    }
    */
}

impl FromStr for Binop {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Binop::Plus),
            "-" => Ok(Binop::Minus),
            "*" => Ok(Binop::Mult),
            "/" => Ok(Binop::Div),
            "+." => Ok(Binop::PlusDot),
            "-." => Ok(Binop::MinusDot),
            "*." => Ok(Binop::MultDot),
            "/." => Ok(Binop::DivDot),
            "<" => Ok(Binop::Lt),
            ">" => Ok(Binop::Gt),
            "<=" => Ok(Binop::Leq),
            ">=" => Ok(Binop::Geq),
            "=" => Ok(Binop::Eq),
            "and" => Ok(Binop::And),
            "or" => Ok(Binop::Or),
            "fby" => Ok(Binop::Fby),
            invalid => anyhow::bail!("invalid binop {invalid}"),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Binop {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Self::from_str(pair.as_str())
    }
}

impl Display for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
