use crate::eval::value::Value;

pub fn show(val: Value) {
    match val {
        Value::Integer(i) => print!("{i}"),
        Value::Float(f) => print!("{f}"),
        Value::String(s) => print!("{s}"),

        _ => todo!(),
    }
}
