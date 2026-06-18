use crate::{
    error::PawResult,
    lexer::{Lexer, PeekableLexer, TokenKind},
    parser::{
        Spanned,
        ast::{Parse, decl::Decl},
    },
    util::Location,
};

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub decls: Vec<Spanned<Decl>>,
    pub loc:   Location,
}

impl Parse for File {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let mut decls = Vec::new();

        let mut loc = None;

        while let Some(token) = lexer.peek_token() {
            if loc == None {
                loc = Some(token.loc.clone());
            }
            match token.kind {
                TokenKind::EOF => {
                    lexer.next_token();
                    break;
                }

                _ => {
                    let decl = Decl::parse(lexer)?;
                    decls.push(Spanned::new(decl, token.loc.clone()));
                }
            }
        }

        Ok(File { decls, loc: loc.unwrap_or(lexer.loc.clone()) })
    }
}
