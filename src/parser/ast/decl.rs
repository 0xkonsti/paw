use crate::{
    error::{PawError, PawErrorKind, PawResult},
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::ast::{
        Parse,
        decl::{func_decl::FuncDecl, var_decl::VarDecl},
    },
};

pub mod func_decl;
pub mod var_decl;

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Func(FuncDecl),
    Var(VarDecl),
}

impl Parse for Decl {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let token = lexer.peek_token().unwrap();

        match token.kind {
            TokenKind::Func => {
                let decl = FuncDecl::parse(lexer)?;
                Ok(Decl::Func(decl))
            }

            _ => Err(PawError::new(
                PawErrorKind::UnexpectedToken(token.val.clone()),
                "Expected Declaration".to_string(),
                token.loc.clone(),
            )),
        }
    }
}
