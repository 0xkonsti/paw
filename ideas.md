# Ideas / future design notes

## Ref<T> (safe pointer)

Ref is a runtime-builtin safe pointer — { arena_id, slot } pair. Deref returns Option<T> (None if arena freed).

```paw
#arena_current();           // returns current arena id
#arena_alloc<T>(arena, T);  // allocates T in arena, returns slot
#arena_get<T>(arena, slot); // returns Option<T>
#arena_get_unchecked<T>(arena, slot); // returns T directly — garbage if arena dead
#arena_free(arena);         // drops arena
```

Use cases for raw arena intrinsics:
- **Custom allocators** — implement a pool allocator, freelist, or ring buffer on top of an arena
- **Manual lifetime control** — free a specific arena early instead of waiting for scope exit
- **Shared memory** — allocate in a parent arena from a child scope to extend a value's lifetime
- **Interop / FFI** — pass arena ids to external code that allocates on paw's behalf
- **Advanced data structures** — intrusive linked lists, slot maps, arenas of arenas
- **Rollback / transactions** — allocate in a temp arena, `#arena_free` on rollback, or merge on commit

Unsafe variant exists for performance-critical paths. Standard library uses `Option` by default.

```paw
struct Ref<T> {
    int arena,
    int slot,

    func $new<T>(T val) :: Self {
        return {
            arena: #arena_current(),
            slot:  #arena_alloc::<T>(#arena_current(), val),
        }
    }

    func deref<T>(ref self) :: Option<T> {
        return #arena_get::<T>(self.arena, self.slot);
    }

    // unsafe — returns garbage if arena freed
    func deref_unchecked<T>(ref self) :: T {
        return #arena_get_unchecked::<T>(self.arena, self.slot);
    }
}
```

## Arena allocation

Arenas are opt-in. Default: share the caller's arena.

```paw
func process() {
    let data = load("big.csv");     // uses caller's arena

    arena {
        let parsed = parse(data);    // scratch — dropped after }
        filter(parsed);
        save("out.csv", parsed);
    }

    // arena memory freed here
    // data is still alive
}

// function with its own arena
func big_work() arena {
    let arr = Array.new(10000);     // lives in this func's arena
    // freed on return
}
```

- `arena { ... }` — new arena for a block, freed on `}`
- `func foo() arena { ... }` — function gets its own arena, freed on return
- Every non-arena block/function uses the current (nearest outer) arena
- `arena` function returning an arena-allocated value is an error (dangling)

Implementation:
- Start with 4KB pages, chain new pages on demand (double each time)
- No user-facing size control for now — optimize later if needed

Potential refinements to revisit later:
- What happens if you assign an arena-local value to an outer scope variable?
- How do nested arenas interact?

## Compiled port

The arena/Ref design is nearly 1:1 between interpreted and compiled:

- `Ref<T>` layout stays `{ arena_id, slot }` — same in both
- `#arena_*` intrinsics are the same runtime calls under both
- Unchecked variant compiles to a direct memory load — C speed
- Checked variant adds a generation compare (couple of instructions)
- The arena table (id → { ptr, offset, generation }) is a runtime component shared by both paths

No redesign needed to compile. The interpreter and compiler share the same arena runtime.

## Feature checklist

### Must-haves (core language)
- [ ] Booleans + logical ops (`&&`, `||`, `!`)
- [ ] String type + operations (concat, len, indexing, interpolation)
- [ ] Dynamic arrays / vectors (stdlib)

### Must-haves (stdlib)
- [ ] Print / println
- [ ] File I/O (read, write, append)
- [ ] String formatting
- [ ] Array / Vec with push, pop, len, at

### Need design decisions
- [ ] Module / import system (`use`, `import`, or `#include`-like?)
- [x] Error handling — **Result type** (not exceptions). No unmarked control flow kills. Errors are explicit values. `try` keyword or `?` postfix for early return.
- [ ] `ref` keyword for mutable parameters? Or a `mut` modifier on params?
- [ ] Type annotations on `let` — `let int x = 5` or always inferred `let x = 5`?
- [ ] Operator overloading — yes/no? If yes, which operators?
- [ ] Slice / range syntax — `arr[0..3]` vs separate function
- [ ] Multi-return? `func div(int a, int b) :: int, int`?
- [ ] `defer` / scope-exit guards?
- [ ] Null / nil literal — or just `Option<T>` everywhere?
- [ ] Integer types — just `int`, or `u8`, `i32`, `u64` etc.?
- [ ] String type — mutable or immutable?
- [ ] Const / compile-time evaluation?
- [ ] Byte / binary type for raw data
- [ ] Closures / lambdas?
- [ ] Switch on integers (match is for enums only?)
- [ ] Default parameter values?
- [ ] Variadic functions?
- [ ] ABI / FFI for calling C from paw
- [ ] Compile target when compiling — native binary? Bytecode VM? Both?

### Tooling
- [ ] Formatter (trivial with paw's syntax)
- [ ] Error messages with spans and suggestions
- [ ] Language server (LSP)
- [ ] REPL
- [ ] Package manager (far future)
