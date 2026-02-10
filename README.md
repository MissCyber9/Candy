# Candy 🍭

**Candy** is a resource-oriented, crypto-safe programming language designed for
**governed agentic systems**, **deterministic behavior**, and **auditability by construction**.

Candy is not a scripting language.
Candy is not a VM playground.
Candy is a **compiler that enforces invariants**.

---

## Core Design Principles

- **Crypto-safety by design**
  - Linear secrets (no copy, no use-after-move)
  - Forbidden control-flow on secret data
- **Determinism by default**
  - All side-effects must be explicitly declared
  - No hidden runtime behavior
- **Auditability**
  - Stable diagnostic codes
  - Precise source spans
  - Machine-readable (agent) diagnostics
- **Compiler-first**
  - Errors are rejected at compile-time
  - No silent fallback or runtime magic

---

## Language Overview

### Top-level items

A Candy program consists of **top-level items**:

program := (fn_decl | protocol_decl)*


---

### Functions

```candy
fn main() -> Unit {
    return;
}


Functions may declare explicit effects:

fn log_time() -> Unit effects(io, time) {
    log("now");
    let t = now();
    return;
}


If an effect is missing, compilation fails.

Effects System (v0.4+)

Supported effects:

io

net

time

rand

Rules:

Effects are explicit

Effects do not propagate implicitly

Calling a function requires declaring all its effects

Diagnostics:

undeclared-effect

effect-leak

Secrets & Linearity (v0.3+)
let x: secret Int = ...;
let y = x;        // ❌ compile error (secret-copy)
let y = move(x); // ✅ ownership transfer


Rules:

Secrets cannot be copied

Secrets cannot be branched on

Use-after-move is rejected

Diagnostics:

secret-copy

use-after-move

secret-branch

Protocols (v0.5)

Candy supports protocol definitions as explicit state machines.

protocol Channel {
    state Init;
    state Open;
    transition Init -> Open;
}

Current Status (v0.5.0)

✅ Implemented:

Lexer support (protocol, state, transition)

Parser support (top-level protocols)

AST representation

Typechecking:

Duplicate state detection

Unknown state in transitions

❌ Not yet implemented:

Runtime protocol execution

Protocol tokens / step semantics

Transition effects enforcement

Reachability analysis

This release does NOT claim full protocol verification.
Only static structural validation is enforced.

Diagnostics:

protocol-duplicate-state

protocol-unknown-state

CLI
cargo run -p candy-cli -- check file.candy

Agent mode (JSON-only output)
cargo run -p candy-cli -- check --agent file.candy


Agent diagnostics are:

JSON-only

Stable schema

Stable error codes

Designed for AI-driven code repair.

Build & Quality Gates

Mandatory invariants:

cargo fmt
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings


The project is considered invalid if any gate fails.

Project Status

Current version: v0.5.0

Stability: audit-ready

Security claims: compile-time only

No runtime guarantees beyond what is enforced statically

Roadmap (Short)

v0.5.x — Protocol semantic enforcement (illegal transitions, tokens)

v0.6 — Protocol tokens + step typing

v0.7 — Agent policies & explainability

v1.0 — Stable, security-reviewed release

License

TBD (not yet declared)

