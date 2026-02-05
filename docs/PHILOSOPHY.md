# Candy 🍭 Philosophy (S3P)

Candy is an S3P language: **Safe, Stateful, Proof-oriented**.

## Resource → State → Proof

- **Resources are explicit**: secrets, keys, nonces, permissions are tracked and (eventually) linear by default.
- **State is explicit**: protocols and agents are modeled as state machines; transitions are checked.
- **Proofs are explicit**: critical actions produce verifiable justification artifacts (compile-time and/or runtime certificates).

## v0.1 scope
Foundations only: buildable workspace, deterministic tooling, CI, docs, and a minimal end-to-end pipeline (parse → typecheck → CLI).
