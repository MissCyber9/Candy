# Changelog

## v0.5.2
- Protocols: static semantic validation completed:
  - duplicate state detection
  - unknown state references in transitions
  - empty protocol checks
  - duplicate transitions checks
  - missing Init detection
  - unreachable states detection
  - dead-end states checks
  - final state semantics (final has no outgoing; final may be dead-end)
  - nondeterministic outgoing transitions detection
- Added protocol-focused typechecker tests (phase1..phase4)
- Added audit artifacts (toolchain + tree + test/clippy logs)

