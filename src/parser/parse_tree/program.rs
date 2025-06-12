use super::directive::PTNDirective;
use super::stmt::PTNStmt;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::Lexer;
use crate::lexer::location::Location;
use crate::lexer::token::TokenType;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNProgram {
    pub(crate) stmts: Vec<PTNStmt>,
    pub(crate) directives: Vec<PTNDirective>,
    pub(crate) location: Location,
}

impl PTNode for PTNProgram {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        let mut stmts = Vec::new();
        let mut directives = Vec::new();
        let mut location = Location::default();

        while let Some(token) = lexer.peek() {
            location = token.location().clone();
            match token.token_type() {
                TokenType::Eof => {
                    lexer.next();
                    break;
                }
                _type if _type.is_directive() => {
                    let directive = PTNDirective::parse(lexer)?;
                    directives.push(downcast_node!(directive, PTNDirective));
                }
                _ => {
                    let stmt = PTNStmt::parse(lexer)?;
                    stmts.push(downcast_node!(stmt, PTNStmt));
                }
            }
        }

        Ok(Box::new(PTNProgram {
            stmts,
            directives,
            location,
        }))
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Program
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn location(&self) -> &Location {
        &self.location
    }
}
