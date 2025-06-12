pub mod location;
pub mod token;
mod util;

use std::iter::Peekable;
use std::str::Chars;
use token::{Token, TokenType};
use util::{is_identifier, is_number};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    cursor: usize,
    location: location::Location,

    at_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(path: String, source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
            cursor: 0,
            location: location::Location::new(path, 1, 1),

            at_eof: false,
        }
    }

    // -----------------< Private Methods >-----------------

    fn advance(&mut self) -> Option<char> {
        let c = self.source.next();
        if let Some(c) = c {
            self.cursor += 1;
            self.location.advance(&c);
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn trim_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn handle_comment(&mut self) -> bool {
        let c = self.peek();
        if let Some(c) = c {
            match c {
                '/' => {
                    while let Some(&c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }

                    return true;
                }
                '*' => {
                    while let Some(c) = self.advance() {
                        if c == '*' {
                            if let Some(&next) = self.peek() {
                                if next == '/' {
                                    self.advance();
                                    break;
                                }
                            }
                        }
                    }

                    return true;
                }
                _ => {}
            }
        }

        false
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_eof {
            return None;
        }

        self.trim_whitespace();

        let location = self.location.clone();
        let Some(c) = self.advance() else {
            self.at_eof = true;
            return Some(Token::new(TokenType::Eof, String::new(), location));
        };

        if self.handle_comment() {
            return self.next();
        }

        let mut is_float = false;

        let (tt, lexeme) = match c {
            c if is_identifier(c, true) => {
                let mut lexeme = c.to_string();
                while let Some(&c) = self.peek() {
                    if !is_identifier(c, false) {
                        break;
                    }
                    lexeme.push(self.advance().unwrap());
                }

                if let Some(tt) = TokenType::is_keyword(&lexeme) {
                    (tt, lexeme)
                } else {
                    (TokenType::Identifier, lexeme)
                }
            }
            c if is_number(c, &mut is_float) => {
                let mut lexeme = c.to_string();
                while let Some(&c) = self.peek() {
                    if !is_number(c, &mut is_float) {
                        break;
                    }
                    lexeme.push(self.advance().unwrap());
                }

                (
                    if is_float {
                        TokenType::Float
                    } else {
                        TokenType::Integer
                    },
                    lexeme,
                )
            }
            '"' => {
                let mut lexeme = String::new();
                while let Some(c) = self.advance() {
                    if c == '"' {
                        break;
                    }
                    lexeme.push(c);
                }

                (TokenType::String, lexeme)
            }
            '\'' => {
                let mut lexeme = String::new();
                while let Some(c) = self.advance() {
                    if c == '\'' {
                        break;
                    }
                    lexeme.push(c);
                }

                (TokenType::Character, lexeme)
            }
            _ => {
                if let Some(tt) = TokenType::is_single_char_token(c) {
                    if let Some(nc) = self.peek()
                        && let Some(dctt) = TokenType::is_double_char_token(*nc, &tt)
                    {
                        let mut lexeme = c.to_string();
                        lexeme.push(self.advance().unwrap());
                        (dctt, lexeme)
                    } else {
                        (tt, c.to_string())
                    }
                } else {
                    (TokenType::Invalid, c.to_string())
                }
            }
        };

        let token = Token::new(tt, lexeme, location);
        Some(token)
    }
}
