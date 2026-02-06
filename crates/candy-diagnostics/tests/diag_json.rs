use candy_diagnostics::{Diagnostic, DiagnosticReport, Severity, Span};

#[test]
fn agent_json_is_stable_and_parseable() {
    let sp = Span {
        file: "main.candy".to_string(),
        start_line: 12,
        start_col: 8,
        end_line: 12,
        end_col: 18,
    };

    let mut r = DiagnosticReport::new();
    r.push(
        Diagnostic::error("type-mismatch", "Expected Int, got Bool.", sp).with_fix(
            "let x: Int = true;".to_string(),
            "let x: Int = 1;".to_string(),
        ),
    );

    let json = r.to_json_pretty();

    // JSON must parse
    let v: serde_json::Value = serde_json::from_str(&json).expect("JSON must parse");

    // Schema invariants
    assert!(v.get("diagnostics").is_some());
    let diags = v.get("diagnostics").unwrap().as_array().unwrap();
    assert_eq!(diags.len(), 1);

    let d0 = &diags[0];
    assert_eq!(d0.get("code").unwrap(), "type-mismatch");
    assert_eq!(d0.get("severity").unwrap(), "Error");
    assert_eq!(d0.get("message").unwrap(), "Expected Int, got Bool.");

    let span = d0.get("span").unwrap();
    assert_eq!(span.get("file").unwrap(), "main.candy");
    assert_eq!(span.get("start_line").unwrap(), 12);
    assert_eq!(span.get("start_col").unwrap(), 8);

    // fix must exist and contain replace/with
    let fix = d0.get("fix").unwrap();
    assert!(fix.get("replace").is_some());
    assert!(fix.get("with").is_some());

    // Ensure report considers errors as not OK
    assert!(!r.is_ok());

    // And a warning-only report is OK
    let mut w = DiagnosticReport::new();
    w.push(Diagnostic {
        code: "unused-var".into(),
        severity: Severity::Warning,
        message: "Variable `x` is never used.".into(),
        span: Span::single_point("main.candy", 1, 1),
        fix: None,
    });
    assert!(w.is_ok());
}
