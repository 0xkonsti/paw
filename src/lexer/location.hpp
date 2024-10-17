#pragma once

#include <cstdint>
#include <ostream>
#include <string>

using std::string;

struct Location {
    string path;
    uint64_t line;
    uint64_t column;

    Location(string path, uint64_t const line, uint64_t const column)
        : path(std::move(path)), line(line), column(column) {
    }

    [[nodiscard]] string to_string() const;

    void advance(char c);

    void advance(string const& s);
};

std::ostream& operator<<(std::ostream& os, Location const& location);
