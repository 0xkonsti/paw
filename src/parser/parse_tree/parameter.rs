use crate::lexer::Lexer;
use crate::parser::parse_tree::{PTNode, PTNodeType};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PTNParameter {
    pub(crate) identifier: String,
}

impl PTNParameter {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
        }
    }
}

impl PTNode for PTNParameter {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Box<dyn PTNode>, String> {
        todo!("Implement parameter parsing")
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Parameter
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
