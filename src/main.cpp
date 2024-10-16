#include <iostream>
#include "ast/ast.hpp"
#include "lexer/lexer.hpp"
#include "utils/utils.hpp"

int main() {
    std::string path = "data/paw/test.paw";
    /*std::string path = "data/lexer.txt";*/

    auto content = read_file(path);
    int code = 0;

    if (content.has_value()) {
        std::cout << content.value() << std::endl;

        // LL(1) lexer
        auto lexer = Lexer(content.value(), path, 1);

        /*for (auto token = lexer.next(); token->type != TokenType::END_OF_FILE; token = lexer.next()) {*/
        /*    std::cout << *token << std::endl;*/
        /*}*/

        // LL(2) parser
        auto ast = Ast::parse(make_shared<Lexer>(content.value(), path, 2));

        std::cout << ast.debug_string() << std::endl;

        auto exit = ast.interpret();
        if (exit.has_value()) {
            std::cout << "Exit code: " << exit.value() << std::endl;
        } else {
            std::cerr << "Something went wrong :(" << std::endl;
            code = 1;
        }
    }

    return code;
}
