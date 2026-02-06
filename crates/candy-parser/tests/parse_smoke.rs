use candy_parser::parse_program;

#[test]
fn parse_smoke() {
    let p = parse_program("fn main() -> i32 { 42 }").unwrap();
    assert!(!p.funcs.is_empty());
}
