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
