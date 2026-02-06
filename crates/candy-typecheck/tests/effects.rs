use candy_parser::parse_program;
use candy_typecheck::typecheck;

#[test]
fn missing_effect_for_log_is_error() {
    let src = r#"
fn main() -> Unit {
  log("x");
  return;
}
"#;
    let p = parse_program(src).expect("parse ok");
    let err = typecheck(&p).expect_err("should fail");
    assert!(err
        .diagnostics
        .iter()
        .any(|d| d.code == "undeclared-effect"));
}

#[test]
fn missing_effect_for_now_is_error() {
    let src = r#"
fn main() -> Unit {
  let t: Int = now();
  return;
}
"#;
    let p = parse_program(src).expect("parse ok");
    let err = typecheck(&p).expect_err("should fail");
    assert!(err
        .diagnostics
        .iter()
        .any(|d| d.code == "undeclared-effect"));
}

#[test]
fn missing_effect_for_rand_is_error() {
    let src = r#"
fn main() -> Unit {
  let r: Int = rand();
  return;
}
"#;
    let p = parse_program(src).expect("parse ok");
    let err = typecheck(&p).expect_err("should fail");
    assert!(err
        .diagnostics
        .iter()
        .any(|d| d.code == "undeclared-effect"));
}

#[test]
fn effect_leak_through_call_is_error() {
    let src = r#"
fn g() -> Unit effects(io) {
  log("x");
  return;
}

fn main() -> Unit {
  g();
  return;
}
"#;
    let p = parse_program(src).expect("parse ok");
    let err = typecheck(&p).expect_err("should fail");
    assert!(err.diagnostics.iter().any(|d| d.code == "effect-leak"));
}

#[test]
fn effect_declared_allows_intrinsic() {
    let src = r#"
fn main() -> Unit effects(io, time, rand) {
  log("x");
  let t: Int = now();
  let r: Int = rand();
  return;
}
"#;
    let p = parse_program(src).expect("parse ok");
    typecheck(&p).expect("should pass");
}
