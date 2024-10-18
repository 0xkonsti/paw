#pragma once

#include <optional>
#include "ast/ast.hpp"
#include "ast/nodes/expr.hpp"

using std::optional, std::unique_ptr;

enum class StmtType {
    EXPR,
    DECL,
    ASSIGN,
};

struct StmtNode : AstNode {
    using AstNode::AstNode;

    ~StmtNode() override = default;

    [[nodiscard]] AstNodeType get_type() const override {
        return AstNodeType::STMT;
    }

    [[nodiscard]] virtual StmtType get_stmt_type() const = 0;
};

struct ExprStmtNode final : StmtNode {
    using StmtNode::StmtNode;

    unique_ptr<ExprNode> expr;

    [[nodiscard]] StmtType get_stmt_type() const override {
        return StmtType::EXPR;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct DeclStmtNode final : StmtNode {
    using StmtNode::StmtNode;

    string id;
    NodeValueType type;
    unique_ptr<ExprNode> expr;

    [[nodiscard]] StmtType get_stmt_type() const override {
        return StmtType::DECL;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct AssignStmtNode final : StmtNode {
    using StmtNode::StmtNode;

    string id;
    unique_ptr<ExprNode> expr;

    [[nodiscard]] StmtType get_stmt_type() const override {
        return StmtType::ASSIGN;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

[[nodiscard]]
optional<unique_ptr<StmtNode>> parse_stmt(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope);
