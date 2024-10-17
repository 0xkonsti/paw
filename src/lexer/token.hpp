#pragma once

#include <map>
#include <optional>
#include <ostream>
#include <string>
#include "location.hpp"

using std::string, std::optional, std::map;

enum class TokenType {
    // SPECIAL
    END_OF_FILE,
    UNKNOWN,

    // LITERALS
    IDENTIFIER,
    STRING,
    INTEGER,
    FLOAT,

    // SINGLE CHAR TOKENS
    LPAREN,
    RPAREN,

    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    EQ,

    SEMICOLON,

    // MULTI CHAR TOKENS

    // KEYWORDS
    LET,

};

string tt_to_name(TokenType const& type);
optional<TokenType> tt_is_single_char(char c);
optional<TokenType> tt_is_keyword(string const& s);
map<string, TokenType> get_multi_char_tokens();

struct Token {
    TokenType type;
    string value;
    Location location;

    Token(TokenType const type, string value, Location location)
        : type(type), value(std::move(value)), location(std::move(location)) {
    }

    [[nodiscard]] string to_string() const;
};

std::ostream& operator<<(std::ostream& os, TokenType const& type);
std::ostream& operator<<(std::ostream& os, Token const& token);
