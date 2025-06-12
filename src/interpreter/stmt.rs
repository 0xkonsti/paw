use super::let_decl::interpret_let_decl;
use super::scope::Scope;
use crate::interpreter::RuntimeError;
use crate::interpreter::expr::interpret_expr;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};

pub fn interpret_stmt(stmt: &PTNStmt, scope: &mut Scope) -> Result<(), RuntimeError> {
    match &stmt._type {
        StmtType::LetDecl(decl) => {
            interpret_let_decl(decl.clone(), scope)?;
        }
        StmtType::Expr(expr) => {
            interpret_expr(expr, scope)?;
        }
        _ => {
            unimplemented!("Interpretation for this statement type is not implemented yet");
        }
    }

    Ok(())
}
