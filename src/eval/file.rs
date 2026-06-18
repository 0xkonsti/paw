use crate::{
    error::PawResult,
    eval::{EvalContext, TEval, value::Value},
    parser::{
        SyntaxNode,
        ast::{decl::Decl, file::File},
    },
};

impl<'a> TEval<'a> for File {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        let mut entry = None;
        for decl in &self.decls {
            decl.value().eval(ctx)?;
            if let Decl::Func(f) = decl.value()
                && f.is_entry
            {
                entry = Some(f);
            }
        }

        entry.expect("Entry point should exist").eval(ctx)
    }
}
