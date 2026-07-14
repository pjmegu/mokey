mod ast;
mod generate;
mod ir;
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
    let ast = dbg!(parser::parse(&token)).unwrap();
    let ir = dbg!(ir::genir(ast)).unwrap();
    let c = generate::generate(ir).unwrap();
    eprintln!("{c}");
    println!("{}", c)
}
