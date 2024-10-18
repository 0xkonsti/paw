#pragma once

#include <iostream>
#include <map>
#include <memory>
#include <string>
#include <utility>
#include "utils/utils.hpp"

using std::string, std::shared_ptr, std::unique_ptr, std::map;

struct Symbol {
    string id;
    NodeValueType type = NodeValueType::UNDEFINED;
    OptionalNodeValue value;

    Symbol(string id, OptionalNodeValue value) : id(std::move(id)), value(std::move(value)) {
        if (value.has_value()) {
            type = get_type(value.value());
        }
    }

    explicit Symbol(string id) : id(std::move(id)), value({}) {
    }
};

std::ostream& operator<<(std::ostream& os, Symbol const& symbol);

class SymbolTable {
public:
    explicit SymbolTable(shared_ptr<SymbolTable> const& parent) : parent(parent) {
    }
    SymbolTable() : parent({}) {
    }

    void insert(string const& id, OptionalNodeValue const& value) {
        symbols[id] = std::make_unique<Symbol>(id, value);
    }

    [[nodiscard]] bool contains(string const& id) const;

    [[nodiscard]] OptionalNodeValue lookup(string const& id);

    [[nodiscard]] Symbol const& get(string const& id) const {
        return *symbols.at(id);
    }

    void update(string const& id, OptionalNodeValue const& value);

    [[nodiscard]] string debug_string(int indent) const;

private:
    shared_ptr<SymbolTable> parent;
    map<string, unique_ptr<Symbol>> symbols;
};
