use crate::lexer::Lexer;
use parse_tree::ParseTree;

pub mod parse_tree;

#[derive(Debug)]
pub struct Parser {
    tree: ParseTree,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Self, String> {
        Ok(Self {
            tree: ParseTree::parse(lexer)?,
        })
    }

    pub fn tree(&self) -> &ParseTree {
        &self.tree
    }
}
