mod lexer;
mod parser;
mod util;

fn main() {
    println!("Hello, world!");

    let path = "./data/code/hello_world.paw";
    let source = std::fs::read_to_string(&path).expect("Failed to read file");

    let lexer = lexer::Lexer::new(path, &source);

    for token in lexer {
        println!("{token}");
    }
}
