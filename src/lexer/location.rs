#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    path: String,
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(path: String, line: usize, column: usize) -> Self {
        Self {
            path,
            line,
            column,
        }
    }

    pub fn advance(&mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        write!(f, "<{}:{}:{}>", self.path, self.line, self.column)
    }
}
