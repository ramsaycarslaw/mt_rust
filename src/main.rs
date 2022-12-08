use std::io::*;
use std::fs::File;

mod tokens;
mod lexer;
mod parser;
mod ast;
mod typechecker;
mod eval;

fn read_file(fname: String) -> String {
    let mut f = File::open(fname).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

fn repl(debug: bool) {
    loop {
        print!("mt -> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut l = lexer::Lexer::new(input);
        let mut p = parser::Parser::new(l.lex(debug));
        let ast = p.parse();
        // let check = typechecker::typecheck(ast);
        let eval = eval::eval(ast);
    }   
}

fn main() {
    repl(false);
}
