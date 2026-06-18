use crate::{
    error::{PawError, PawErrorKind, PawResult},
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::{
        Spanned,
        ast::{Parse, expr::Expr},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: Spanned<String>,
    pub expr: Spanned<Expr>,
}

impl Parse for VarDecl {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        lexer.expect_token(TokenKind::Let, "Expected Let at begining of a variable assignment")?;

        let name_token = lexer.expect_token(TokenKind::Identifier, "Expected variable name")?;
        let name = Spanned::<String>::from(name_token);

        let _ = lexer.expect_token(TokenKind::Equal, "Expected '=' or after variable name")?;

        let loc = if let Some(next_token) = lexer.peek_token() {
            next_token.loc.clone()
        } else {
            return Err(PawError::new(
                PawErrorKind::UnexpectedEndOfFile,
                "Expected expression after '='".to_string(),
                lexer.loc.clone(),
            ));
        };

        let expr = Spanned::new(Expr::parse(lexer)?, loc);

        lexer.expect_token(TokenKind::Semicolon, "Expected ';' after variable assignment")?;

        Ok(Self { name, expr })
    }
}
