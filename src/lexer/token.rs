use super::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    // Special tokens
    Eof,
    Invalid,

    // Single-character tokens
    LBrace,
    RBrace,
    LParen,
    RParen,

    Equal,

    Comma,
    Colon,
    Semicolon,
    Pipe,

    // Double-character tokens
    ColonColon,

    PipePipe,

    // Keywords
    Let,
    True,
    False,

    // Directives (also Keywords)
    From,

    // Types (more Keywords)

    // Literals
    Integer,
    Float,
    String,
    Character,

    // Identifiers
    Identifier,
}

impl TokenType {
    pub fn is_keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "let" => Some(TokenType::Let),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),

            "from" => Some(TokenType::From),

            // Types
            _ => None,
        }
    }

    pub fn is_directive(&self) -> bool {
        matches!(self, TokenType::From)
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenType::Integer | TokenType::Float | TokenType::String | TokenType::Character
        )
    }

    pub fn is_single_char_token(c: char) -> Option<TokenType> {
        match c {
            '{' => Some(TokenType::LBrace),
            '}' => Some(TokenType::RBrace),
            '(' => Some(TokenType::LParen),
            ')' => Some(TokenType::RParen),

            '=' => Some(TokenType::Equal),

            ',' => Some(TokenType::Comma),
            ':' => Some(TokenType::Colon),
            ';' => Some(TokenType::Semicolon),

            '|' => Some(TokenType::Pipe),

            _ => None,
        }
    }

    pub fn is_double_char_token(c: char, leader: &TokenType) -> Option<TokenType> {
        match leader {
            TokenType::Colon => match c {
                ':' => Some(TokenType::ColonColon),
                _ => None,
            },
            TokenType::Pipe => match c {
                '|' => Some(TokenType::PipePipe),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }

    // -----------------< Private Methods >-----------------
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, location: Location) -> Self {
        Self {
            token_type,
            lexeme,
            location,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn is(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }

    pub fn is_not(&self, token_type: TokenType) -> bool {
        self.token_type != token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn error(&self, msg: &str) -> String {
        format!("{}: {}", msg, self.location)
    }

    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    pub fn is_directive(&self) -> bool {
        self.token_type.is_directive()
    }

    pub fn is_literal(&self) -> bool {
        self.token_type.is_literal()
    }

    pub fn infix_precedence(&self) -> Option<(u8, u8)> {
        match self.token_type {
            TokenType::LParen => Some((20, 21)), // very high precedence
            _ => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:<15}] ~ {:<60} {}",
            self.token_type.to_string(),
            self.lexeme,
            self.location
        )
    }
}
