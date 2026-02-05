# CANDY_STATE (canonical)

## Current
- Repo: https://github.com/MissCyber9/Candy
- Local: ~/projects/Candy/candy
- Branch: main
- Current milestone: v0.1 closure → start v0.2

## Commit signing
- Policy: NO GPG signing for Candy early releases.
- Local repo setting: `git config commit.gpgsign false`

## v0.1 status — Foundations (expected)
- Workspace crates: candy-cli, candy-parser, candy-ast, candy-typecheck
- Pipeline: parse → typecheck → CLI (verify via cargo test)
- CI: should be green

## What changed in this window
- Created this state file to make progress portable across ChatGPT windows.
- Next: tag v0.1.0 + GitHub release notes + TODO closeout.

## Repro (quality gates)
```bash
cargo metadata --no-deps
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
Next steps

Close v0.1:

Update docs/TODO.md to mark v0.1 complete

Commit + tag v0.1.0 (NO SIGN) + push

Draft GitHub release notes

v0.2 focus: lexer + spans + diagnostics + agent JSON + type scaffolding
