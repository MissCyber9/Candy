use candy_ast::{Effect, Expr};
use candy_parser::parse_program;

#[test]
fn parse_fn_with_effects_clause() {
    let src = r#"
fn main() -> Unit effects(io, time) {
  return;
}
"#;

    let p = parse_program(src).expect("parse ok");
    let main = p.funcs.iter().find(|f| f.name.name == "main").unwrap();

    assert_eq!(main.effects.len(), 2);
    assert_eq!(main.effects[0].effect, Effect::Io);
    assert_eq!(main.effects[1].effect, Effect::Time);
}

#[test]
fn parse_call_and_string_literal() {
    let src = r#"
fn main() -> Unit {
  log("hi");
  return;
}
"#;

    let p = parse_program(src).expect("parse ok");
    let main = p.funcs.iter().find(|f| f.name.name == "main").unwrap();

    // first stmt is Expr(log("hi");)
    let s0 = &main.body.stmts[0];
    let candy_ast::Stmt::Expr { expr, .. } = s0 else {
        panic!("expected expr stmt");
    };

    match expr {
        Expr::Call { callee, args, .. } => {
            assert_eq!(callee.name, "log");
            assert_eq!(args.len(), 1);
            match &args[0] {
                Expr::StrLit { value, .. } => assert_eq!(value, "hi"),
                _ => panic!("expected string literal arg"),
            }
        }
        _ => panic!("expected call expr"),
    }
}

#[test]
fn parse_call_with_two_args() {
    let src = r#"
fn main() -> Unit {
  f(1, true);
  return;
}
"#;

    let p = parse_program(src).expect("parse ok");
    let main = p.funcs.iter().find(|f| f.name.name == "main").unwrap();

    let candy_ast::Stmt::Expr { expr, .. } = &main.body.stmts[0] else {
        panic!("expected expr stmt");
    };

    match expr {
        Expr::Call { callee, args, .. } => {
            assert_eq!(callee.name, "f");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("expected call expr"),
    }
}
