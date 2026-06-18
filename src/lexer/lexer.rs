use std::{iter::Peekable, path::PathBuf, str::Chars};

use crate::{
    error::{PawError, PawErrorKind, PawResult},
    lexer::{Token, TokenKind},
    util::Location,
};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source:  Peekable<Chars<'a>>,
    cursor:  usize,
    pub loc: Location,

    current_token:  Option<Token>,
    previous_token: Option<Token>,

    at_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(path: &str, source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            cursor: 0,
            loc:    Location::new(PathBuf::from(path), 1, 1),

            current_token:  None,
            previous_token: None,

            at_eof: false,
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        if self.at_eof {
            return None;
        }

        self.trim();

        let loc = self.loc.clone();
        let Some(c) = self.advance() else {
            self.at_eof = true;
            self.current_token = Some(Token::eof(loc));
            return self.current_token.clone();
        };

        if self.comment(c) {
            return self.get_next_token();
        }

        let mut float_flag = false;

        let (kind, val) = match c {
            c if is_identifier(c, true) => {
                let mut val = c.to_string();
                while let Some(&c) = self.peek_char() {
                    if !is_identifier(c, false) {
                        break;
                    }
                    val.push(self.advance().unwrap());
                }

                if let Some(tk) = TokenKind::is_keyword(&val) {
                    (tk, val)
                } else {
                    (TokenKind::Identifier, val)
                }
            }
            c if is_number(c, &mut float_flag) => {
                let mut val = c.to_string();
                while let Some(&c) = self.peek_char() {
                    if !is_number(c, &mut float_flag) {
                        break;
                    }
                    val.push(self.advance().unwrap());
                }
                (if float_flag { TokenKind::Float } else { TokenKind::Integer }, val)
            }
            '"' => {
                let mut val = String::new();
                while let Some(c) = self.advance() {
                    if c == '"' {
                        break;
                    }
                    val.push(c);
                }
                (TokenKind::String, val)
            }
            _ => {
                if let Some(tk) = TokenKind::is_single_char_token(c) {
                    (tk, c.to_string())
                } else {
                    (TokenKind::Invalid, c.to_string())
                }
            }
        };

        self.current_token = Some(Token::new(kind, val, loc));
        self.current_token.clone()
    }

    fn peek(&mut self) -> Option<&Token> {
        if self.at_eof {
            return None;
        }
        if self.current_token.is_none() {
            self.get_next_token();
        }
        self.current_token.as_ref()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.next();
        if let Some(c) = c {
            self.cursor += c.len_utf8();
            self.loc.advance(c);
        }
        c
    }

    fn trim(&mut self) {
        while let Some(&c) = self.source.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn comment(&mut self, c: char) -> bool {
        if c != '/' {
            return false;
        }

        if let Some(&c) = self.peek_char() {
            if c == '/' {
                while let Some(&c) = self.peek_char() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                return true;
            }

            if c == '*' {
                while let Some(c) = self.advance() {
                    if c == '*'
                        && let Some(&next) = self.peek_char()
                        && next == '/'
                    {
                        self.advance();
                        break;
                    }
                }
                return true;
            }
        }

        false
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_token.is_none() {
            self.get_next_token();
        }

        self.previous_token = self.current_token.take();
        self.previous_token.clone()
    }
}

pub trait PeekableLexer<'a> {
    fn peek_token(&mut self) -> Option<Token>;
    fn peek_is(&mut self, kind: TokenKind) -> bool;
    fn next_token(&mut self) -> Option<Token>;
    fn previous_token(&mut self) -> Option<Token>;
    fn expect_token(&mut self, kind: TokenKind, msg: &str) -> PawResult<Token>;
}

impl<'a> PeekableLexer<'a> for Lexer<'a> {
    fn peek_token(&mut self) -> Option<Token> {
        self.peek().cloned()
    }

    fn peek_is(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.peek()
            && token.is(kind)
        {
            return true;
        }
        false
    }

    fn next_token(&mut self) -> Option<Token> {
        self.next()
    }

    fn previous_token(&mut self) -> Option<Token> {
        self.previous_token.clone()
    }

    fn expect_token(&mut self, kind: TokenKind, msg: &str) -> PawResult<Token> {
        let token = self.next_token().unwrap();
        if token.kind == kind {
            return Ok(token.clone());
        }
        Err(PawError::new(
            PawErrorKind::UnexpectedToken(token.val.clone()),
            msg.to_string(),
            token.loc.clone(),
        ))
    }
}

fn is_identifier(c: char, is_first_char: bool) -> bool {
    c == '_' || c.is_alphabetic() || (!is_first_char && c.is_ascii_digit())
}

fn is_number(c: char, is_float: &mut bool) -> bool {
    if c == '.' {
        if *is_float {
            return false;
        }
        *is_float = true;
        return true;
    }

    c.is_ascii_digit()
}
