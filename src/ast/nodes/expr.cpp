#include "expr.hpp"
#include <memory>
#include <sstream>

void TermExprNode::parse(shared_ptr<Lexer> const lexer) {
    if (auto term = parse_term(lexer, scope); term.has_value()) {
        this->term = std::move(term.value());
    } else {
        lexer->next();
    }
}

OptionalNodeValue TermExprNode::interpret() const {
    return term->interpret();
}

string TermExprNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "TermExprNode: {\n";
    ss << string(indent + INDENT, ' ') << "term: " << term->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void BinOpExprNode::parse(shared_ptr<Lexer> const lexer) {
    auto const op = lexer->consume_token(set{TokenType::PLUS, TokenType::MINUS});
    if (!op.has_value()) {
        // error
    }

    this->op = op.value()->value;

    if (auto rhs = parse_expr(lexer, scope); rhs.has_value()) {
        this->rhs = std::move(rhs.value());
    } else {
        lexer->next();
    }
}

OptionalNodeValue BinOpExprNode::interpret() const {
    auto rhs = &this->rhs;
    auto op = this->op;
    auto result = this->lhs->interpret();

    while (rhs->get()->get_expr_type() == ExprType::BIN_OP) {
        auto bin_op = dynamic_cast<BinOpExprNode*>(rhs->get());
        if (op == "+") {
            result = {result.value() + bin_op->lhs->interpret().value()};
        } else if (op == "-") {
            result = {result.value() - bin_op->lhs->interpret().value()};
        }

        rhs = &bin_op->rhs;
        op = bin_op->op;
    }

    if (op == "+") {
        result = {result.value() + rhs->get()->interpret().value()};
    } else if (op == "-") {
        result = {result.value() - rhs->get()->interpret().value()};
    }

    return result;
}

string BinOpExprNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "BinOpExprNode: {\n";
    ss << string(indent + INDENT, ' ') << "op: " << op << '\n';
    ss << string(indent + INDENT, ' ') << "lhs: " << lhs->debug_string(indent + INDENT) << '\n';
    ss << string(indent + INDENT, ' ') << "rhs: " << rhs->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

optional<unique_ptr<ExprNode>> parse_expr(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope) {
    auto lhs = std::make_unique<TermExprNode>(scope);
    lhs->parse(lexer);

    if (auto token = lexer->peek(); token->type == TokenType::PLUS || token->type == TokenType::MINUS) {
        auto expr = std::make_unique<BinOpExprNode>(scope);
        expr->lhs = std::move(lhs->term);
        expr->parse(lexer);
        return std::move(expr);
    }

    return std::move(lhs);
}
