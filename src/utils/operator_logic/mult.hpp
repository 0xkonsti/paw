#pragma once

#include <cstdint>
#include <optional>
#include <string>

using std::string, std::optional;

// --------------< return type: int64_t >--------------

optional<int64_t> mult(int64_t const a, int64_t const b);

// --------------< return type: double >--------------

optional<double> mult(double const a, double const b);

optional<double> mult(double const a, int64_t const b);

optional<double> mult(int64_t const a, double const b);

// --------------< return type: string >--------------

optional<string> mult(string const& a, string const& b);

optional<string> mult(string const& a, int64_t const b);

optional<string> mult(int64_t const a, string const& b);

optional<string> mult(string const& a, double const b);

optional<string> mult(double const a, string const& b);
