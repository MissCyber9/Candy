use candy_parser::parse_file;

#[test]
fn parse_smoke_valid_program() {
    let src = "fn main() -> Unit { return; }";
    let p = parse_file("main.candy", src).unwrap();
    assert_eq!(p.funcs.len(), 1);
    assert_eq!(p.funcs[0].name.name, "main");
}

#[test]
fn parse_reports_error_with_span() {
    // missing `fn`
    let src = "main() -> Unit { return; }";
    let err = parse_file("main.candy", src).unwrap_err();
    assert!(!err.diagnostics.is_empty());

    let d0 = &err.diagnostics[0];
    assert_eq!(d0.code, "parse-expected-fn");
    assert_eq!(d0.span.file, "main.candy");
    assert!(d0.span.start_line >= 1);
    assert!(d0.span.start_col >= 1);
}

#[test]
fn parse_secret_type_in_let() {
    let src = "fn main() -> Unit { let x: secret Int = 1; return; }";
    let p = parse_file("main.candy", src).unwrap();
    assert_eq!(p.funcs.len(), 1);
    // Just smoke: if it parsed, we’re good for now.
}
