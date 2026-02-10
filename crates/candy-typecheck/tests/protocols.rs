use candy_parser::parse_program;
use candy_typecheck::typecheck;

fn must_fail(src: &str, expected_code: &str) {
    let prog = parse_program(src).expect("parser must succeed");
    let err = typecheck(&prog).expect_err("typecheck must fail");
    let codes: Vec<&str> = err.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(
        codes.contains(&expected_code),
        "expected error code `{}`, got {:?}",
        expected_code,
        codes
    );
}

#[test]
fn protocol_duplicate_state_is_error() {
    let src = r#"
protocol P {
  state A;
  state A;
}
fn main() -> Unit { return; }
"#;
    must_fail(src, "protocol-duplicate-state");
}

#[test]
fn protocol_unknown_state_in_transition_is_error() {
    let src = r#"
protocol P {
  state A;
  transition A -> B;
}
fn main() -> Unit { return; }
"#;
    must_fail(src, "protocol-unknown-state");
}
