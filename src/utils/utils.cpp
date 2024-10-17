#include "utils.hpp"
#include <fstream>
#include <iostream>

std::optional<std::string> read_file(std::string const& path) {
    std::ifstream file(path);

    if (!file.is_open()) {
        std::cerr << "Failed to open file: " << path << std::endl;
        return std::nullopt;
    }

    std::string content((std::istreambuf_iterator(file)), (std::istreambuf_iterator<char>()));

    file.close();

    return content;
}
