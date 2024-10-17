#pragma once

#include <optional>
#include <ostream>
#include <set>
#include <string>

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
