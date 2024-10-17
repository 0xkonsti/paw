#pragma once

#include "ast/symbol_table.hpp"
#include "lexer/lexer.hpp"

using std::string;

enum class AstNodeType { PROGRAM, STMT, EXPR, TERM, FACTOR };

struct AstNode {
    shared_ptr<SymbolTable> scope;

    explicit AstNode(shared_ptr<SymbolTable> const& scope) : scope(scope) {
    }

    virtual ~AstNode() = default;

    virtual void parse(shared_ptr<Lexer> lexer) = 0;

    [[nodiscard]] virtual AstNodeType get_type() const = 0;

    [[nodiscard]] virtual OptionalNodeValue interpret() const = 0;

    [[nodiscard]] virtual string debug_string(int indent) const = 0;
};

class Ast {
public:
    static Ast parse(shared_ptr<Lexer> const& lexer);

    [[nodiscard]] std::shared_ptr<SymbolTable> get_global_scope() const;

    [[nodiscard]] std::shared_ptr<AstNode> get_root() const;

    [[nodiscard]] bool has_failed_parse() const;

    [[nodiscard]] std::string debug_string() const;

private:
    std::shared_ptr<SymbolTable> global_scope;
    std::shared_ptr<AstNode> root;

    bool failed_parse = false;
};
