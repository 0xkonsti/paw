#pragma once

#include <cstdint>
#include <optional>
#include <string>

using std::string, std::optional;

// --------------< return type: int64_t >--------------

optional<int64_t> add(int64_t const a, int64_t const b);

// --------------< return type: double >--------------

optional<double> add(double const a, double const b);

optional<double> add(double const a, int64_t const b);

optional<double> add(int64_t const a, double const b);

// --------------< return type: string >--------------

optional<string> add(string const& a, string const& b);

optional<string> add(string const& a, int64_t const b);

optional<string> add(int64_t const a, string const& b);

optional<string> add(string const& a, double const b);

optional<string> add(double const a, string const& b);

optional<string> add(string const& a, bool const b);
