use crate::{
    error::PawResult,
    eval::{EvalContext, TEval, intrinsic, value::Value},
    parser::{
        SyntaxNode,
        ast::stmt::{Stmt, intrinsic::Intrinsic},
    },
};

impl<'a> TEval<'a> for Stmt {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        match self {
            Stmt::Let(vdecl) => vdecl.eval(ctx),
            Stmt::Intrinsic(intr) => intr.eval(ctx),
        }
    }
}

impl<'a> TEval<'a> for Intrinsic {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        match self.name.value().as_str() {
            "show" => {
                if self.params.len() != 1 {
                    todo!()
                }
                let val = self.params[0].eval(ctx)?;
                intrinsic::show(val);
            }
            _ => todo!(),
        }

        Ok(Value::Unit)
    }
}
