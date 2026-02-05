# TODO — Candy 🍭

## v0.1 Foundations (release gate)
- [ ] Workspace: single root Cargo.toml, crates under crates/
- [ ] MVP pipeline: candy-cli prints "Candy🍭: parse+typecheck OK"
- [ ] Quality: fmt, clippy (-D warnings), tests
- [ ] CI: GitHub Actions runs fmt/clippy/tests
- [ ] Docs: README + PHILOSOPHY + TODO
- [ ] Release hygiene: tag v0.1.0 + GitHub release notes
- [ ] Handoff: CANDY_STATE.md committed

## v0.2 Type System (next)
- [ ] Diagnostics with spans (line/col)
- [ ] Modules + functions + basic types
- [ ] candy check --agent JSON errors + suggested patches (diff)

## v0.3 Secrets & Linear Types
- [ ] `secret` keyword + linear restrictions
- [ ] zeroization on drop
- [ ] forbid secret-dependent branches (static rule)
