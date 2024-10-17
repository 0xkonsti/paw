#include "symbol_table.hpp"
#include <sstream>

std::ostream& operator<<(std::ostream& os, Symbol const& symbol) {
    os << "Symbol: { id: " << symbol.id << ", value: " << symbol.value << " }";
    return os;
}

bool SymbolTable::contains(string const& id) const {
    return symbols.contains(id) || (parent && parent->contains(id));
}

OptionalNodeValue SymbolTable::lookup(string const& id) {
    if (symbols.contains(id)) {
        return symbols[id]->value;
    }
    if (parent) {
        return parent->lookup(id);
    }
    return {};
}

void SymbolTable::update(string const& id, OptionalNodeValue const& value) {
    if (symbols.contains(id)) {
        symbols[id]->value = value;
    } else if (parent) {
        parent->update(id, value);
    }
}

std::string SymbolTable::debug_string(int indent) const {
    std::stringstream ss;

    ss << "SymbolTable: {\n";
    for (auto const& [id, symbol] : symbols) {
        ss << string(indent + INDENT, ' ') << *symbol << "\n";
    }
    ss << string(indent, ' ') << "}";

    return ss.str();
}
