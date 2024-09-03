use crate::ast::{Binop, Const, Unop};
use crate::parser::{Pair, Rule};
use crate::{inner, next, next_string};

use anyhow::bail;

#[derive(Debug, Clone, PartialEq)]
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

impl Expr {
    /// Attempts to parse a call.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_call(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut inner = pair.into_inner();
        let name = next_string!(inner);
        let mut args = Vec::<Expr>::new();
        for pair in inner {
            args.push(Self::try_from(pair)?);
        }
        Ok(Self::Call { name, args })
    }

    /// Attempts to parse a const expression.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_const(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        Ok(Self::Const(Const::try_from(inner!(pair))?))
    }

    /// Attempts to parse an unop expression.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_unop(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut inner = pair.into_inner();
        let unop = Unop::try_from(next!(inner))?;
        let expr = Self::try_from(next!(inner))?;
        Ok(Self::Unop(unop, Box::new(expr)))
    }

    /// Attempts to parse an if-else expression.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_if(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut inner = pair.into_inner();
        let condition = Self::try_from(next!(inner))?;
        let if_body = Self::try_from(next!(inner))?;
        let else_body = Self::try_from(next!(inner))?;
        Ok(Self::If {
            condition: Box::new(condition),
            if_body: Box::new(if_body),
            else_body: Box::new(else_body),
        })
    }

    /// Attempts to parse an identifier, like `a` or `b`.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    #[allow(clippy::needless_pass_by_value)] // just to keep the API unified
    pub fn parse_ident(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        Ok(Self::Ident(pair.as_str().to_string()))
    }

    /// Attempts to parse a nested expression between parentheses.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_pexpr(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        Self::try_from(inner!(pair))
    }

    /// Attempts to parse a tuple.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_tuple(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut exprs = Vec::new();
        for pair in pair.into_inner() {
            exprs.push(Self::try_from(pair)?);
        }
        Ok(Self::Tuple(exprs))
    }

    /// Attempts to parse a generic or binop/fby expression.
    ///
    /// # Errors
    ///
    /// Throws an error if parsing fails.
    pub fn parse_expr(pair: Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut inner = pair.into_inner();
        let lhs = Self::try_from(next!(inner))?;
        if let Some(binop_pair) = inner.next() {
            let binop = Binop::try_from(binop_pair)?;
            let rhs = Self::try_from(next!(inner))?;
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
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::call => Self::parse_call(pair),
            Rule::constant => Self::parse_const(pair),
            Rule::unop_expr => Self::parse_unop(pair),
            Rule::ifrule => Self::parse_if(pair),
            Rule::ident => Self::parse_ident(pair),
            Rule::pexpr => Self::parse_pexpr(pair),
            Rule::expr_tuple => Self::parse_tuple(pair),
            Rule::expr => Self::parse_expr(pair),
            invalid => bail!("invalid expr pair {invalid:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Const, Unop};
    use crate::next;
    use crate::parser::{Lustre, Parser};

    #[test]
    fn parse_call() {
        let input = "print(\"hello\n\")";
        let mut pair = Lustre::parse(Rule::call, input).unwrap();
        assert_eq!(
            Expr::parse_call(next!(pair, "uw")).unwrap(),
            Expr::Call {
                name: "print".to_string(),
                args: vec![Expr::Const(Const::String("\"hello\n\"".to_string()))],
            }
        );
        let input = "foo(false, (0.15, -2), quux(a, b))";
        let mut pair = Lustre::parse(Rule::call, input).unwrap();
        assert_eq!(
            Expr::parse_call(next!(pair, "uw")).unwrap(),
            Expr::Call {
                name: "foo".to_string(),
                args: vec![
                    Expr::Const(Const::Bool(false)),
                    Expr::Tuple(vec![
                        Expr::Const(Const::Float(0.15)),
                        Expr::Unop(Unop::Minus, Box::new(Expr::Const(Const::Int(2))))
                    ]),
                    Expr::Call {
                        name: "quux".to_string(),
                        args: vec![Expr::Ident("a".to_string()), Expr::Ident("b".to_string())],
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_if() {
        // TODO why can't this be parsed??
        let input = "if initialized then print(\"hello\") else;";
        let mut pair = Lustre::parse(Rule::ifrule, input).unwrap();
        assert_eq!(
            Expr::parse_if(next!(pair, "uw")).unwrap(),
            Expr::If {
                condition: Box::new(Expr::Const(Const::Bool(true))),
                if_body: Box::new(Expr::Call {
                    name: "print".to_string(),
                    args: vec![Expr::Const(Const::String("\"hello\"".to_string()))],
                }),
                else_body: Box::new(Expr::Const(Const::Unit)),
            }
        );

        let input = r#"if x < pmin then (x, pmax)
            else if x > pmax then (pmin, x)
            else (pmin, pmax);"#;
        let mut pair = Lustre::parse(Rule::ifrule, input).unwrap();
        assert_eq!(
            Expr::parse_if(next!(pair, "uw")).unwrap(),
            Expr::If {
                condition: Box::new(Expr::Binop(
                    Binop::Lt,
                    Box::new(Expr::Ident("x".to_string())),
                    Box::new(Expr::Ident("pmin".to_string()))
                )),
                if_body: Box::new(Expr::Tuple(vec![
                    Expr::Ident("x".to_string()),
                    Expr::Ident("pmax".to_string())
                ])),
                else_body: Box::new(Expr::If {
                    condition: Box::new(Expr::Binop(
                        Binop::Gt,
                        Box::new(Expr::Ident("x".to_string())),
                        Box::new(Expr::Ident("pmax".to_string()))
                    )),
                    if_body: Box::new(Expr::Tuple(vec![
                        Expr::Ident("pmin".to_string()),
                        Expr::Ident("x".to_string())
                    ])),
                    else_body: Box::new(Expr::Tuple(vec![
                        Expr::Ident("pmin".to_string()),
                        Expr::Ident("pmax".to_string())
                    ])),
                }),
            }
        );
    }
}
