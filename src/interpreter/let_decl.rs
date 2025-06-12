use super::scope::{Scope, Value};
use crate::interpreter::RuntimeError;
use crate::parser::parse_tree::expr::ExprType;
use crate::parser::parse_tree::let_decl::PTNLetDecl;

pub fn interpret_let_decl(decl: PTNLetDecl, scope: &mut Scope) -> Result<(), RuntimeError> {
    match decl.value._type {
        ExprType::Lambda(lambda) => {
            scope.set(decl.identifier.clone(), Value::lambda(lambda));
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(())
}
