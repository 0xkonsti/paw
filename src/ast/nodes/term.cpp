#include "term.hpp"
#include <sstream>

void FactorTermNode::parse(shared_ptr<Lexer> const lexer) {
    if (auto factor = parse_factor(lexer, scope); factor.has_value()) {
        this->factor = std::move(factor.value());
    } else {
        lexer->next();
    }
}

OptionalNodeValue FactorTermNode::interpret() const {
    return {};
}

string FactorTermNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "FactorTermNode: {\n";
    ss << string(indent + INDENT, ' ') << "factor: " << factor->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void BinOpTermNode::parse(shared_ptr<Lexer> lexer) {
    auto op = lexer->consume_token(set{TokenType::STAR, TokenType::SLASH});
    if (!op.has_value()) {
        // error
    }

    this->op = op.value()->value;

    if (auto rhs = parse_term(lexer, scope); rhs.has_value()) {
        this->rhs = std::move(rhs.value());
    } else {
        lexer->next();
    }
}

OptionalNodeValue BinOpTermNode::interpret() const {
    return {};
}

string BinOpTermNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "BinOpTermNode: {\n";
    ss << string(indent + INDENT, ' ') << "op: " << op << '\n';
    ss << string(indent + INDENT, ' ') << "lhs: " << lhs->debug_string(indent + INDENT) << '\n';
    ss << string(indent + INDENT, ' ') << "rhs: " << rhs->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

optional<unique_ptr<TermNode>> parse_term(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope) {
    auto lhs = std::make_unique<FactorTermNode>(scope);
    lhs->parse(lexer);

    if (lexer->peek()->type == TokenType::STAR || lexer->peek()->type == TokenType::SLASH) {
        auto term = std::make_unique<BinOpTermNode>(scope);
        term->lhs = std::move(lhs->factor);
        term->parse(lexer);
        return std::move(term);
    }

    return std::move(lhs);
}
