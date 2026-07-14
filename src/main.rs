mod ast;
mod generate;
mod ir;
mod lexer;
mod parser;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Arg {
    #[arg(short)]
    script: Option<String>,
    #[arg(short)]
    file: Option<PathBuf>,
}

fn main() {
    let arg = Arg::parse();
    let script = match (arg.script, arg.file) {
        (Some(script), None) => script,
        (None, Some(path)) => std::fs::read_to_string(path).unwrap(),
        (Some(_), Some(_)) | (None, None) => panic!("please give me arguement file or script"),
    };
    let token = dbg!(lexer::lexer(&script)).unwrap();
    let ast = dbg!(parser::parse(&token)).unwrap();
    let ir = dbg!(ir::genir(ast)).unwrap();
    let c = generate::generate(ir).unwrap();
    eprintln!("{c}");
    println!("{}", c)
}
