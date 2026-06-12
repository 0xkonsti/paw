mod ast;

use ast::Ast;

use crate::parser::ast::Parse;

pub struct Parser {
    ast: Ast,
}

impl Parser {
    pub fn new() -> Self {
        Self { ast: Ast::parse() }
    }
}
