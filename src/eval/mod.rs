use hashbrown::HashMap;

use crate::{
    error::PawResult,
    eval::value::Value,
    parser::{Spanned, SyntaxNode, ast::Ast},
};

mod value;

mod block;
mod decl;
mod expr;
mod file;
mod stmt;

pub trait TEval<'a> {
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>>;
}

impl<'a, T> TEval<'a> for Spanned<T>
where
    T: TEval<'a>,
{
    fn eval(&'a self, ctx: &mut EvalContext<'a>) -> PawResult<Value<'a>> {
        self.value().eval(ctx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalContext<'a> {
    // functions: HashMap<String, &'a FuncDecl>,
    pub env: EnvStack<'a>,
}

impl<'a> EvalContext<'a> {
    pub fn new() -> EvalContext<'a> {
        Self { env: EnvStack::new() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvStack<'a> {
    scopes: Vec<HashMap<String, Value<'a>>>,
}

impl<'a> EnvStack<'a> {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String, val: Value<'a>) -> Result<(), String> {
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name) {
            return Err(format!("'{}' already declared in this scope", name));
        }
        scope.insert(name, val);
        Ok(())
    }

    pub fn set(&mut self, name: String, val: Value<'a>) {
        self.scopes.last_mut().unwrap().insert(name, val);
    }

    pub fn get(&self, name: &str) -> Option<&Value<'a>> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }
}

pub struct Eval;

impl<'a> Eval {
    pub fn run(ast: &'a Ast) -> PawResult<Value<'a>> {
        let mut ctx = EvalContext::<'a>::new();
        ctx.env.push_scope();
        let r = ast.root.eval(&mut ctx);

        println!("{ctx:#?}");

        r
    }
}
