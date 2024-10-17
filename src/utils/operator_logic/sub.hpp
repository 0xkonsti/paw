#pragma once

#include <cstdint>
#include <optional>
#include <string>

using std::string, std::optional;

// --------------< return type: int64_t >--------------

optional<int64_t> sub(int64_t const a, int64_t const b);

// --------------< return type: double >--------------

optional<double> sub(double const a, double const b);

optional<double> sub(double const a, int64_t const b);

optional<double> sub(int64_t const a, double const b);

// --------------< return type: string >--------------

optional<string> sub(string const& a, string const& b);

optional<string> sub(string const& a, int64_t const b);

optional<string> sub(int64_t const a, string const& b);

optional<string> sub(string const& a, double const b);

optional<string> sub(double const a, string const& b);
