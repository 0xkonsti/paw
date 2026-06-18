use crate::{
    error::PawResult,
    eval::{EvalContext, TEval, value::Value},
    parser::ast::stmt::{Stmt, intrinsic::Intrinsic},
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
        todo!()
    }
}
