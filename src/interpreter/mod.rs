pub mod block;
pub mod builtin;
pub mod expr;
pub mod let_decl;
pub mod scope;
pub mod stmt;

use crate::downcast_node;
use crate::interpreter::builtin::load_builtins;
use crate::parser::parse_tree::directive::DirecticeNamespace;
use crate::parser::parse_tree::program::PTNProgram;
use crate::parser::parse_tree::stmt::StmtType;
use crate::parser::parse_tree::{PTNodeType, ParseTree};
use block::interpret_block;
use let_decl::interpret_let_decl;
use scope::{Scope, Value};

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}", self.message)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter<'a> {
    global_scope: Scope<'a>,

    entry_point: Option<String>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        let mut scope = Scope::new();

        load_builtins(&mut scope);

        Self {
            global_scope: scope,

            entry_point: None,
        }
    }

    pub fn interpret(&mut self, parse_tree: &ParseTree) -> Result<(), RuntimeError> {
        let root = parse_tree.root();

        if root.node_type() != PTNodeType::Program {
            return Err(RuntimeError {
                message: "Expected a program node".to_string(),
            });
        }

        let program = downcast_node!(root, PTNProgram);

        for directive in program.directives {
            match directive.namespace {
                DirecticeNamespace::From {
                    entry,
                } => {
                    if self.entry_point.is_some() {
                        return Err(RuntimeError {
                            message: "Multiple entry points defined".to_string(),
                        });
                    }
                    self.entry_point = Some(entry);
                }
            }
        }

        for stmt in program.stmts {
            match stmt._type {
                StmtType::LetDecl(let_decl) => {
                    interpret_let_decl(let_decl, &mut self.global_scope)?
                }
                _ => {
                    unimplemented!(
                        "Interpretation for this statement type is not implemented yet: {:?}",
                        stmt._type
                    );
                }
            }
        }

        if let Some(entry) = &self.entry_point {
            if let Some(value) = self.global_scope.get(entry).cloned() {
                match value {
                    Value::Lambda {
                        parameters,
                        body,
                    } => {
                        if !parameters.is_empty() {
                            return Err(RuntimeError {
                                message: format!("Entry point '{}' cannot have parameters", entry),
                            });
                        }

                        interpret_block(&body, &mut self.global_scope)?;
                    }

                    _ => {
                        return Err(RuntimeError {
                            message: format!("Entry point '{}' is not a lambda", entry),
                        });
                    }
                }
            } else {
                return Err(RuntimeError {
                    message: format!("Entry point '{}' not found in global scope", entry),
                });
            }
        }

        Ok(())
    }
}
