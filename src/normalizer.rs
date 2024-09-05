//! Normalization transforms a raw AST into a normalized AST.
//!
//! This is done by adding new intermediate equations for nested calls and `fby` operations. For
//! instance the following Lustre code which represents two nested calls to `f`:
//!
//! ```lustre
//! expr = f(f(42));
//! ```
//!
//! is parsed as:
//!
//! ```rust
//! expr = Expr::Call{
//!     name: "f",
//!     args: vec![Expr::Call{
//!         name: "f",
//!         args: vec![Expr::Const(Const::Int(42))],
//!     }],
//! };
//! ```
//!
//! and is normalized into these two equations:
//!
//! ```rust
//! tmp1 = Expr::Call{
//!     name: "f",
//!     args: vec![Bexpr::Atom(Atom::Const(Const::Int(42)))],
//! };
//! expr = Expr::Call{
//!     name: "f",
//!     args: vec![Bexpr::Atom(Atom::Ident("tmp1"))],
//! };
//! ```

use crate::ast::Expr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub struct Normalizer {
    pub counter: AtomicUsize,
    pub memory: HashMap<String, Expr>,
}

impl Normalizer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn next_tmp(&self) -> String {
        let i = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("tmp_{i}")
    }
}
