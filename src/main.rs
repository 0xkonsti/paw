use std::{env, process::exit};

mod error;
mod eval;
mod lexer;
mod parser;
mod semantics;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No Source File provided");
        return;
    }
    let path = args[1].clone();
    let source = std::fs::read_to_string(&path).expect("Failed to read file");

    let mut lexer = lexer::Lexer::new(&path, &source);

    // for token in lexer.clone() {
    //     println!("{token}");
    // }

    let parser = parser::Parser::new(&mut lexer).unwrap_or_else(|err| {
        eprintln!("Failed to parse:\n    > {err}");
        exit(1);
    });

    // println!("{parser:#?}");

    if let Err(errors) = semantics::SematicAnaylser::check(&parser) {
        for e in errors {
            eprintln!("{e}");
        }
        exit(1);
    }

    if let Err(err) = eval::Eval::run(&parser.ast) {
        eprintln!("{err}");
        exit(1);
    }
}
