mod interpreter;
mod lexer;
mod parser;

fn main() {
    let path = "./data/code/hello_world.paw";
    let source = std::fs::read_to_string(path).expect("Failed to read file");

    println!("-----------------");
    println!("{}", source);
    println!("-----------------");

    let lexer = lexer::Lexer::new(path.to_string(), &source);

    // for token in lexer {
    //     println!("{}", token);
    // }

    let parser = parser::Parser::new(lexer);
    if let Err(e) = parser {
        eprintln!("Error parsing:\n{}", e);
        return;
    }
    let parser = parser.unwrap();

    // println!("{:#?}", parser);
    // println!("-----------------");

    let mut interpreter = interpreter::Interpreter::new();
    if let Err(e) = interpreter.interpret(&parser.tree()) {
        eprintln!("Error interpreting:\n{}", e);
        return;
    }

    // println!("{:#?}", interpreter);
}
