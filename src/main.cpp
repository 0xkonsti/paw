#include <iostream>
#include "utils/utils.hpp"

int main() {
    auto content = read_file("data/paw/test.paw");

    if (content.has_value()) {
        std::cout << content.value() << std::endl;
    }

    return 0;
}
