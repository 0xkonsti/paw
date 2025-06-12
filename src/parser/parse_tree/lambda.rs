use super::block::PTNBlock;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::Lexer;
use crate::lexer::location::Location;
use crate::lexer::token::TokenType;
use crate::parser::parse_tree::parameter::PTNParameter;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNLambda {
    pub(crate) params: Vec<PTNParameter>,
    pub(crate) body: PTNBlock,
    pub(crate) location: Location,
}

impl PTNode for PTNLambda {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        let peeked = lexer
            .peek()
            .ok_or("Expected lambda expression".to_string())?;

        let mut params = Vec::new();
        let mut can_have_next_param = true;
        let location = peeked.location().clone();

        if peeked.is(TokenType::Pipe) {
            lexer.next();

            while let Some(token) = lexer.peek() {
                if token.is(TokenType::Pipe) {
                    lexer.next();
                    break;
                }
                if !can_have_next_param {
                    return Err(token.error("Unexpected token in lambda parameters"));
                }

                let param = PTNParameter::parse(lexer)?;
                params.push(downcast_node!(param, PTNParameter));

                if let Some(token) = lexer.peek()
                    && token.is(TokenType::Comma)
                {
                    lexer.next();
                } else {
                    can_have_next_param = false;
                }
            }
        } else if peeked.is(TokenType::PipePipe) {
            lexer.next();
        } else {
            return Err(peeked.error("Expected '|' or '||' for lambda parameters"));
        }

        let body = PTNBlock::parse(lexer)?;

        Ok(Box::new(PTNLambda {
            params,
            body: downcast_node!(body, PTNBlock),
            location,
        }))
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Lambda
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn location(&self) -> &Location {
        &self.location
    }
}
