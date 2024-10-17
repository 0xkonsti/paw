#include "location.hpp"

string Location::to_string() const {
    return path + ":" + std::to_string(line) + ":" + std::to_string(column);
}

void Location::advance(char const c) {
    if (c == '\n') {
        line++;
        column = 1;
    } else {
        column++;
    }
}

void Location::advance(string const& s) {
    for (char c : s) {
        advance(c);
    }
}

std::ostream& operator<<(std::ostream& os, Location const& location) {
    os << location.to_string();
    return os;
}
