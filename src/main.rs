extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod nast;
mod normalizer;
mod parser;
mod rustfmt;
mod sequentializer;
mod typer;

use crate::normalizer::normalize;
use crate::parser::parse;
use crate::rustfmt::format;
use crate::sequentializer::sequentialize;

use structopt::StructOpt;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    src: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(opt.src).expect("invalid file path");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("couldn't read file");
    println!("file contents read: {:?}", contents);

    let parsed = parse(&contents).unwrap();
    println!("parsed: {:?}", &parsed);

    let normalized = normalize(&parsed);
    println!("normalized: {:?}", &normalized);

    let sequentialized = sequentialize(&normalized);
    println!("sequentialized: {:?}", &sequentialized);
}
