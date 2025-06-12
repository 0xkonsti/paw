use super::lambda::PTNLambda;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprType {
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),

    Lambda(PTNLambda),

    Call {
        callee: Box<ExprType>,
        args: Vec<Box<ExprType>>,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNExpr {
    pub(crate) _type: ExprType,
}

impl PTNExpr {
    fn pratt_parse(lexer: &mut Peekable<Lexer>, min_prec: u8) -> Result<Box<dyn PTNode>, String> {
        let mut lhs = PTNExpr::parse_nud(lexer)?;

        loop {
            let token = match lexer.peek().cloned() {
                Some(token) => token,
                None => break,
            };

            if let Some((left_prec, right_prec)) = token.infix_precedence() {
                if left_prec < min_prec {
                    break;
                }

                lexer.next(); // Consume the operator token

                lhs = PTNExpr::parse_led(lexer, lhs, &token, right_prec)?;
            } else {
                break; // No infix precedence, stop parsing
            }
        }

        Ok(lhs)
    }

    fn parse_nud(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        if let Some(token) = lexer.peek() {
            match token.token_type() {
                TokenType::Identifier => {
                    let identifier = token.lexeme().to_string();
                    lexer.next(); // Consume the identifier token
                    Ok(Box::new(PTNExpr {
                        _type: ExprType::Identifier(identifier),
                    }))
                }

                TokenType::String => {
                    let string_literal = token.lexeme().to_string();
                    lexer.next(); // Consume the string literal token
                    Ok(Box::new(PTNExpr {
                        _type: ExprType::StringLiteral(string_literal),
                    }))
                }

                TokenType::Integer => {
                    let integer_literal = token.lexeme().parse::<i64>().map_err(|e| {
                        // NOTE: this should not happen, as the lexer should ensure valid integers
                        token.error(&format!("Failed to parse integer literal: {}", e))
                    })?;
                    lexer.next(); // Consume the integer token
                    Ok(Box::new(PTNExpr {
                        _type: ExprType::IntegerLiteral(integer_literal),
                    }))
                }

                TokenType::Float => {
                    let float_literal = token.lexeme().parse::<f64>().map_err(|e| {
                        // NOTE: this should not happen, as the lexer should ensure valid floats
                        token.error(&format!("Failed to parse float literal: {}", e))
                    })?;
                    lexer.next(); // Consume the float token
                    Ok(Box::new(PTNExpr {
                        _type: ExprType::FloatLiteral(float_literal),
                    }))
                }

                TokenType::Pipe | TokenType::PipePipe => {
                    let lambda = PTNLambda::parse(lexer)?;
                    Ok(Box::new(PTNExpr {
                        _type: ExprType::Lambda(downcast_node!(lambda, PTNLambda)),
                    }))
                }

                _ => Err(token.error("Expected expression")),
            }
        } else {
            Err("Unexpected end of input while parsing expression".to_string())
        }
    }

    fn parse_led(
        lexer: &mut Peekable<Lexer>,
        left: Box<dyn PTNode>,
        token: &Token,
        right_prec: u8,
    ) -> Result<Box<dyn PTNode>, String> {
        match token.token_type() {
            TokenType::LParen => {
                let mut args = Vec::new();

                while lexer.peek().is_some() {
                    let expr = PTNExpr::pratt_parse(lexer, 0)?;
                    args.push(downcast_node!(expr, PTNExpr));
                    if let Some(next) = lexer.peek() {
                        if next.is_not(TokenType::Comma) {
                            break;
                        }
                        lexer.next(); // Consume the ',' token
                    }
                }

                if lexer
                    .peek()
                    .map_or(false, |t| t.token_type() == TokenType::RParen)
                {
                    lexer.next(); // Consume the ')' token
                } else {
                    return Err(token.error("Expected ')' after function call arguments"));
                }

                Ok(Box::new(PTNExpr {
                    _type: ExprType::Call {
                        callee: Box::new(downcast_node!(left, PTNExpr)._type.clone()),
                        args: args
                            .into_iter()
                            .map(|arg| Box::new(arg._type))
                            .collect(),
                    },
                }))
            }
            _ => Err(token.error("Unexpected token in expression")),
        }
    }
}

impl PTNode for PTNExpr {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        PTNExpr::pratt_parse(lexer, 0)
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Expr
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
