# CANDY_STATE (canonical)

## Current
- Repo: https://github.com/MissCyber9/Candy
- Local: ~/projects/Candy/candy
- Branch: main
- Version milestone: v0.2.0 (Diagnostics + Agent mode + Type scaffolding)
- Commit signing: disabled (NO GPG) — `git config commit.gpgsign false`

## v0.2.0 — What’s in
### Diagnostics
- `candy-diagnostics`: Span + Diagnostic + DiagnosticReport
- Stable JSON schema for agent mode
- Tests validate JSON parseability and schema invariants

### Lexer + Parser
- `candy-lexer`: minimal tokenization + line/col spans + tests
- `candy-parser`: lexer-based parsing → spanned AST or DiagnosticReport
- Parser tests for valid + invalid inputs (spanned diagnostics)

### AST
- `candy-ast`: all nodes carry Span (Ident/Type/Expr/Stmt/FnDecl/Program)

### Type system (v0.2 scope)
- `candy-typecheck`: Int/Bool/Unit scaffolding
- main() strict validation
- let annotation + mismatch diagnostics
- unknown name diagnostics
- Typecheck tests

### CLI
- `candy check file.candy`
- `candy check --agent file.candy` => JSON only on stdout
- docs: `docs/agent-json.md`
- CLI test validates JSON output

## Repro commands
```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Example
cargo run -p candy-cli -- check --agent examples/hello.candy
Known pitfalls

If git commit fails due to GPG: git config commit.gpgsign false

Agent mode contract: stdout must be JSON only; logs go to stderr.

Next (v0.3 focus)

secret keyword + linear/affine resource tracking

Automatic zeroization on drop

Forbidden secret-dependent control flow

Nonce safety / crypto-safe stdlib wrappers (later v0.8+ per roadmap)
