use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    path:   PathBuf,
    line:   u32,
    column: u32,
}

impl Location {
    pub fn new(path: PathBuf, line: u32, column: u32) -> Self {
        Self { path, line, column }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}:{}:{}>", self.path.to_str().unwrap(), self.line, self.column)
    }
}
