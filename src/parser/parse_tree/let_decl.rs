use super::expr::PTNExpr;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::Lexer;
use crate::lexer::token::TokenType;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNLetDecl {
    pub(crate) identifier: String,
    pub(crate) value: PTNExpr,
}

impl PTNode for PTNLetDecl {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        // TODO: Add Errors for missing tokens

        if let Some(token) = lexer.peek().cloned() {
            if token.is(TokenType::Let) {
                lexer.next(); // Consume the 'let' keyword
                if let Some(iden_token) = lexer.next() {
                    if iden_token.is(TokenType::Identifier) {
                        let iden = iden_token.lexeme().to_string();

                        if let Some(eq_token) = lexer.next() {
                            if eq_token.is(TokenType::Equal) {
                                let value = PTNExpr::parse(lexer)?;

                                return Ok(Box::new(PTNLetDecl {
                                    identifier: iden,
                                    value: downcast_node!(value, PTNExpr),
                                }));
                            } else {
                                return Err(eq_token
                                    .error("Expected '=' after identifier in let declaration"));
                            }
                        }
                    } else {
                        return Err(iden_token.error("Expected identifier after 'let' keyword"));
                    }
                }
            } else {
                return Err(token.error("Expected 'let' keyword at the start of let declaration"));
            }
        }

        unimplemented!("PTNLetDecl::parse");
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::LetDecl
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
