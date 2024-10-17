#include "div.hpp"

optional<int64_t> divi(int64_t const a, int64_t const b) {
    return a / b;
}

optional<double> divi(double const a, double const b) {
    return a / b;
}

optional<double> divi(double const a, int64_t const b) {
    return a / static_cast<double>(b);
}

optional<double> divi(int64_t const a, double const b) {
    return static_cast<double>(a) / b;
}

optional<string> divi(string const& a, string const& b) {
    return {};
}

optional<string> divi(string const& a, int64_t const b) {
    return {};
}

optional<string> divi(int64_t const a, string const& b) {
    return {};
}

optional<string> divi(string const& a, double const b) {
    return {};
}

optional<string> divi(double const a, string const& b) {
    return {};
}
