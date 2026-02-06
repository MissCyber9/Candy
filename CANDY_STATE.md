# Candy 🍭 — Project State

## Current version
- Target release: v0.3.0
- Branch: main
- Repo: https://github.com/MissCyber9/Candy

## What is completed (v0.3.0)
### Language surface
- Minimal functions + let + return
- Types: Int, Bool, Unit
- Secret wrapper type: `secret T`
- Expressions:
  - literals (Int/Bool)
  - variable reference
  - `move(x)` (ownership transfer)
- Statements:
  - let-binding with optional type annotation
  - return
  - if/else

### Crypto-safety by design (v0.3 MVP)
- Secrets cannot be copied (`secret-copy`)
- Secrets can be transferred via `move(x)`
- Use-after-move is rejected (`use-after-move`)
- Branching on secret condition is forbidden (`secret-branch`)

### Diagnostics & agent usability
- Structured diagnostics (code, severity, message, span, optional fix)
- CLI:
  - `candy check <file.candy>`
  - `candy check --agent <file.candy>` outputs JSON ONLY on stdout
- Agent tests:
  - JSON-only contract verified
  - Secret diagnostics verified (race-free via unique tmp files)

## Repro commands (must pass)
```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace


Known pitfalls

CLI tests must use unique temp files (tests run in parallel).

CLI --agent is accepted in any position (flag semantics).

Next version focus (v0.4)

Effects system (io/net/time/rand) with determinism by default

Deterministic logs / audit trail primitives

Extend parser incrementally without adding runtime magic
