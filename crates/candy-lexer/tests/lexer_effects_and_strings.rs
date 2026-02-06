use candy_lexer::{Lexer, TokenKind};

#[test]
fn lex_effects_keyword() {
    let src = "effects(io, time)";
    let toks = Lexer::new("<memory>", src).lex_all();
    assert!(matches!(toks[0].kind, TokenKind::KwEffects));
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Comma)));
}

#[test]
fn lex_string_literal_basic() {
    let src = r#"fn main() -> Unit { "hello"; }"#;
    let toks = Lexer::new("<memory>", src).lex_all();
    let has_str = toks
        .iter()
        .any(|t| matches!(t.kind, TokenKind::StrLit(ref s) if s == "hello"));
    assert!(has_str, "expected StrLit(\"hello\") token");
}
