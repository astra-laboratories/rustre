use crate::parser::{Pair, Rule};

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Unary operators.
#[derive(Debug, Clone, Copy)]
pub enum Unop {
    Minus,
    MinusDot,
    Not,
}

impl Unop {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Minus | Self::MinusDot => "-",
            Self::Not => "!",
        }
    }
}

impl FromStr for Unop {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Minus),
            "-." => Ok(Self::MinusDot),
            "not" => Ok(Self::Not),
            invalid => anyhow::bail!("invalid unop {invalid}"),
        }
    }
}

impl Display for Unop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<Pair<'_, Rule>> for Unop {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        anyhow::ensure!(pair.as_rule() == Rule::unop, "expected unop pair");
        Self::from_str(pair.as_str())
    }
}
