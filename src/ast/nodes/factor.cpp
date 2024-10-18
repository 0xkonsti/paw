#include "factor.hpp"
#include <iostream>
#include <sstream>
#include "expr.hpp"
#include "utils/utils.hpp"

void IdentFactorNode::parse(shared_ptr<Lexer> lexer) {
    auto const token = lexer->consume_token(TokenType::IDENTIFIER);
    if (!token.has_value()) {
        // error
    }
    auto const& token_val = token.value();
    id = token_val->value;
    if (!scope->contains(id)) {
        std::cerr << "Variable " << id << " is not defined ";
        std::cerr << token_val->location << std::endl;
    } else {
        type = scope->get(id).type;
    }
}

OptionalNodeValue IdentFactorNode::interpret() const {
    return scope->lookup(id);
}

string IdentFactorNode::debug_string(int indent) const {
    std::stringstream ss;

    ss << "IdentFactorNode: {\n";
    ss << string(indent + INDENT, ' ') << "id: " << id << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void StringFactorNode::parse(shared_ptr<Lexer> lexer) {
    auto const token = lexer->consume_token(TokenType::STRING);
    if (!token.has_value()) {
        // error
    }
    value = token.value()->value;
    type = NodeValueType::STRING;
}

OptionalNodeValue StringFactorNode::interpret() const {
    return {value};
}

string StringFactorNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "StringFactorNode: {\n";
    ss << string(indent + INDENT, ' ') << "value: " << value << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void IntFactorNode::parse(shared_ptr<Lexer> const lexer) {
    auto const token = lexer->consume_token(TokenType::INTEGER);
    if (!token.has_value()) {
        // error
    }
    value = std::stoll(token.value()->value);
    type = NodeValueType::INT;
}

OptionalNodeValue IntFactorNode::interpret() const {
    return {value};
}

string IntFactorNode::debug_string(int indent) const {
    std::stringstream ss;

    ss << "IntFactorNode: {\n";
    ss << string(indent + INDENT, ' ') << "value: " << value << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void FloatFactorNode::parse(shared_ptr<Lexer> const lexer) {
    auto const token = lexer->consume_token(TokenType::FLOAT);
    if (!token.has_value()) {
        // error
    }
    value = std::stod(token.value()->value);
    type = NodeValueType::FLOAT;
}

OptionalNodeValue FloatFactorNode::interpret() const {
    return {value};
}

string FloatFactorNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "FloatFactorNode: {\n";
    ss << string(indent + INDENT, ' ') << "value: " << value << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void ParenExprFactorNode::parse(shared_ptr<Lexer> lexer) {
    lexer->consume_token(TokenType::LPAREN);
    auto expr = parse_expr(lexer, scope);
    if (!expr.has_value()) {
        // error
    }
    this->expr = std::move(expr.value());
    lexer->consume_token(TokenType::RPAREN);
    type = this->expr->type;
}

OptionalNodeValue ParenExprFactorNode::interpret() const {
    return expr->interpret();
}

string ParenExprFactorNode::debug_string(int indent) const {
    std::stringstream ss;

    ss << "ParenExprFactorNode: {\n";
    ss << string(indent + INDENT, ' ') << "expr: " << expr->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

optional<unique_ptr<FactorNode>> parse_factor(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope) {
    switch (auto const token = lexer->peek(); token->type) {
        case TokenType::IDENTIFIER: {
            auto factor = std::make_unique<IdentFactorNode>(scope);
            factor->parse(lexer);
            return std::move(factor);
        }
        case TokenType::STRING: {
            auto factor = std::make_unique<StringFactorNode>(scope);
            factor->parse(lexer);
            return std::move(factor);
        }
        case TokenType::INTEGER: {
            auto factor = std::make_unique<IntFactorNode>(scope);
            factor->parse(lexer);
            return std::move(factor);
        }
        case TokenType::FLOAT: {
            auto factor = std::make_unique<FloatFactorNode>(scope);
            factor->parse(lexer);
            return std::move(factor);
        }
        case TokenType::LPAREN: {
            auto factor = std::make_unique<ParenExprFactorNode>(scope);
            factor->parse(lexer);
            return std::move(factor);
        }
        default: {
            std::cerr << "Unexpected token " << token << std::endl;
            lexer->next();
            break;
        }
    }

    return {};
}
