use crate::{
    error::{PawError, PawErrorKind, PawResult},
    eval::{EvalContext, TEval, value::Value},
    parser::{
        SyntaxNode,
        ast::expr::{BinOp, Expr, Literal},
    },
};

impl<'a> TEval<'a> for Expr {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        match self {
            Expr::Literal(l) => l.eval(ctx),
            Expr::Identifier(i) => ctx.env.get(i.value()).map_or(
                Err(PawError::new(
                    PawErrorKind::UnkownVarible(i.value().clone()),
                    "Variable is not defined for the current scope".to_string(),
                    i.loc(),
                )),
                |v| Ok(v.clone()),
            ),
            Expr::Binary { lhs, rhs, op } => eval_bin(lhs.eval(ctx)?, rhs.eval(ctx)?, *op),
        }
    }
}

impl<'a> TEval<'a> for Literal {
    fn eval(&'a self, _ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        Ok(match self {
            Literal::Integer(i) => Value::Integer(*i),
            Literal::Float(f) => Value::Float(*f),
        })
    }
}

fn eval_bin<'a>(lhs: Value<'a>, rhs: Value<'a>, op: BinOp) -> PawResult<Value<'a>> {
    match op {
        BinOp::Add => lhs.add(rhs),
        BinOp::Sub => lhs.sub(rhs),
        BinOp::Mul => lhs.mul(rhs),
        BinOp::Div => lhs.div(rhs),
    }
}
