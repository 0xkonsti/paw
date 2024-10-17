#include "utils.hpp"
#include <cstdint>
#include <fstream>
#include <iostream>
#include <variant>
#include "utils/operator_logic/add.hpp"
#include "utils/operator_logic/div.hpp"
#include "utils/operator_logic/mult.hpp"
#include "utils/operator_logic/sub.hpp"

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

template <typename Func>
OptionalNodeValue overload(NodeValue const& left, NodeValue const& right, Func func) {
    if (std::holds_alternative<int64_t>(left)) {
        auto const& lhs = std::get<int64_t>(left);

        if (std::holds_alternative<int64_t>(right)) {
            return func(lhs, std::get<int64_t>(right));
        }
        if (std::holds_alternative<double>(right)) {
            return func(lhs, std::get<double>(right));
        }
        if (std::holds_alternative<std::string>(right)) {
            return func(lhs, std::get<std::string>(right));
        }
    }

    if (std::holds_alternative<double>(left)) {
        auto const& lhs = std::get<double>(left);

        if (std::holds_alternative<int64_t>(right)) {
            return func(lhs, std::get<int64_t>(right));
        }
        if (std::holds_alternative<double>(right)) {
            return func(lhs, std::get<double>(right));
        }
        if (std::holds_alternative<std::string>(right)) {
            return func(lhs, std::get<std::string>(right));
        }
    }

    if (std::holds_alternative<std::string>(left)) {
        auto const& lhs = std::get<std::string>(left);

        if (std::holds_alternative<int64_t>(right)) {
            sub(lhs, std::to_string(std::get<int64_t>(right)));
            return func(lhs, std::get<int64_t>(right));
        }
        if (std::holds_alternative<double>(right)) {
            return func(lhs, std::get<double>(right));
        }
        if (std::holds_alternative<std::string>(right)) {
            return func(lhs, std::get<std::string>(right));
        }
    }

    throw std::runtime_error("Invalid types for operator overloading -- should never happen");
}

OptionalNodeValue operator+(NodeValue const& left, NodeValue const& right) {
    return overload(left, right, [](auto const& a, auto const& b) { return add(a, b); });
}

OptionalNodeValue operator-(NodeValue const& left, NodeValue const& right) {
    return overload(left, right, [](auto const& a, auto const& b) { return sub(a, b); });
}

OptionalNodeValue operator*(NodeValue const& left, NodeValue const& right) {
    return overload(left, right, [](auto const& a, auto const& b) { return mult(a, b); });
}

OptionalNodeValue operator/(NodeValue const& left, NodeValue const& right) {
    return overload(left, right, [](auto const& a, auto const& b) { return divi(a, b); });
}
