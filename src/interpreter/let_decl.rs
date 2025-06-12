use super::scope::{Scope, Value};
use crate::interpreter::RuntimeError;
use crate::parser::parse_tree::expr::ExprType;
use crate::parser::parse_tree::let_decl::PTNLetDecl;

pub fn interpret_let_decl(decl: PTNLetDecl, scope: &mut Scope) -> Result<(), RuntimeError> {
    match decl.value._type {
        ExprType::Identifier(identifier) => {
            if let Some(value) = scope.get(&identifier) {
                scope.set(decl.identifier.clone(), value.clone());
            } else {
                return Err(RuntimeError {
                    message: format!("Undefined variable `{}`", identifier),
                });
            }
        }
        ExprType::StringLiteral(value) => {
            scope.set(decl.identifier.clone(), Value::String(value));
        }
        ExprType::IntegerLiteral(value) => {
            scope.set(decl.identifier.clone(), Value::Integer(value));
        }
        ExprType::FloatLiteral(value) => {
            scope.set(decl.identifier.clone(), Value::Float(value));
        }
        ExprType::Lambda(lambda) => {
            scope.set(decl.identifier.clone(), Value::lambda(lambda));
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(())
}
