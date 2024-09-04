use crate::nast::{Atom, Bexpr};

#[derive(Debug, Clone)]
pub enum Expr {
    Bexpr(Bexpr),
    Call { name: String, args: Vec<Bexpr> },
    Fby(Vec<Atom>, Vec<Bexpr>),
}
