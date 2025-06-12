use super::{PTNode, PTNodeType};
use crate::lexer::Lexer;
use crate::lexer::token::TokenType;
use crate::parser::parse_tree::expr::PTNExpr;
use crate::parser::parse_tree::let_decl::PTNLetDecl;
use crate::{check_for_semicolon, downcast_node};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum StmtType {
    LetDecl(PTNLetDecl),
    Expr(PTNExpr),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PTNStmt {
    pub(crate) _type: StmtType,
}

impl PTNStmt {
    fn let_decl(decl: Box<dyn PTNode>) -> Self {
        PTNStmt {
            _type: StmtType::LetDecl(downcast_node!(decl, PTNLetDecl)),
        }
    }
}

impl PTNode for PTNStmt {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        if let Some(token) = lexer.peek() {
            match token.token_type() {
                TokenType::Let => {
                    let decl = PTNLetDecl::parse(lexer)?;

                    check_for_semicolon!(lexer, "Expected semicolon after let declaration");

                    return Ok(Box::new(PTNStmt::let_decl(decl)));
                }
                _ => {
                    let expr = PTNExpr::parse(lexer)?;

                    check_for_semicolon!(lexer, "Expected semicolon after expression");

                    return Ok(Box::new(PTNStmt {
                        _type: StmtType::Expr(downcast_node!(expr, PTNExpr)),
                    }));
                }
            }
        }

        unimplemented!("PTNStmt::parse")
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Stmt
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
