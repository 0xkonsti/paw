use phf::phf_map;

use crate::util::{Location, shorten_string};

#[derive(Debug, Clone, Copy)]
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

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon,

    DollarSign,
}

const KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "func" => TokenKind::Func,
};

const SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '(' => TokenKind::LParen,
    ')' => TokenKind::RParen,
    '{' => TokenKind::LBrace,
    '}' => TokenKind::RBrace,
    ';' => TokenKind::Semicolon,
    '$' => TokenKind::DollarSign,
};

impl TokenKind {
    pub fn is_keyword(s: &str) -> Option<TokenKind> {
        KEYWORDS.get(s).cloned()
    }

    pub fn is_single_char_token(c: char) -> Option<TokenKind> {
        SINGLE_CHAR_TOKENS.get(&c).cloned()
    }

    fn name(&self) -> String {
        format!("{self:?}").to_uppercase()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    val:  String,
    loc:  Location,
}

impl Token {
    pub fn new(kind: TokenKind, val: String, loc: Location) -> Self {
        Self { kind, val, loc }
    }

    pub fn eof(loc: Location) -> Self {
        Self { kind: TokenKind::EOF, val: String::new(), loc }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{:<15}] ~ {:<60} {}", self.kind.name(), shorten_string(&self.val.escape_default(), 60), self.loc)
    }
}
