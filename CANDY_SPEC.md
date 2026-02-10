# Candy 🍭 Spec (Truth Table)

## v0.5.0 — Protocol Syntax / AST (Release Option A)

### Implemented ✅
- Lexer keywords: protocol/state/transition
- Parser: top-level supports `fn` and `protocol`
- AST: Program contains `funcs` and `protocols`
- Diagnostics: stable error codes + spans
- Effects system (effects(...) + undeclared-effect + effect-leak)
- Secret linearity (secret-copy, use-after-move, secret-branch)
- Agent JSON diagnostics stability (tests)

### Partially Implemented 🟡
- Protocols: syntax & AST only (no semantic validation yet)

### Planned ⏳
- Protocol semantic validation in typechecker:
  - protocol-unknown-state
  - protocol-duplicate-state
  - protocol-illegal-transition
  - protocol-unreachable-state (optional)
