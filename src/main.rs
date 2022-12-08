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
    let mut history = Vec::new();
    loop {
        print!("mt -> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut l = lexer::Lexer::new(input.clone());
        let mut p = parser::Parser::new(l.lex(debug));
        let ast = p.parse();
        // let check = typechecker::typecheck(ast);
        eval::eval(ast);
        history.push(input);
    }   
}

fn main() {
    repl(false);
}
