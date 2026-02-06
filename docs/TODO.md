

# TODO — Candy 🍭

## v0.1 Foundations (release gate)
- [ ] Workspace: single root Cargo.toml, crates under crates/
- [ ] MVP pipeline: candy-cli prints "Candy🍭: parse+typecheck OK"
- [ ] Quality: fmt, clippy (-D warnings), tests
- [ ] CI: GitHub Actions runs fmt/clippy/tests
- [ ] Docs: README + PHILOSOPHY + TODO
- [ ] Release hygiene: tag v0.1.0 + GitHub release notes
- [ ] Handoff: CANDY_STATE.md committed

## v0.2 — Type System & Diagnostics (IN PROGRESS / READY FOR TAG)

### Lexer (minimal mais réel)
- [x] Tokenize identifiers, int literals, keywords (fn/let/return)
- [x] Tokenize symbols: (), {}, :, ;, =, ->
- [x] Spans on every token (line/col)
- [x] Lexer tests (positions, newlines)

### AST with spans
- [x] Span type (file, start/end line/col)
- [x] Every AST node carries Span

### Diagnostics
- [x] Diagnostic { code, severity, message, span, fix? }
- [x] DiagnosticReport { diagnostics[] } + JSON serialization
- [x] Diagnostics tests (JSON parseable, schema invariants)

### Parser v0.2
- [x] Real lexer-based parser
- [x] Parse errors return DiagnosticReport with spans
- [x] Parser tests (valid + invalid)

### CLI
- [x] `candy check <file.candy>`
- [x] `candy check --agent <file.candy>` outputs JSON only to stdout
- [x] Agent JSON schema documented (docs/agent-json.md)
- [x] CLI tests for agent JSON output

### Type system v0.2 (scaffolding)
- [x] Primitive types: Int, Bool, Unit
- [x] main() strict: `fn main() -> Unit` with 0 params
- [x] Let annotations + mismatch diagnostics
- [x] Unknown name diagnostics
- [x] Typecheck tests (main invalid, unknown name)

### Quality gates
- [x] cargo fmt
- [x] cargo clippy -D warnings
- [x] cargo test --workspace


## v0.3 Secrets & Linear Types
- [ ] `secret` keyword + linear restrictions
- [ ] zeroization on drop
- [ ] forbid secret-dependent branches (static rule)


