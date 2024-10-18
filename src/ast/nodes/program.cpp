#include "program.hpp"
#include <iostream>
#include <sstream>
#include "ast/nodes/stmt.hpp"

void ProgramNode::parse(shared_ptr<Lexer> const lexer) {
    while (lexer->has_next()) {
        if (auto stmt = parse_stmt(lexer, scope); stmt.has_value()) {
            statements.push_back(std::move(stmt.value()));
        } else {
            std::cerr << "Failed to parse statement\n";
            lexer->next();
            break;
        }
    }
}

OptionalNodeValue ProgramNode::interpret() const {
    int64_t success = 0;

    for (auto const& stmt : statements) {
        if (auto result = stmt->interpret(); result.has_value()) {
            std::cout << result.value() << std::endl;
        }
    }

    return {success};
}

string ProgramNode::debug_string(int const indent) const {
    std::stringstream ss;

    ss << "ProgramNode: {\n";
    ss << string(indent + INDENT, ' ') << "scope: " << scope->debug_string(indent + INDENT) << '\n';
    ss << string(indent + INDENT, ' ') << "statements: [\n";
    int i = 0;
    for (auto const& stmt : statements) {
        ss << string(indent + INDENT * 2, ' ') << i++ << ": " << stmt->debug_string(indent + INDENT * 2) << '\n';
    }
    ss << string(indent + INDENT, ' ') << "]\n";
    ss << string(indent, ' ') << "}";

    return ss.str();
}
