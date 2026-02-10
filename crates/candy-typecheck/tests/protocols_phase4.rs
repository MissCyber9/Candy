use candy_parser::parse_file;
use candy_typecheck::typecheck;

fn codes(src: &str) -> Vec<String> {
    let p = parse_file("test.candy", src).expect("parse ok");
    let err = typecheck(&p).expect_err("typecheck must fail");
    err.diagnostics.into_iter().map(|d| d.code).collect()
}

#[test]
fn final_state_allows_dead_end() {
    let src = r#"
protocol P {
  state Init;
  final state Done;
  transition Init -> Done;
}
fn main() -> Unit { return; }
"#;
    let p = parse_file("test.candy", src).unwrap();
    let ok = typecheck(&p);
    assert!(ok.is_ok());
}

#[test]
fn dead_end_non_final_is_error() {
    let src = r#"
protocol P {
  state Init;
  state A;
  transition Init -> A;
}
fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-dead-end-state".to_string()));
}

#[test]
fn final_with_outgoing_is_error() {
    let src = r#"
protocol P {
  state Init;
  final state Done;
  transition Init -> Done;
  transition Done -> Done;
}
fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-final-has-outgoing".to_string()));
}

#[test]
fn nondeterministic_outgoing_is_error() {
    let src = r#"
protocol P {
  state Init;
  state A;
  state B;
  transition Init -> A;
  transition Init -> B;
}
fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-nondeterministic".to_string()));
}
