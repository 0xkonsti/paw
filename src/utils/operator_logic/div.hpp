#pragma once

#include <stdlib.h>
#include <cstdint>
#include <optional>
#include <string>

using std::string, std::optional;

// --------------< return type: int64_t >--------------

optional<int64_t> divi(int64_t const a, int64_t const b);

// --------------< return type: double >--------------

optional<double> divi(double const a, double const b);

optional<double> divi(double const a, int64_t const b);

optional<double> divi(int64_t const a, double const b);

// --------------< return type: string >--------------

optional<string> divi(string const& a, string const& b);

optional<string> divi(string const& a, int64_t const b);

optional<string> divi(int64_t const a, string const& b);

optional<string> divi(string const& a, double const b);

optional<string> divi(double const a, string const& b);
