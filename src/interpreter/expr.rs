use crate::interpreter::RuntimeError;
use crate::interpreter::scope::{Scope, Value};
use crate::parser::parse_tree::expr::{ExprType, PTNExpr};

pub fn interpret_expr(expr: &PTNExpr, scope: &mut Scope) -> Result<Value, RuntimeError> {
    interpret_expr_type(&expr._type, scope)
}

fn interpret_expr_type(expr_type: &ExprType, scope: &mut Scope) -> Result<Value, RuntimeError> {
    match expr_type {
        ExprType::StringLiteral(value) => Ok(Value::String(value.clone())),
        ExprType::IntegerLiteral(value) => Ok(Value::Integer(*value)),
        ExprType::FloatLiteral(value) => Ok(Value::Float(*value)),

        ExprType::Identifier(name) => scope
            .get(name)
            .ok_or_else(|| RuntimeError {
                message: format!("Undefined variable `{name}`"),
            })
            .map(|v| v.clone()),

        ExprType::Call {
            callee,
            args,
        } => {
            let callee_value = interpret_expr_type(&callee, scope)?;
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(interpret_expr_type(arg, scope)?);
            }

            match callee_value {
                Value::BuiltinFn(f) => f(arg_values),
                // TODO: Handle calling lambda functions directly
                _ => Err(RuntimeError {
                    message: "Attempted to call a non-function value".to_string(),
                }),
            }
        }
        _ => {
            unimplemented!("Interpretation for this expression type is not implemented yet");
        }
    }
}
