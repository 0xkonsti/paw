#pragma once

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>
#include "location.hpp"
#include "token.hpp"

using std::string, std::shared_ptr, std::optional, std::set, std::vector;

class Lexer {
public:
    explicit Lexer(string content, string path, size_t ll);
    ~Lexer() = default;

    shared_ptr<Token> next();
    shared_ptr<Token> peek(size_t n = 0);

    optional<shared_ptr<Token>> consume_token(TokenType type);
    optional<shared_ptr<Token>> consume_token(set<TokenType> const& types);

    [[nodiscard]] bool has_next();

private:
    string content;
    uint64_t cursor = 0;
    Location location;

    size_t lookahead_size;
    vector<shared_ptr<Token>> lookahead;

    bool at_eof = false;
    bool valid = true;

    void trim_whitespace();
    char peek_char();
    char advance();

    bool handle_comment();
    optional<TokenType> handle_multi_char_token(string& value);

    shared_ptr<Token> _next();
};
