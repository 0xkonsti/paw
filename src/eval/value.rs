use std::fmt;

use crate::{error::PawResult, parser::ast::decl::func_decl::FuncDecl};

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Unit,

    Integer(i64),
    Float(f64),

    Func(&'a FuncDecl),
}

impl<'a> Value<'a> {
    pub fn add(self, other: Value<'a>) -> PawResult<Value<'a>> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Integer(a + b as i64)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + b as f64)),
            _ => todo!("Type Error"),
        }
    }

    pub fn sub(self, other: Value<'a>) -> PawResult<Value<'a>> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Integer(a - b as i64)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - b as f64)),
            _ => todo!("Type Error"),
        }
    }

    pub fn mul(self, other: Value<'a>) -> PawResult<Value<'a>> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Integer(a * b as i64)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * b as f64)),
            _ => todo!("Type Error"),
        }
    }

    pub fn div(self, other: Value<'a>) -> PawResult<Value<'a>> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Integer(a / b as i64)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a / b as f64)),
            _ => todo!("Type Error"),
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),

            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(fl) => write!(f, "{fl}"),

            Value::Func(f) => todo!(),
        }
    }
}
