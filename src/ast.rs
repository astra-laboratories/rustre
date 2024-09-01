// Raw AST
//
// This is a 1:1 representation of Lustre source files.
//
// Dot operators can be applied to floats (non-dot operators can be applied to integers).

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    String,
    Tuple(Vec<Type>),
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

#[derive(Debug, Clone)]
pub enum Const {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
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

/// Unary operators.
#[derive(Debug, Clone, Copy)]
pub enum Unop {
    Minus,
    MinusDot,
    Not,
}

impl Unop {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Minus | Self::MinusDot => "-",
            Self::Not => "!",
        }
    }
}

impl Display for Unop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Binary operators.
#[derive(Debug, Clone, Copy)]
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
}

impl Binop {
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
        }
    }
}

impl Display for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Const(Const),
    Unop(Unop, Box<Expr>),
    Binop(Binop, Box<(Expr, Expr)>),
    /// Yields an initial value followed by an expression
    Fby(Box<(Expr, Expr)>),
    If(Box<(Expr, Expr, Expr)>),
    /// Reference to the result of another equation.
    Ident(String),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub names: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args_in: HashMap<String, Type>,
    pub args_out: HashMap<String, Type>,
    pub locals: HashMap<String, Type>,
    pub body: Vec<Equation>,
}
