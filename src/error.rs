use std::fmt;

use crate::util::Location;

pub type PawResult<T> = Result<T, PawError>;
pub type PawResults<T> = Result<T, Vec<PawError>>;

#[derive(Debug, Clone)]
pub enum PawErrorKind {
    UnexpectedToken(String),
    UnexpectedEndOfFile,

    MissingEntryPoint,
    MultipleEntryPoints,

    DuplicateDeclaration(String),

    UnkownVarible(String),
    VariableAlreadyDefined(String),
}

impl fmt::Display for PawErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PawErrorKind::UnexpectedToken(token) => write!(f, "Unexpected token '{token}'"),
            PawErrorKind::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),

            PawErrorKind::MissingEntryPoint => write!(f, "Missing entry point"),
            PawErrorKind::MultipleEntryPoints => write!(f, "Multiple entry points"),
            PawErrorKind::DuplicateDeclaration(name) => write!(f, "Duplicate declaration '{name}'"),

            PawErrorKind::UnkownVarible(name) => {
                write!(f, "Variable '{name}' is unkown in this scope")
            }
            PawErrorKind::VariableAlreadyDefined(name) => {
                write!(f, "Variable '{name}' is already defined")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PawError {
    pub kind: PawErrorKind,
    pub msg: String,
    pub loc: Location,
}

impl PawError {
    pub fn new(kind: PawErrorKind, msg: String, loc: Location) -> Self {
        PawError { kind, msg, loc }
    }
}

impl fmt::Display for PawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} {}", self.kind, self.msg, self.loc)
    }
}

impl std::error::Error for PawError {}
