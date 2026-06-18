use crate::{
    error::{PawError, PawErrorKind, PawResult},
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::ast::{Parse, decl::var_decl::VarDecl, stmt::intrinsic::Intrinsic},
};

pub mod intrinsic;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(VarDecl),

    Intrinsic(Intrinsic),
}

impl Parse for Stmt {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let token = lexer.peek_token().unwrap();
        let stmt = match token.kind {
            TokenKind::Let => Stmt::Let(VarDecl::parse(lexer)?),

            TokenKind::Hash => Stmt::Intrinsic(Intrinsic::parse(lexer)?),

            _ => {
                return Err(PawError::new(
                    PawErrorKind::UnexpectedToken(token.val),
                    "Expected Statement".to_string(),
                    token.loc.clone(),
                ));
            }
        };

        Ok(stmt)
    }
}
