use super::{PTNode, PTNodeType};
use crate::check_for_semicolon;
use crate::lexer::Lexer;
use crate::lexer::location::Location;
use crate::lexer::token::TokenType;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirecticeNamespace {
    From { entry: String },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PTNDirective {
    pub(crate) namespace: DirecticeNamespace,
    pub(crate) location: Location,
}

impl PTNode for PTNDirective {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        // TODO: Add Errors for missing tokens

        if let Some(token) = lexer.next() {
            let location = token.location().clone();
            if token.is_directive() {
                match token.token_type() {
                    TokenType::From => {
                        if let Some(dc_token) = lexer.next() {
                            if dc_token.is_not(TokenType::ColonColon) {
                                return Err(dc_token.error("Expected '::' after 'from' directive"));
                            }
                            if let Some(entry_token) = lexer.next() {
                                if entry_token.is_not(TokenType::Identifier) {
                                    return Err(entry_token
                                        .error("Expected identifier after 'from ::' directive"));
                                }
                                let entry = entry_token.lexeme().to_string();

                                check_for_semicolon!(
                                    lexer,
                                    "Expected semicolon after 'from' directive"
                                );

                                return Ok(Box::new(PTNDirective {
                                    namespace: DirecticeNamespace::From {
                                        entry,
                                    },
                                    location,
                                }));
                            }
                        }
                    }
                    _ => unreachable!("Unexpected directive token type: {:?}", token.token_type()),
                }
            }
        }

        unimplemented!("PTNDirective::parse")
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Directive
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn location(&self) -> &Location {
        &self.location
    }
}
