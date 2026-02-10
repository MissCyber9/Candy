use candy_parser::parse_file;
use candy_typecheck::typecheck;

fn codes(src: &str) -> Vec<String> {
    let p = parse_file("test.candy", src).expect("parse ok");
    let err = typecheck(&p).expect_err("typecheck must fail");
    err.diagnostics.into_iter().map(|d| d.code).collect()
}

#[test]
fn protocol_missing_init_is_error() {
    let src = r#"
protocol P {
  state A;
  transition A -> A;
}

fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-missing-init".to_string()));
}

#[test]
fn protocol_unreachable_state_is_error() {
    let src = r#"
protocol P {
  state Init;
  state A;
  transition Init -> Init;
}

fn main() -> Unit { return; }
"#;
    let c = codes(src);
    assert!(c.contains(&"protocol-unreachable-state".to_string()));
}

#[test]
fn protocol_dead_end_state_is_error() {
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
