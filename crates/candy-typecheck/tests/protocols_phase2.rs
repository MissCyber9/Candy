use candy_parser::parse_program;
use candy_typecheck::typecheck;

fn diag_codes(src: &str) -> Vec<String> {
    let p = parse_program(src).expect("parse ok");
    match typecheck(&p) {
        Ok(()) => vec![],
        Err(r) => r.diagnostics.into_iter().map(|d| d.code).collect(),
    }
}

#[test]
fn protocol_empty_is_error() {
    let codes = diag_codes("protocol P { } fn main() -> Unit { return; }");
    assert!(codes.contains(&"protocol-empty".to_string()));
}

#[test]
fn protocol_duplicate_transition_is_error() {
    let codes = diag_codes(
        "protocol P { state A; transition A -> A; transition A -> A; } fn main() -> Unit { return; }",
    );
    assert!(codes.contains(&"protocol-duplicate-transition".to_string()));
}
