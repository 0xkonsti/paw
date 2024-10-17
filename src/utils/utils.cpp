#include "utils.hpp"
#include <cstdint>
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

std::ostream& operator<<(std::ostream& os, NodeValueType const& type) {
    switch (type) {
        case NodeValueType::INT:
            os << "INT";
            break;
        case NodeValueType::FLOAT:
            os << "FLOAT";
            break;
        case NodeValueType::STRING:
            os << "STRING";
            break;
    }

    return os;
}

NodeValueType get_type(NodeValue const& value) {
    return static_cast<NodeValueType>(value.index());
}

std::ostream& operator<<(std::ostream& os, NodeValue const& value) {
    std::visit([&os](auto const& v) { os << v; }, value);
    return os;
}

std::ostream& operator<<(std::ostream& os, OptionalNodeValue const& value) {
    if (value.has_value()) {
        os << value.value();
    } else {
        os << "nullopt";
    }

    return os;
}

// Operator overloading for OptionalValue

OptionalNodeValue operator+(NodeValue const& left, NodeValue const& right) {
    if (get_type(left) != get_type(right)) {
        // TODO: maybe do some casting here
        return std::nullopt;
    }

    if (get_type(left) == NodeValueType::INT) {
        return std::get<int64_t>(left) + std::get<int64_t>(right);
    }
    if (get_type(left) == NodeValueType::FLOAT) {
        return std::get<double>(left) + std::get<double>(right);
    }
    return std::get<std::string>(left) + std::get<std::string>(right);
}
