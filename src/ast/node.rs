use crate::ast::{Equation, Type};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args_in: HashMap<String, Type>,
    pub args_out: HashMap<String, Type>,
    pub locals: HashMap<String, Type>,
    pub body: Vec<Equation>,
}
