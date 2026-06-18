use crate::{
    error::PawResult,
    eval::{EvalContext, TEval, value::Value},
    parser::{SyntaxNode, ast::block::Block},
};

impl<'a> TEval<'a> for Block {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        ctx.env.push_scope();

        for stmt in &self.stmts {
            let stmt = stmt.value();
            stmt.eval(ctx)?;
        }

        ctx.env.pop_scope();

        Ok(Value::Unit)
    }
}
