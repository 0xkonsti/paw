use crate::{
    error::PawResult,
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::{
        Spanned,
        ast::{Parse, block::Block},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub name: Spanned<String>,
    pub is_entry: bool,

    pub body: Block,
}

impl Parse for FuncDecl {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let _ = lexer.expect_token(TokenKind::Func, "Expected 'func' keyword")?;

        let mut is_entry = false;
        if lexer.peek_is(TokenKind::ExcMark) {
            is_entry = true;
            lexer.next_token();
        }

        let name_token = lexer.expect_token(TokenKind::Identifier, "Expected function name")?;
        let name = Spanned::<String>::from(name_token);

        if lexer.peek_is(TokenKind::LParen) {
            lexer.next_token();

            // TODO: Handle function parameter

            let _ = lexer.expect_token(TokenKind::RParen, "Expected ')'")?;
        }

        // TODO: Handle return type

        // NOTE: Here might be a arena keyword later on

        let _ = lexer.expect_token(TokenKind::LBrace, "Expected '{'")?;

        let body = Block::parse(lexer)?;

        let _ = lexer.expect_token(TokenKind::RBrace, "Expected '}'")?;

        Ok(FuncDecl { name, is_entry, body })
    }
}
