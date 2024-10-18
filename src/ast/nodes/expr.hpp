#pragma once

#include "ast/ast.hpp"
#include "ast/nodes/term.hpp"

enum class ExprType {
    TERM,
    BIN_OP,
};

struct ExprNode : AstNode {
    using AstNode::AstNode;

    NodeValueType type = NodeValueType::UNDEFINED;

    ~ExprNode() override = default;

    [[nodiscard]] AstNodeType get_type() const override {
        return AstNodeType::EXPR;
    }

    [[nodiscard]] virtual ExprType get_expr_type() const = 0;
};

struct TermExprNode final : ExprNode {
    using ExprNode::ExprNode;

    unique_ptr<TermNode> term;

    [[nodiscard]] ExprType get_expr_type() const override {
        return ExprType::TERM;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct BinOpExprNode final : ExprNode {
    using ExprNode::ExprNode;

    unique_ptr<TermNode> lhs;
    unique_ptr<ExprNode> rhs;
    string op;

    [[nodiscard]] ExprType get_expr_type() const override {
        return ExprType::BIN_OP;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

[[nodiscard]]
optional<unique_ptr<ExprNode>> parse_expr(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope);
