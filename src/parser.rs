pub use pest::iterators::{Pair, Pairs};
pub use pest::Parser;
use pest_derive::Parser as ParserT;

#[derive(ParserT)]
#[grammar = "../pest/mlustre.pest"]
pub struct Lustre;
