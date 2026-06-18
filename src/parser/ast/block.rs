use crate::{
    error::PawResult,
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::{
        Spanned,
        ast::{Parse, stmt::Stmt},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Spanned<Stmt>>,
}

impl Parse for Block {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let mut stmts = Vec::new();

        while let Some(token) = lexer.peek_token() {
            if token.kind == TokenKind::RBrace {
                break;
            }

            stmts.push(Spanned::new(Stmt::parse(lexer)?, token.loc.clone()))
        }

        Ok(Block { stmts })
    }
}
