

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
- [X] `secret` keyword + linear restrictions
- [X] zeroization on drop
- [X] forbid secret-dependent branches (static rule)



## v0.4 — Effects + Determinism Scaffold

A. Lexer / Tokens
- [ ] keyword `effects`
- [ ] token `,` (Comma)
- [ ] string literals `"..."` with spans
- [ ] lexer tests: effects/comma/string/spans

B. AST
- [ ] `Effect` enum: Io | Net | Time | Rand (derive Copy/Eq/Ord)
- [ ] `EffectSpec { effect, span }`
- [ ] `FnDecl.effects: Vec<EffectSpec>`
- [ ] `Expr::Call { callee, args, span }`
- [ ] `Expr::StrLit { value, span }`

C. Parser
- [ ] parse optional `effects(io, time, rand, net)` in fn signature
- [ ] spans for each effect item
- [ ] parse calls `f(...)`
- [ ] parse string literal expressions
- [ ] parser tests for effects grammar + calls + strings

D. Typechecker
- [ ] default pure when effects omitted
- [ ] intrinsics:
  - [ ] `log("...")` requires io
  - [ ] `now()` requires time
  - [ ] `rand()` requires rand
- [ ] rules:
  - [ ] pure uses effectful intrinsic -> `undeclared-effect` + fix add effects(...)
  - [ ] pure calls effectful function -> `effect-leak` + fix add effects(...)
- [ ] typecheck tests for effect errors + OK cases

E. CLI / Agent mode
- [ ] stable JSON codes for effect errors in `candy check --agent`
- [ ] CLI tests: agent JSON includes `undeclared-effect` and `effect-leak`
- [ ] docs update: `docs/agent-json.md` add new codes

F. State / Release
- [ ] update `CANDY_STATE.md` for v0.4 status
- [ ] tag + GitHub release v0.4.0

## v0.4 — Effects & Determinism (in progress)

- [x] Lexer: `effects` keyword, comma, string literals
- [x] AST: `Effect` enum, effect spans, `FnDecl.effects`, `Expr::Call`, `Expr::StrLit`
- [x] Parser: parse `effects(...)` clause + calls + string literals
- [x] Typechecker: effects rules + intrinsics + diagnostics (`undeclared-effect`, `effect-leak`)
- [x] Tests: parser smoke still green + new typecheck effects tests
- [x] CLI tests: agent JSON effect diagnostics
- [ ] Update CANDY_STATE.md with v0.4 status (end-of-release)
- [ ] Tag + GitHub Release v0.4.0
