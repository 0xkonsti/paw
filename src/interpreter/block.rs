use super::scope::Scope;
use super::stmt::interpret_stmt;
use crate::interpreter::RuntimeError;
use crate::parser::parse_tree::block::PTNBlock;

pub fn interpret_block(block: &PTNBlock, scope: &mut Scope) -> Result<(), RuntimeError> {
    let mut block_scope = Scope::with_parent(scope);

    for stmt in &block.stmts {
        interpret_stmt(stmt, &mut block_scope)?;
    }

    Ok(())
}
