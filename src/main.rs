mod ast;
mod generate;
mod lexer;
mod parser;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Arg {
    #[arg(short)]
    script: String,
}

fn main() {
    let arg = Arg::parse();
    let script = arg.script;
    let token = dbg!(lexer::lexer(&script)).unwrap();
    let ast = dbg!(parser::parse(&token)).unwrap();
    let c = dbg!(generate::generate(ast));
    println!("{}", c)
}
