use super::stmt::PTNStmt;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::Lexer;
use crate::lexer::location::Location;
use crate::lexer::token::TokenType;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNBlock {
    pub(crate) stmts: Vec<PTNStmt>,
    pub(crate) location: Location,
}

impl PTNode for PTNBlock {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        // TODO: Add Errors for missing tokens

        let mut location = Location::default(); // Placeholder for location, should be set based on the first token

        if let Some(token) = lexer.next() {
            if token.token_type() != TokenType::LBrace {
                return Err(token.error("Expected '{' at the start of block"));
            }
            location = token.location().clone(); // Set location from the '{' token
        }

        let mut stmts = Vec::new();

        while let Some(token) = lexer.peek() {
            match token.token_type() {
                TokenType::RBrace => {
                    break; // End of block
                }
                TokenType::Eof => {
                    return Err(token.error("Unexpected end of file, expected '}' to close block"));
                }
                _ => {
                    let stmt = PTNStmt::parse(lexer)?;
                    stmts.push(downcast_node!(stmt, PTNStmt));
                }
            }
        }

        if let Some(token) = lexer.next() {
            if token.token_type() != TokenType::RBrace {
                return Err(token.error("Expected '}' at the end of block"));
            }
        }

        Ok(Box::new(PTNBlock {
            stmts,
            location,
        }))
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Block
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn location(&self) -> &Location {
        &self.location
    }
}
