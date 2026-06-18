use crate::{error::PawResult, lexer::Lexer};

pub mod block;
pub mod decl;
pub mod expr;
pub mod file;
pub mod stmt;

pub trait Parse {
    fn parse(lexer: &mut Lexer) -> PawResult<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub root: file::File,
}

impl Parse for Ast {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        Ok(Self { root: file::File::parse(lexer)? })
    }
}
