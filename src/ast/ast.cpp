#include "ast.hpp"
#include <sstream>
#include "ast/nodes/program.hpp"

Ast Ast::parse(shared_ptr<Lexer> const& lexer) {
    Ast ast;
    ast.failed_parse = false;
    ast.global_scope = std::make_shared<SymbolTable>();
    ast.root = std::make_shared<ProgramNode>(ast.global_scope);
    ast.root->parse(lexer);
    return ast;
}

std::shared_ptr<SymbolTable> Ast::get_global_scope() const {
    return global_scope;
}

shared_ptr<AstNode> Ast::get_root() const {
    return root;
}

bool Ast::has_failed_parse() const {
    return failed_parse;
}

string Ast::debug_string() const {
    std::stringstream ss;

    ss << "AST: {\n";
    ss << string(INDENT, ' ') << "global_scope: " << global_scope->debug_string(INDENT) << '\n';
    ss << string(INDENT, ' ') << "root: " << root->debug_string(INDENT) << '\n';
    ss << "}\n";

    return ss.str();
}
