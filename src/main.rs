use std::io::*;
use std::fs::File;

mod tokens;
mod lexer;
mod parser;
mod ast;
mod typechecker;
mod eval;
mod environment;

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
        let mut e = eval::Evaluator::new();
        e.eval(ast);
        history.push(input);
    }   
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        repl(false);
    } else if args.len() == 2 {
        let fname = args[1].clone();
        let mut l = lexer::Lexer::new(read_file(fname));
        let mut p = parser::Parser::new(l.lex(false));
        let ast = p.parse();
        let type_map = p.get_type_map();
        // println!("Typechecking... got {:?}", typechecker::typecheck(&ast));
        typechecker::typecheck(&ast, type_map.clone());
        let mut e = eval::Evaluator::new();
        e.eval(ast);
    } else {
        println!("Usage: mt [filename]");
    }
}
