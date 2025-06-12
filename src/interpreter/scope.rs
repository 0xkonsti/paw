use crate::interpreter::RuntimeError;
use crate::parser::parse_tree::block::PTNBlock;
use crate::parser::parse_tree::lambda::PTNLambda;
use crate::parser::parse_tree::parameter::PTNParameter;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),

    Lambda {
        parameters: Vec<PTNParameter>,
        body: PTNBlock,
    },

    BuiltinFn(fn(Vec<Value>) -> Result<Value, RuntimeError>),

    Unit, // Represents a unit type, similar to `()`
}

impl Value {
    pub fn lambda(node: PTNLambda) -> Self {
        Self::Lambda {
            parameters: node.params,
            body: node.body,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(fl) => write!(f, "{fl}"),
            Value::Unit => write!(f, "()"),
            Value::BuiltinFn(_) => write!(f, "<builtin fn>"),
            Value::Lambda {
                ..
            } => write!(f, "<lambda>"),
            // You can add more as needed
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope<'a> {
    parent: Option<Box<&'a Scope<'a>>>,
    variables: HashMap<String, Value>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: &'a Scope) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(name)))
    }
}
