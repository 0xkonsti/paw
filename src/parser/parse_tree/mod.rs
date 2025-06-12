pub mod block;
pub mod directive;
pub mod expr;
pub mod lambda;
pub mod let_decl;
pub mod parameter;
pub mod program;
pub mod stmt;

use crate::lexer::Lexer;
use std::iter::Peekable;

#[macro_export]
macro_rules! downcast_node {
    ($node:expr, $node_type:ident) => {
        $node.as_any().downcast_ref::<$node_type>().unwrap().clone()
    };
}

#[macro_export]
macro_rules! check_for_semicolon {
    ($lexer:expr, $msg:expr) => {
        if let Some(token) = $lexer.peek() {
            if token.is(crate::lexer::token::TokenType::Semicolon) {
                $lexer.next();
            } else {
                return Err(token.error($msg));
            }
        } else {
            return Err($msg.to_string());
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum PTNodeType {
    Program,

    Directive,

    Block,

    Stmt,
    LetDecl,

    Expr,
    Lambda,
    Parameter,
}

pub trait PTNode: std::fmt::Debug {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String>
    where
        Self: Sized;

    fn node_type(&self) -> PTNodeType;

    fn as_any(&self) -> Box<dyn std::any::Any>;
}

#[derive(Debug)]
pub struct ParseTree {
    root: Box<dyn PTNode>,
}

impl ParseTree {
    pub fn parse(lexer: Lexer) -> Result<Self, String> {
        let root = program::PTNProgram::parse(&mut lexer.peekable())?;

        Ok(Self {
            root,
        })
    }

    pub fn root(&self) -> &Box<dyn PTNode> {
        &self.root
    }
}
