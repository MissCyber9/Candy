use candy_parser::parse_file;
use candy_typecheck::typecheck;

#[test]
fn branching_on_secret_is_error() {
    let src = r#"
fn main() -> Unit {
  let s: secret Bool = true;
  if (s) { return; } else { return; }
}
"#;
    let p = parse_file("main.candy", src).unwrap();
    let err = typecheck(&p).unwrap_err();
    assert!(err.diagnostics.iter().any(|d| d.code == "secret-branch"));
}
