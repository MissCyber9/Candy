use candy_parser::parse_file;
use candy_typecheck::typecheck;

#[test]
fn main_ok_unit_return() {
    let src = "fn main() -> Unit { return; }";
    let p = parse_file("main.candy", src).unwrap();
    typecheck(&p).unwrap();
}

#[test]
fn main_invalid_return_type() {
    let src = "fn main() -> Int { return 1; }";
    let p = parse_file("main.candy", src).unwrap();
    let err = typecheck(&p).unwrap_err();
    assert!(err
        .diagnostics
        .iter()
        .any(|d| d.code == "main-invalid-signature"));
}

#[test]
fn unknown_name_is_error() {
    let src = "fn main() -> Unit { x; return; }";
    let p = parse_file("main.candy", src).unwrap();
    let err = typecheck(&p).unwrap_err();
    assert!(err.diagnostics.iter().any(|d| d.code == "name-unknown"));
}
