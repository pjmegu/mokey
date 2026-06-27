mod ast;
mod lexer;
mod parser;

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
    let _ = dbg!(parser::parse(&token));
}
