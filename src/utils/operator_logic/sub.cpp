#include "sub.hpp"
#include <cstddef>
#include <iostream>
#include <ostream>
#include <string>

optional<int64_t> sub(int64_t const a, int64_t const b) {
    return a - b;
}

optional<double> sub(double const a, double const b) {
    return a - b;
}

optional<double> sub(double const a, int64_t const b) {
    return a - static_cast<double>(b);
}

optional<double> sub(int64_t const a, double const b) {
    return static_cast<double>(a) - b;
}

optional<string> sub(string const& a, string const& b) {
    string result = a;
    for (char i : b) {
        std::erase(result, i);
    }
    return result;
}

optional<string> sub(string const& a, int64_t const b) {
    string result = a;
    for (size_t i = 0; i < b && !result.empty(); ++i) {
        result.pop_back();
    }
    return result;
}

optional<string> sub(int64_t const a, string const& b) {
    return {};
}

optional<string> sub(string const& a, double const b) {
    return sub(a, static_cast<int64_t>(b));
}

optional<string> sub(double const a, string const& b) {
    return {};
}
