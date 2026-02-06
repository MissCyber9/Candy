use candy_parser::parse_file;
use candy_typecheck::typecheck;

#[test]
fn secret_copy_is_error() {
    let src = r#"
fn main() -> Unit {
  let a: secret Int = 1;
  let b: secret Int = a;
  return;
}
"#;
    let p = parse_file("main.candy", src).unwrap();
    let err = typecheck(&p).unwrap_err();
    assert!(err.diagnostics.iter().any(|d| d.code == "secret-copy"));
}

#[test]
fn move_then_use_is_error() {
    let src = r#"
fn main() -> Unit {
  let a: secret Int = 1;
  let b: secret Int = move(a);
  a;
  return;
}
"#;
    let p = parse_file("main.candy", src).unwrap();
    let err = typecheck(&p).unwrap_err();
    assert!(err.diagnostics.iter().any(|d| d.code == "use-after-move"));
}

#[test]
fn move_ok_compiles() {
    let src = r#"
fn main() -> Unit {
  let a: secret Int = 1;
  let b: secret Int = move(a);
  b;
  return;
}
"#;
    let p = parse_file("main.candy", src).unwrap();
    typecheck(&p).unwrap();
}
