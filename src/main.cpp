#include <iostream>
#include "lexer/lexer.hpp"
#include "utils/utils.hpp"

int main() {
    /*std::string path = "data/paw/test.paw";*/
    std::string path = "data/lexer.txt";

    auto content = read_file(path);

    if (content.has_value()) {
        std::cout << content.value() << std::endl;

        auto lexer = Lexer(content.value(), path);

        for (auto token = lexer.next_token(); token->type != TokenType::END_OF_FILE; token = lexer.next_token()) {
            std::cout << *token << std::endl;
        }
    }

    return 0;
}
