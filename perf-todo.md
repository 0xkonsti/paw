## Data structure redesign

Root cause of most redundant clones — three structs force heap copies everywhere.

### [ ] Location: `PathBuf` → `FileId`

Current: `Location { path: PathBuf, line: u32, column: u32 }` — every clone copies the heap path.

Replace with:
```rust
struct FileId(u32);
struct Location { file: FileId, line: u32, col: u32 }   // Copy, 12 bytes
```
FileId indexes into a session-owned `Vec<PathBuf>`. Location becomes Copy — no heap.

### [ ] Token.val: `String` → `Option<String>`

Punctuation tokens (`+`, `-`, `=`, `;`, `()`, `{}`, `!`, `#`, `,`, `*`, `/`, `%`) allocate a heap string for a value that's never read.

```rust
struct Token { kind: TokenKind, val: Option<String>, loc: Location }
```

Punctuation → `val: None`. Identifiers/numbers/strings → `val: Some(...)`.

### [ ] String interning: `String` → `Symbol`

Identifiers appear as heap strings in Token.val, Spanned\<String\>, and HashMap keys in EnvStack. Each clone allocates.

Replace with:
```rust
struct Symbol(u32);   // Copy, 4 bytes

Spanned<Symbol>
Expr::Identifier(Spanned<Symbol>)
FuncDecl { name: Spanned<Symbol>, ... }
VarDecl { name: Spanned<Symbol>, ... }
EnvStack { scopes: Vec<HashMap<Symbol, Value>> }
```

Thread an `Interner` through the pipeline (lexer creates symbols, parser stores them, eval uses them as keys).

**Combined effect for `let x = 5;`**: ~10 heap allocations → effectively 0.

---

## Parser clone hotspots

### [ ] `expect_token` clones the returned token

`lexer.rs:216`: `return Ok(token.clone())` — token is already owned from `self.next_token()`. Just `return Ok(token)`. Called by every parser function, so every keyword, identifier, and punctuation token gets a redundant clone.

### [ ] `peek_token()` clones on every peek

`lexer.rs:193`: `self.peek().cloned()` — returns an owned clone of the token. The parser peeks constantly. Return `&Token` instead.

---

## Medium impact

### [ ] `env.get()` clones `Value`

`eval/expr.rs:20`: `|v| Ok(v.clone())` — clones the entire Value on every variable read. Fine for Integer/Float/Bool but allocates for String and Func. Can optimize later.

---

## Easy fix

### [ ] `main.rs` arg handling

`args[1].clone()` is unnecessary — use `&args[1]`. `env::args().nth(1)` avoids the Vec allocation entirely.

---

## Already planned (arena)

### [ ] `Box<Expr>` per binary op

Two heap allocations per expression. Arena allocation for AST nodes solves this naturally.
