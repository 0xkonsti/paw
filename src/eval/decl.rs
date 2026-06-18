use crate::{
    error::{PawError, PawErrorKind, PawResult},
    eval::{EvalContext, TEval, value::Value},
    parser::{
        SyntaxNode,
        ast::decl::{Decl, func_decl::FuncDecl, var_decl::VarDecl},
    },
};

impl<'a> TEval<'a> for Decl {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        match self {
            Decl::Func(f) => {
                if let Err(e) = ctx.env.declare(f.name.value().clone(), Value::Func(f)) {
                    return Err(PawError::new(
                        PawErrorKind::VariableAlreadyDefined(f.name.value().clone()),
                        e,
                        f.name.loc(),
                    ));
                }
            }
            Decl::Var(v) => {
                v.eval(ctx)?;
            }
        };
        Ok(Value::Unit)
    }
}

impl<'a> TEval<'a> for FuncDecl {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        ctx.env.push_scope(); // function scope

        self.body.eval(ctx)?;

        ctx.env.pop_scope();
        Ok(Value::Unit)
    }
}

impl<'a> TEval<'a> for VarDecl {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        let value = self.expr.value().eval(ctx)?;
        if let Err(e) = ctx.env.declare(self.name.value().clone(), value) {
            return Err(PawError::new(
                PawErrorKind::VariableAlreadyDefined(self.name.value().clone()),
                e,
                self.name.loc(),
            ));
        }
        Ok(Value::Unit)
    }
}
