use phf::phf_map;

use crate::util::{Location, shorten_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Special tokens
    EOF,
    Invalid,

    // Literals
    Identifier,
    Integer,
    Float,
    String,

    // Keywords
    Func,
    Let,

    True,
    False,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,

    Equal,

    Semicolon,
    Comma,

    ExcMark,
    Hash,

    // Operator
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
}

const KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "func" => TokenKind::Func,
    "let" => TokenKind::Let,

    "true" => TokenKind::True,
    "false" => TokenKind::False,
};

const SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '(' => TokenKind::LParen,
    ')' => TokenKind::RParen,
    '{' => TokenKind::LBrace,
    '}' => TokenKind::RBrace,
    '=' => TokenKind::Equal,
    ';' => TokenKind::Semicolon,
    ',' => TokenKind::Comma,
    '!' => TokenKind::ExcMark,
    '#' => TokenKind::Hash,
    '+' => TokenKind::Plus,
    '-' => TokenKind::Minus,
    '*' => TokenKind::Asterisk,
    '/' => TokenKind::Slash,
    '%' => TokenKind::Percent,
};

impl TokenKind {
    pub fn is_keyword(s: &str) -> Option<TokenKind> {
        KEYWORDS.get(s).cloned()
    }

    pub fn is_single_char_token(c: char) -> Option<TokenKind> {
        SINGLE_CHAR_TOKENS.get(&c).cloned()
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenKind::Identifier
                | TokenKind::Integer
                | TokenKind::Float
                | TokenKind::String
                | TokenKind::True
                | TokenKind::False
        )
    }

    fn name(&self) -> String {
        format!("{self:?}").to_uppercase()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub val:  String,
    pub loc:  Location,
}

impl Token {
    pub fn new(kind: TokenKind, val: String, loc: Location) -> Self {
        Self { kind, val, loc }
    }

    pub fn eof(loc: Location) -> Self {
        Self { kind: TokenKind::EOF, val: String::new(), loc }
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:<15}] ~ {:<60} {}",
            self.kind.name(),
            shorten_string(&self.val.escape_default(), 60),
            self.loc
        )
    }
}
