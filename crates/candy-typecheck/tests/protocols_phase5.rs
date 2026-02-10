use candy_parser::parse_file;
use candy_typecheck::typecheck;

fn codes(src: &str) -> Vec<String> {
    let p = parse_file("test.candy", src).expect("parse ok");
    let err = typecheck(&p).expect_err("typecheck must fail");
    err.diagnostics.into_iter().map(|d| d.code).collect()
}

#[test]
fn unreachable_state_is_error() {
    let src = r#"
protocol P {
  state Init;
  state A;
  state Unused;
  transition Init -> A;
}
fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-unreachable-state".to_string()));
}

#[test]
fn no_final_reachable_is_error() {
    let src = r#"
protocol P {
  state Init;
  state A;
  transition Init -> A;
  transition A -> Init;
}
fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-no-final-reachable".to_string()));
}

#[test]
fn ok_when_final_reachable() {
    let src = r#"
protocol P {
  state Init;
  final state Done;
  transition Init -> Done;
}
fn main() -> Unit { return; }
"#;
    let p = parse_file("test.candy", src).expect("parse ok");
    let ok = typecheck(&p);
    assert!(ok.is_ok());
}
