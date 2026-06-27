mod lexer;

use clap::Parser;

#[derive(Parser)]
struct Arg {
    #[arg(short)]
    script: String,
}

fn main() {
    let arg = Arg::parse();
    let script = arg.script;
    dbg!(lexer::lexer(&script));
}
