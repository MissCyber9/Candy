use candy_parser::parse_file;
use candy_typecheck::typecheck;

fn codes(src: &str) -> Vec<String> {
    let p = parse_file("test.candy", src).expect("parse ok");
    let err = typecheck(&p).expect_err("typecheck must fail");
    err.diagnostics.into_iter().map(|d| d.code).collect()
}

#[test]
fn protocol_duplicate_state_is_error() {
    let src = r#"
protocol P {
  state Init;
  state Init;
}
fn main() -> Unit { return; }
"#;

    let c = codes(src);
    assert!(c.contains(&"protocol-duplicate-state".to_string()));
}

#[test]
fn protocol_unknown_state_in_transition_is_error() {
    // IMPORTANT: Init is required by spec, so we include it to avoid masking the real error.
    let src = r#"
protocol P {
  state Init;
  transition Init -> Missing;
}
fn main() -> Unit { return; }
"#;

    let c = codes(src);
    assert!(c.contains(&"protocol-unknown-state".to_string()));
}
