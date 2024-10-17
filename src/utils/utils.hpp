#pragma once

#include <cstdint>
#include <optional>
#include <ostream>
#include <set>
#include <string>
#include <variant>

#define INDENT 4

std::optional<std::string> read_file(std::string const& path);

template <typename T>
std::ostream& operator<<(std::ostream& os, std::set<T> const& values) {
    os << "{";
    for (auto const& v : values) {
        os << v << ", ";
    }
    os << "}";
    return os;
}

enum class NodeValueType { INT, FLOAT, STRING };

std::ostream& operator<<(std::ostream& os, NodeValueType const& type);

using NodeValue = std::variant<int64_t, double, std::string>;

NodeValueType get_type(NodeValue const& value);

std::ostream& operator<<(std::ostream& os, NodeValue const& value);

using OptionalNodeValue = std::optional<NodeValue>;

std::ostream& operator<<(std::ostream& os, OptionalNodeValue const& value);

// TODO: implement op overloading for OptionalValue

OptionalNodeValue operator+(NodeValue const& left, NodeValue const& right);

OptionalNodeValue operator-(NodeValue const& left, NodeValue const& right);

OptionalNodeValue operator*(NodeValue const& left, NodeValue const& right);

OptionalNodeValue operator/(NodeValue const& left, NodeValue const& right);
