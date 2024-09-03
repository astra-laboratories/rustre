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
    If {
        condition: Box<Expr>,
        if_body: Box<Expr>,
        else_body: Box<Expr>, // TODO should be an option?
    },
    /// Reference to the result of another equation.
    Ident(String),
    Tuple(Vec<Expr>),
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::call => {
                let mut inner = pair.into_inner();
                let name = inner
                    .next()
                    .ok_or(anyhow!("expected next pair"))?
                    .as_str()
                    .to_string();
                let mut args = Vec::<Expr>::new();
                for pair in inner {
                    args.push(Self::try_from(pair)?);
                }
                Ok(Self::Call { name, args })
            }
            Rule::constant => Ok(Self::Const(Const::try_from(try_inner_pair(pair)?)?)),
            Rule::unop_expr => {
                let mut inner = pair.into_inner();
                let unop = Unop::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                let expr = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                Ok(Self::Unop(unop, Box::new(expr)))
            }
            Rule::ifrule => {
                let mut inner = pair.into_inner();
                let condition = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                let if_body = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                let else_body = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                Ok(Self::If {
                    condition: Box::new(condition),
                    if_body: Box::new(if_body),
                    else_body: Box::new(else_body),
                })
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
                let mut inner = pair.into_inner();
                let lhs = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                if let Some(binop_pair) = inner.next() {
                    let binop = Binop::try_from(binop_pair)?;
                    let rhs = Self::try_from(inner.next().ok_or(anyhow!("expected next pair"))?)?;
                    // TODO why is this differentiation needed between Binop and Fby
                    if let Binop::Fby = binop {
                        Ok(Self::Fby(Box::new(lhs), Box::new(rhs)))
                    } else {
                        Ok(Self::Binop(binop, Box::new(lhs), Box::new(rhs)))
                    }
                } else {
                    Ok(lhs)
                }
            }
            invalid => bail!("invalid expr pair {invalid:?}"),
        }
    }
}
