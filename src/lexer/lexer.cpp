#include "lexer.hpp"
#include <iostream>
#include <optional>
#include "lexer/token.hpp"
#include "utils/utils.hpp"

bool is_identifier(char const c, bool const first) {
    return c == '_' || isalpha(c) || (!first && isdigit(c));
}

bool is_number(char const c, bool& is_float, bool const first) {
    if (c == '.') {
        if (is_float) {
            return false;
        }
        is_float = true;
        return true;
    }
    if (first) {
        return isdigit(c);
    }
    return isdigit(c);
}

shared_ptr<Token> Lexer::next_token() {
    current = peeked ? peeked : next();
    peeked = nullptr;
    return current;
}

shared_ptr<Token> Lexer::peek_token() {
    peeked = peeked ? peeked : next();
    return peeked;
}

optional<shared_ptr<Token>> Lexer::consume_token(set<TokenType> types) {
    if (auto token = peek_token(); types.contains(token->type)) {
        return next_token();
    } else {
        std::cerr << "Expected token of type " << types << "but got " << token->value;
        std::cerr << token->location << std::endl;
    }
    return {};
}

void Lexer::trim_whitespace() {
    while (cursor < content.size() && isspace(content[cursor])) {
        advance();
    }
}

char Lexer::advance() {
    if (cursor >= content.size()) {
        at_eof = true;
        return '\0';
    }
    char const c = content[cursor++];
    location.advance(c);
    return c;
}

char Lexer::peek_char() {
    if (cursor >= content.size()) {
        at_eof = true;
        return '\0';
    }
    return content[cursor];
}

bool Lexer::handle_comment() {
    if (peek_char() == '/') {
        while (peek_char() != '\n') {
            advance();
        }
        return true;
    }
    if (peek_char() == '*') {
        advance();
        while (true) {
            if (peek_char() == '*') {
                advance();
                if (peek_char() == '/') {
                    advance();
                    break;
                }
            } else {
                advance();
            }
        }
        return true;
    }

    return false;
}

optional<TokenType> Lexer::handle_multi_char_token(string& value) {
    for (auto const mcts = get_multi_char_tokens(); auto const& [k, v] : mcts) {
        if (content.substr(cursor, k.size()) == k) {
            value = k;
            cursor += k.size();
            return v;
        }
    }
    return std::nullopt;
}

shared_ptr<Token> Lexer::next() {
    trim_whitespace();

    Location loc = location;
    char const c = advance();

    if (at_eof) {
        return std::make_shared<Token>(TokenType::END_OF_FILE, "", loc);
    }

    if (handle_comment()) {
        return next();
    }

    TokenType type = TokenType::UNKNOWN;
    string value;

    if (is_identifier(c, true)) {
        value = c;
        while (is_identifier(peek_char(), false)) {
            value += advance();
        }

        if (auto const keyword = tt_is_keyword(value); keyword.has_value()) {
            type = keyword.value();
        } else {
            type = TokenType::IDENTIFIER;
        }
    } else if (auto const mct = handle_multi_char_token(value); mct.has_value()) {
        type = mct.value();
    } else if (auto const sct = tt_is_single_char(c); sct.has_value()) {
        type = sct.value();
        value = c;
    } else if (c == '"') {
        while (peek_char() != '"') {
            value += advance();
        }
        advance();
        type = TokenType::STRING;
    } else if (c == '\'') {
        while (peek_char() != '\'') {
            value += advance();
        }
        advance();
        type = TokenType::STRING;
    } else if (bool is_float = false; is_number(c, is_float, true)) {
        value = c;
        while (is_number(peek_char(), is_float, false)) {
            value += advance();
        }

        if (is_float) {
            type = TokenType::FLOAT;
        } else {
            type = TokenType::INTEGER;
        }
    } else {
        /*std::cerr << "Unknown character " << c << " at " << loc << std::endl;*/
        valid = false;
        value = c;
    }

    return std::make_shared<Token>(type, value, loc);
}
