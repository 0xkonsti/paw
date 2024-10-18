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
    return expr->interpret();
}

string ExprStmtNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "ExprStmtNode: {\n";
    ss << string(indent + INDENT, ' ') << "expr: " << expr->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

void DeclStmtNode::parse(shared_ptr<Lexer> const lexer) {
    lexer->consume_token(TokenType::LET);

    id = lexer->consume_token(TokenType::IDENTIFIER).value()->value;

    lexer->consume_token(TokenType::EQ);

    auto expr = parse_expr(lexer, scope);
    if (expr.has_value()) {
        this->expr = std::move(expr.value());
        lexer->consume_token(TokenType::SEMICOLON);
    }

    scope->insert(id, this->expr->interpret());  // FIXME: might not be good to interpret here
}

OptionalNodeValue DeclStmtNode::interpret() const {
    return {};
}

string DeclStmtNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "DeclStmtNode: {\n";
    ss << string(indent + INDENT, ' ') << "id: " << id << '\n';
    ss << string(indent + INDENT, ' ') << "expr: " << expr->debug_string(indent + INDENT) << '\n';
    ss << string(indent, ' ') << "}";

    return ss.str();
}

optional<unique_ptr<StmtNode>> parse_stmt(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope) {
    switch (lexer->peek()->type) {
        case TokenType::LET: {
            auto stmt = std::make_unique<DeclStmtNode>(scope);
            stmt->parse(lexer);
            return std::move(stmt);
        }
        default: {
            auto stmt = std::make_unique<ExprStmtNode>(scope);
            stmt->parse(lexer);
            return std::move(stmt);
        }
    }
}
