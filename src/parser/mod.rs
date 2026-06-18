pub mod ast;

use ast::Ast;

use crate::{
    error::PawResult,
    lexer::{Lexer, Token},
    parser::ast::Parse,
    util::Location,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub ast: Ast,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> PawResult<Self> {
        Ok(Self { ast: Ast::parse(lexer)? })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spanned<T> {
    value: T,
    loc: Location,
}

impl<T> Spanned<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }

    pub fn map<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned { value: f(self.value), loc: self.loc }
    }
}

impl From<Token> for Spanned<String> {
    fn from(token: Token) -> Self {
        Self { value: token.val, loc: token.loc }
    }
}

pub trait SyntaxNode<T> {
    fn loc(&self) -> Location;
    fn loc_ref(&self) -> &Location;
    fn value(&self) -> &T;
}

impl<T> SyntaxNode<T> for Spanned<T> {
    fn loc(&self) -> Location {
        self.loc.clone()
    }

    fn loc_ref(&self) -> &Location {
        &self.loc
    }

    fn value(&self) -> &T {
        &self.value
    }
}
