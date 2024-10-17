#include "mult.hpp"

optional<int64_t> mult(int64_t const a, int64_t const b) {
    return a * b;
}

optional<double> mult(double const a, double const b) {
    return a * b;
}

optional<double> mult(double const a, int64_t const b) {
    return a * static_cast<double>(b);
}

optional<double> mult(int64_t const a, double const b) {
    return mult(b, a);
}

optional<string> mult(string const& a, string const& b) {
    return {};
}

optional<string> mult(string const& a, int64_t const b) {
    // repeat a b times
    string result;
    for (int64_t i = 0; i < b; ++i) {
        result += a;
    }
    return result;
}

optional<string> mult(int64_t const a, string const& b) {
    return mult(b, a);
}

optional<string> mult(string const& a, double const b) {
    size_t new_size = static_cast<size_t>(a.size() * b);
    string result;
    result.reserve(new_size);
    for (size_t i = 0; i < new_size; ++i) {
        result += a[i % a.size()];
    }
    return result;
}

optional<string> mult(double const a, string const& b) {
    return mult(b, a);
}
