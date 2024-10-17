#pragma once

#include <vector>
#include "ast/ast.hpp"
#include "ast/nodes/stmt.hpp"

using std::vector;

struct ProgramNode final : AstNode {
    using AstNode::AstNode;

    ~ProgramNode() override = default;

    vector<shared_ptr<StmtNode>> statements;

    void parse(shared_ptr<Lexer> lexer) override;

    [[nodiscard]] AstNodeType get_type() const override {
        return AstNodeType::PROGRAM;
    }

    [[nodiscard]] OptionalNodeValue interpret() const override;

    [[nodiscard]] string debug_string(int indent) const override;
};
