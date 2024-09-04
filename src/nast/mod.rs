// Normalized AST
//
// This is a restriction of the raw AST. Expressions are flattened into basic expressions that
// cannot contain calls or `fby` operators. Instead they can contain `Atom::Ident` that reference
// the result of other equations.

mod atom;
mod bexpr;
mod expr;
//mod node;

pub use atom::Atom;
pub use bexpr::Bexpr;
pub use expr::Expr;
//pub use node::Node;
