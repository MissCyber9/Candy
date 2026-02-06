# Candy 🍭 — Agent Diagnostics JSON (v0.2)

When running:

```bash
candy check --agent file.candy
Schema (stable)
{
  "diagnostics": [
    {
      "code": "parse-expected-fn",
      "severity": "Error",
      "message": "Expected `fn`.",
      "span": {
        "file": "file.candy",
        "start_line": 1,
        "start_col": 1,
        "end_line": 1,
        "end_col": 5
      },
      "fix": {
        "replace": "let x = y;",
        "with": "let x: Int = y;"
      }
    }
  ]
}

Rules

diagnostics is always present (can be empty).

code is a stable string identifier.

severity is "Error" or "Warning".

span uses 1-based line/col.

fix is optional.

## v0.3 diagnostic codes (added)
- `secret-copy` — secret value copied without move()
- `use-after-move` — variable used after move()
- `secret-branch` — secret used in branching condition

## v0.4 Effect system diagnostics

New stable error codes:

- `undeclared-effect`
  - Meaning: an effectful operation (intrinsic) was used inside a function that did not declare the required effect.
  - Example triggers:
    - `log("...")` requires `io`
    - `now()` requires `time`
    - `rand()` requires `rand`
  - Fix: add the missing effect to the surrounding function signature via `effects(...)`.

- `effect-leak`
  - Meaning: a function calls another function whose declared effects are not a subset of the caller's declared effects.
  - Fix: add the callee's effects to the caller's `effects(...)` list.

Both errors should include a `fix` object with `replace` and `with` patch hints.
