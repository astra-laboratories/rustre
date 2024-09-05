use crate::nast::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct Equation {
    pub names: Vec<String>,
    pub body: Expr,
}

