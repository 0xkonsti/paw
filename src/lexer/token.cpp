#include "token.hpp"
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>

string tt_to_name(TokenType const& type) {
    map<TokenType, string> names = {
        {TokenType::END_OF_FILE, "END_OF_FILE"},
        {TokenType::UNKNOWN, "UNKNOWN"},

        {TokenType::IDENTIFIER, "IDENTIFIER"},
        {TokenType::STRING, "STRING"},
        {TokenType::INTEGER, "INTEGER"},
        {TokenType::FLOAT, "FLOAT"},

        {TokenType::LPAREN, "LPAREN"},
        {TokenType::RPAREN, "RPAREN"},
        {TokenType::PLUS, "PLUS"},
        {TokenType::MINUS, "MINUS"},
        {TokenType::STAR, "STAR"},
        {TokenType::SLASH, "SLASH"},
        {TokenType::PERCENT, "PERCENT"},
        {TokenType::EQ, "EQ"},

        {TokenType::SEMICOLON, "SEMICOLON"},

        {TokenType::LET, "LET"},
    };

    if (names.contains(type)) {
        return names[type];
    }

    // This should never happen (just a reminder to add a name for new token types)
    std::cerr << "Forgot to add a name for token type " << static_cast<int>(type) << std::endl;
    throw std::runtime_error("Unknown token type");
}

optional<TokenType> tt_is_single_char(char c) {
    if (map<char, TokenType> types =
            {
                {'(', TokenType::LPAREN},
                {')', TokenType::RPAREN},
                {'+', TokenType::PLUS},
                {'-', TokenType::MINUS},
                {'*', TokenType::STAR},
                {'/', TokenType::SLASH},
                {'%', TokenType::PERCENT},
                {'=', TokenType::EQ},
                {';', TokenType::SEMICOLON},
            };
        types.contains(c)) {
        return {types[c]};
    }

    return std::nullopt;
}

map<string, TokenType> get_multi_char_tokens() {
    map<string, TokenType> types = {};
    return types;
}

optional<TokenType> tt_is_keyword(string const& s) {
    if (map<string, TokenType> types = {{
            "let",
            TokenType::LET,
        }};
        types.contains(s)) {
        return {types[s]};
    }

    return std::nullopt;
}

string Token::to_string() const {
    std::stringstream ss;

    ss << "[" << std::setw(15) << type << "] ~ " << std::setw(60) << value << " @ " << location;

    return ss.str();
}

std::ostream& operator<<(std::ostream& os, TokenType const& type) {
    os << tt_to_name(type);
    return os;
}

std::ostream& operator<<(std::ostream& os, Token const& token) {
    os << token.to_string();
    return os;
}
