use crate::{error::PawResult, lexer::Lexer, parser::ast::Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Intrinsic {
    pub name: String,
}

impl Parse for Intrinsic {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        todo!()
    }
}
