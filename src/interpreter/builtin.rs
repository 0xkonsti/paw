use crate::interpreter::RuntimeError;
use crate::interpreter::scope::{Scope, Value};

pub fn load_builtins(scope: &mut Scope) {
    scope.set("print".to_string(), Value::BuiltinFn(print));
    scope.set("println".to_string(), Value::BuiltinFn(println));
}

pub fn print(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for arg in args {
        print!("{arg}");
    }

    Ok(Value::Unit) // Return a unit value to indicate success
}

pub fn println(args: Vec<Value>) -> Result<Value, RuntimeError> {
    print(args)?;
    println!(); // Print a newline after printing all arguments
    Ok(Value::Unit)
}
