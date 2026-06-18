# paw

An interpreted language that values simplicity — the kind C has — while adding modern quality-of-life features. Inspired by C, Go, and Rust, but chasing none of them.

## Philosophy

Many languages try to "modernize C" but lose what made it elegant in the process — small surface area, predictable control flow, and the feeling that you can hold the entire language in your head. Paw aims to preserve that simplicity while layering on just enough to remove real friction:

- Duck-typed generics — better than `void*` without the complexity of full type parameters
- Methods on structs — ergonomic, not OOP
- Minimal syntax — no class hierarchies, no traits, no inheritance

## Goals

- **Small spec** — the entire language should fit in a few pages
- **Readable** — syntax that signals intent clearly
- **Manual memory management** — current plan is arena/bump allocation; simple and fast, with predictable lifetimes
- **Interpreted first** — fast iteration, easy to understand the runtime. Long-term, native compilation or bytecode are on the table
- **Self-contained** — minimal dependencies, no runtime bloat

## Non-goals

- No classes, inheritance, or virtual dispatch
- No borrow checker or ownership model
- No compile-time metaprogramming (for now)

## Syntax overview

### Entry point

Every file has exactly one entry function, marked with `!`:

```paw
func !main {
    // program starts here
}
```

### Variables

```paw
let i = 42;
let f = 3.14;
```

Types are inferred from literals.

### Functions

```paw
func add(int a, int b) :: int {
    return a + b;
}

// no return type needed for void functions
func greet(str name) {
    #println("hello ", name);
}
```

Parameters use postfix type annotations (`int a`), resembling Go. Return types use `::`.

### Conditionals

```paw
if x > y {
    #println("x wins");
} else {
    #println("y wins");
}
```

No parentheses around conditions — consistent with Go and Rust.

### Loops

```paw
for i = 0; i < 3; i++ {
    #println(i);
}
```

No parentheses around the three clauses (Go-style). C semantics, cleaner syntax.

### Structs

```paw
struct Vec2 {
    float x,
    float y,

    func $new(float x, float y) :: Self {
        return Vec2 { x: x, y: y }
    }

    func print() {
        #print("Vec2(", self.x, ", ", self.y, ")");
    }
}
```

- `$` prefix marks static methods
- `Self` refers to the enclosing struct type
- `self` is the implicit receiver in methods
- Structs can have fields and methods only — no classes, no inheritance

### Generics (duck-typed)

```paw
// anonymous constraint — any type with a `print` method returning void
func print_twice(<print~void> x) {
    #println(x);
    #println(x);
}
```

The angle brackets signal "this is a generic parameter" at a glance. The constraint lists method names and their return types, separated by `~`:

```paw
func serialize(<to_string~str> x);
```

Multiple constraints can be combined with `&`:

```paw
func compare(<hash~int & eq~bool> x);
```

### Named constraints

Reusable constraints can be defined with `constraint`:

```paw
constraint Printable <print~void>;

func print_twice(<Printable> x) {
    #println(x);
    #println(x);
}
```

The `<>` brackets are still required at the use site — they're the visual marker that distinguishes a constraint from a concrete type.

### Intrinsics

Built-in operations prefixed with `#`:

```paw
#print(...);     // print without newline
#println(...);   // print with newline
```

The `#` prefix makes it clear when control leaves paw-space.

### Enums

```paw
enum Color {
    White,
    Black,
    Red,
}

let c = Color.Red;

match c {
    Color.White => #println("white"),
    Color.Black => #println("black"),
    Color.Red   => #println("red"),
    _           => {},
}
```

Enums are namespaced (`Color.Red`, not `Red`) and backed by an integer type.
The `match` statement requires exhaustive handling — use `_ => {}` as a catch-all to explicitly ignore remaining variants.
No payloads or generics on enums; keep that complexity in structs.

## Pipeline

```
Source → Lexer → Parser → AST → Semantic analysis → Eval (→ IR → VM)
```

Up to the AST phase is shared with the planned compiler path. Semantic analysis is a separate visitor pass (name resolution, type checking in-place constraints). Eval is a tree-walking interpreter, with eventual bytecode compilation as a natural performance step.

## Memory

Paw uses arena allocation. The entire AST and runtime heap live in arenas that are freed in bulk at the end of execution. This avoids the complexity of per-allocation tracking while keeping predictable performance. Long-term, an arena-per-scope or arena-per-iteration pattern could enable finer-grained reuse without abandoning the model.

## Status

Pre-alpha. Lexer exists, everything else is being built incrementally through the stage files in `data/code/`:

| File | Feature |
|------|---------|
| `1.paw` | Empty entry point |
| `2.paw` | let bindings, int/float literals |
| `3.paw` | if/else, comparisons, #println |
| `4.paw` | User functions, parameters, return |
| `5.paw` | For loops |
| `6.paw` | Structs, static methods, self |
| `7.paw` | Generics with duck-typed constraints |
| `8.paw` | Named constraints (constraint) |

## License

MIT
