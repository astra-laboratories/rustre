use crate::ast::Expr;
use crate::parser::{Pair, Rule};

use anyhow::{anyhow, ensure};

#[derive(Debug, Clone)]
pub struct Equation {
    pub names: Vec<String>,
    pub body: Expr,
}

impl TryFrom<Pair<'_, Rule>> for Equation {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::eq, "expected equation rule");
        let mut inner = pair.into_inner();
        let names = inner
            .next()
            .ok_or(anyhow!("expected next pair"))?
            .into_inner()
            .map(|pair| pair.as_str().to_string())
            .collect();
        let body = Expr::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
        Ok(Self { names, body })
    }
}

#[derive(Debug, Clone)]
pub struct List(pub Vec<Equation>);

impl TryFrom<Pair<'_, Rule>> for List {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(
            pair.as_rule() == Rule::eq_list,
            "expected equation list rule"
        );
        let mut eqs = Vec::new();
        for pair in pair.into_inner() {
            eqs.push(Equation::try_from(pair)?);
        }
        Ok(Self(eqs))
    }
}
