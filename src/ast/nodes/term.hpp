#pragma once

#include "ast/ast.hpp"
#include "ast/nodes/factor.hpp"

enum class TermType {
    FACTOR,
    BIN_OP,
};

struct TermNode : AstNode {
    using AstNode::AstNode;

    ~TermNode() override = default;

    [[nodiscard]] AstNodeType get_type() const override {
        return AstNodeType::TERM;
    }

    [[nodiscard]] virtual TermType get_term_type() const = 0;
};

struct FactorTermNode final : TermNode {
    using TermNode::TermNode;

    unique_ptr<FactorNode> factor;

    [[nodiscard]] TermType get_term_type() const override {
        return TermType::FACTOR;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

struct BinOpTermNode final : TermNode {
    using TermNode::TermNode;

    unique_ptr<FactorNode> lhs;
    unique_ptr<TermNode> rhs;
    string op;

    [[nodiscard]] TermType get_term_type() const override {
        return TermType::BIN_OP;
    }

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};

[[nodiscard]]
optional<unique_ptr<TermNode>> parse_term(shared_ptr<Lexer> const& lexer, shared_ptr<SymbolTable> const& scope);
