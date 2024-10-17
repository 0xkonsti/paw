#include "add.hpp"

optional<int64_t> add(int64_t const a, int64_t const b) {
    return a + b;
}

optional<double> add(double const a, double const b) {
    return a + b;
}

optional<double> add(double const a, int64_t const b) {
    return a + b;
}

optional<double> add(int64_t const a, double const b) {
    return a + b;
}

optional<string> add(string const& a, string const& b) {
    return a + b;
}

optional<string> add(string const& a, int64_t const b) {
    return a + std::to_string(b);
}

optional<string> add(int64_t const a, string const& b) {
    return std::to_string(a) + b;
}

optional<string> add(string const& a, double const b) {
    return a + std::to_string(b);
}

optional<string> add(double const a, string const& b) {
    return std::to_string(a) + b;
}

optional<string> add(string const& a, bool const b) {
    return a + std::to_string(b);
}
