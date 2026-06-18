use crate::{
    error::{PawError, PawErrorKind, PawResult},
    lexer::{Lexer, PeekableLexer, Token, TokenKind},
    parser::{Spanned, ast::Parse},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
}

impl Parse for Literal {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        let token = lexer.next_token().unwrap();
        match token.kind {
            TokenKind::Integer => Ok(Self::Integer(token.val.parse::<i64>().unwrap())),
            TokenKind::Float => Ok(Self::Float(token.val.parse::<f64>().unwrap())),

            _ => Err(PawError {
                kind: PawErrorKind::UnexpectedToken(token.val),
                msg: "Expected a literal".into(),
                loc: token.loc.clone(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    pub fn kind_is(kind: &TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Plus => Some(Self::Add),
            TokenKind::Minus => Some(Self::Sub),
            TokenKind::Asterisk => Some(Self::Mul),
            TokenKind::Slash => Some(Self::Div),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Spanned<Literal>),
    Identifier(Spanned<String>),

    Binary { lhs: Box<Expr>, rhs: Box<Expr>, op: BinOp },
}

impl Expr {
    fn pratt_parse(lexer: &mut Lexer, min_prec: u16) -> PawResult<Self> {
        let mut left = Self::nud(lexer)?;

        while let Some(token) = lexer.peek_token() {
            let Some((lprec, rprec)) = Self::get_precedence(token.kind) else {
                break;
            };

            if lprec < min_prec {
                break;
            }

            lexer.next_token();

            left = Self::led(lexer, left, token, rprec)?;
        }

        Ok(left)
    }

    fn nud(lexer: &mut Lexer) -> PawResult<Self> {
        let token = lexer.peek_token().unwrap();

        match token.kind {
            TokenKind::Identifier => {
                let iden = Self::Identifier(Spanned::<String>::from(token));
                lexer.next_token();

                Ok(iden)
            }

            _ if token.kind.is_literal() => {
                Ok(Self::Literal(Spanned::new(Literal::parse(lexer)?, token.loc)))
            }

            _ => Err(PawError {
                kind: PawErrorKind::UnexpectedToken(token.val),
                msg: "Expected an expression".to_string(),
                loc: token.loc.clone(),
            }),
        }
    }

    fn led(lexer: &mut Lexer, left: Self, token: Token, rprec: u16) -> PawResult<Self> {
        match token.kind {
            _ if let Some(bin_op) = BinOp::kind_is(&token.kind) => {
                let right = Self::pratt_parse(lexer, rprec)?;

                Ok(Self::Binary { lhs: Box::new(left), rhs: Box::new(right), op: bin_op })
            }

            _ => Err(PawError {
                kind: PawErrorKind::UnexpectedToken(token.val),
                msg: "Expected a binary operator".to_string(),
                loc: token.loc.clone(),
            }),
        }
    }

    fn get_precedence(kind: TokenKind) -> Option<(u16, u16)> {
        match kind {
            TokenKind::Plus | TokenKind::Minus => Some((50, 51)),
            TokenKind::Asterisk | TokenKind::Slash => Some((60, 61)),
            _ => None,
        }
    }
}

impl Parse for Expr {
    fn parse(lexer: &mut Lexer) -> PawResult<Self> {
        Self::pratt_parse(lexer, 0)
    }
}
