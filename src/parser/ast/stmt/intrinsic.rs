use crate::{
    error::PawResult,
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::{
        Spanned,
        ast::{Parse, expr::Expr},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Intrinsic {
    pub name:   Spanned<String>,
    pub params: Vec<Expr>,
}

impl Parse for Intrinsic {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        lexer.expect_token(TokenKind::Hash, "Expected # at the beginning of an Intrinsic call")?;

        let name_token = lexer.expect_token(TokenKind::Identifier, "Expected intrinsic name")?;
        let name = Spanned::<String>::from(name_token);

        lexer.expect_token(TokenKind::LParen, "Expected '(' after name of intrinsic call")?;
        let mut params = Vec::new();
        while let Some(token) = lexer.peek_token() {
            if token.is(TokenKind::RParen) {
                lexer.next_token();
                break;
            }

            params.push(Expr::parse(lexer)?);

            if let Some(token) = lexer.peek_token()
                && token.is(TokenKind::Comma)
            {
                lexer.next_token();
            }
        }

        lexer.expect_token(TokenKind::Semicolon, "Expected ';' after intrinsic call")?;

        Ok(Self { name, params })
    }
}
