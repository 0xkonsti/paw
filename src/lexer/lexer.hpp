#pragma once

#include <memory>
#include <optional>
#include <set>
#include <string>
#include "location.hpp"
#include "token.hpp"

using std::string, std::shared_ptr, std::optional, std::set;

class Lexer {
public:
    explicit Lexer(string content, string path) : content(std::move(content)), location(std::move(path), 1, 1) {
    }
    ~Lexer() = default;

    shared_ptr<Token> next();
    shared_ptr<Token> peek();

    optional<shared_ptr<Token>> consume_token(TokenType type);
    optional<shared_ptr<Token>> consume_token(set<TokenType> const& types);

    [[nodiscard]] bool has_next();

private:
    string content;
    uint64_t cursor = 0;
    Location location;

    shared_ptr<Token> peeked = nullptr;
    shared_ptr<Token> current = nullptr;

    bool at_eof = false;
    bool valid = true;

    void trim_whitespace();
    char peek_char();
    char advance();

    bool handle_comment();
    optional<TokenType> handle_multi_char_token(string& value);

    shared_ptr<Token> _next();
};
