#pragma once

#include <cstdint>
#include "ast/ast.hpp"

// Forward declarations
struct ExprNode;

enum class FactorType {
    IDENT,
    STRING,
    INT,
    FLOAT,

    PAREN_EXPR
};

struct FactorNode : AstNode {
    using AstNode::AstNode;

    NodeValueType type = NodeValueType::UNDEFINED;

    ~FactorNode() override = default;

    [[nodiscard]] AstNodeType get_type() const override {
        return AstNodeType::FACTOR;
    }

    [[nodiscard]] virtual FactorType get_factor_type() const = 0;
};

struct IdentFactorNode final : FactorNode {
    using FactorNode::FactorNode;

    string id;

    [[nodiscard]] FactorType get_factor_type() const override {
        return FactorType::IDENT;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct StringFactorNode final : FactorNode {
    using FactorNode::FactorNode;

    string value;

    [[nodiscard]] FactorType get_factor_type() const override {
        return FactorType::STRING;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct IntFactorNode final : FactorNode {
    using FactorNode::FactorNode;

    int64_t value;

    [[nodiscard]] FactorType get_factor_type() const override {
        return FactorType::INT;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct FloatFactorNode final : FactorNode {
    using FactorNode::FactorNode;

    double value;

    [[nodiscard]] FactorType get_factor_type() const override {
        return FactorType::FLOAT;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct ParenExprFactorNode final : FactorNode {
    using FactorNode::FactorNode;

    unique_ptr<ExprNode> expr;

    [[nodiscard]] FactorType get_factor_type() const override {
        return FactorType::PAREN_EXPR;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

[[nodiscard]]
optional<unique_ptr<FactorNode>> parse_factor(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope);
