#include "stmt.hpp"
#include <memory>
#include <sstream>
#include "ast/nodes/expr.hpp"

void ExprStmtNode::parse(shared_ptr<Lexer> const lexer) {
    auto expr = parse_expr(lexer, scope);
    if (expr.has_value()) {
        this->expr = std::move(expr.value());
        lexer->consume_token(TokenType::SEMICOLON);
    }
}

OptionalNodeValue ExprStmtNode::interpret() const {
    return {};
}

string ExprStmtNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "ExprStmtNode: {\n";
    ss << string(indent + INDENT, ' ') << "expr: " << expr->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

optional<unique_ptr<StmtNode>> parse_stmt(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope) {
    switch (lexer->peek()->type) {
        default: {
            auto stmt = std::make_unique<ExprStmtNode>(scope);
            stmt->parse(lexer);
            return std::move(stmt);
        }
    }
}
