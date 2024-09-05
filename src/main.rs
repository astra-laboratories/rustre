#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::dbg_macro)]
#![deny(unused_crate_dependencies)]

pub mod ast;
pub mod codegen;
pub mod normalizer;
pub mod parser;
pub mod sequentializer;

use crate::ast::Ast;

use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    src: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut ast = Ast::build(opt.src).expect("invalid lustre input");
    println!("RAW\n\n{ast:#?}");

    ast.normalize();
    println!("NRM\n\n{ast:#?}");

    ast.order();
    println!("ORD\n\n{ast:#?}");

    println!("RUST\n\n{:#?}", ast.codegen());
}

#[macro_export]
macro_rules! next {
    ($pair:expr, "uw") => {
        $pair.next().unwrap()
    };
    ($pair:expr, "ok") => {
        $pair.next().ok_or(anyhow::anyhow!("expected next pair"))?
    };
    ($pair:expr) => {
        next!($pair, "ok")
    };
}

#[macro_export]
macro_rules! next_string {
    ($pair:expr) => {
        next!($pair).as_str().to_string()
    };
}

#[macro_export]
macro_rules! inner {
    ($pair:expr) => {
        next!($pair.into_inner())
    };
}
