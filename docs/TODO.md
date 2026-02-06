# TODO — Candy 🍭

This TODO is a release checklist. Keep it short, deterministic, CI-first.

## v0.4 — Effects & Determinism (RELEASED: v0.4.0)
- [x] Lexer: `effects` keyword, comma, string literals
- [x] AST: `Effect` enum, effect spans, `FnDecl.effects`, `Expr::Call`, `Expr::StrLit`
- [x] Parser: parse `effects(...)` clause + calls + string literals
- [x] Typechecker: effects rules + intrinsics + diagnostics (`undeclared-effect`, `effect-leak`)
- [x] CLI agent mode: JSON-only + stable effect error codes
- [x] Tests: parser/typecheck/CLI tests for effects
- [x] Docs: `docs/agent-json.md` updated for v0.4 codes
- [x] State: `CANDY_STATE.md` reflects v0.4.0
- [x] Tag + GitHub Release: v0.4.0

## v0.5 — Protocol Engine (typed state machines) — IN PROGRESS
### Parser / Syntax
- [ ] Add top-level `protocol <Name> { ... }`
- [ ] Parse `state <Name>;`
- [ ] Parse `transition <S1> -> <S2> [effects(...)] ;`
- [ ] Spans on protocol/state/transition keywords + arrow + effect items

### AST
- [ ] `Item::ProtocolDecl`
- [ ] `ProtocolDecl { name, states, transitions, span }`
- [ ] `StateDecl { name, span }`
- [ ] `TransitionDecl { from, to, effects, span }`

### Typechecker: well-formedness
- [ ] Duplicate state names => `protocol-duplicate-state`
- [ ] Transition references unknown state => `protocol-unknown-state`
- [ ] Duplicate transitions (same from,to) => deterministic error code (to decide)

### Typechecker: protocol tokens + intrinsics
- [ ] `Type::ProtocolToken { protocol, state, span }`
- [ ] `enter(P, S)` => token if P exists and S in P
- [ ] `step(tok, S2)` => legal transition only
- [ ] Illegal transition => `protocol-illegal-transition`
- [ ] Unknown protocol => `protocol-unknown`

### Effects gating for transitions
- [ ] If transition declares effects, `step` requires current function declares them
- [ ] Missing effects uses existing diagnostics (`undeclared-effect` or `effect-leak`) consistently

### Agent mode JSON
- [ ] Stable codes:
  - [ ] `protocol-unknown`
  - [ ] `protocol-duplicate-state`
  - [ ] `protocol-unknown-state`
  - [ ] `protocol-illegal-transition`
- [ ] Fix suggestions:
  - [ ] Missing transition => suggest adding `transition S1 -> S2;`
  - [ ] Missing effects => suggest adding `effects(...)` to function

### Tests
- [ ] Parser tests: protocol + transitions + transition effects
- [ ] Typecheck tests: unknown state, illegal transition, effects required, happy path
- [ ] CLI agent tests: JSON-only + codes appear

### Release
- [ ] Update `docs/agent-json.md` with protocol codes
- [ ] Update `CANDY_STATE.md` end-of-release status
- [ ] Tag v0.5.0
- [ ] GitHub release notes
