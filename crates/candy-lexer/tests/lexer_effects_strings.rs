use candy_lexer::{Lexer, TokenKind};

#[test]
fn lex_effects_keyword_and_comma() {
    let src = r#"effects(io, time, rand)"#;
    let toks = Lexer::new("<t>", src).lex_all();

    assert!(matches!(toks[0].kind, TokenKind::KwEffects));
    assert!(matches!(toks[1].kind, TokenKind::LParen));
    assert!(matches!(toks[2].kind, TokenKind::Ident(ref s) if s == "io"));
    assert!(matches!(toks[3].kind, TokenKind::Comma));
    assert!(matches!(toks[4].kind, TokenKind::Ident(ref s) if s == "time"));
    assert!(matches!(toks[5].kind, TokenKind::Comma));
    assert!(matches!(toks[6].kind, TokenKind::Ident(ref s) if s == "rand"));
    assert!(matches!(toks[7].kind, TokenKind::RParen));
    assert!(matches!(toks.last().unwrap().kind, TokenKind::Eof));
}

#[test]
fn lex_string_literal_token_and_span_nonzero() {
    let src = r#"log("hello");"#;
    let toks = Lexer::new("<t>", src).lex_all();

    // log ( "hello" ) ;
    assert!(matches!(toks[0].kind, TokenKind::Ident(ref s) if s == "log"));
    assert!(matches!(toks[1].kind, TokenKind::LParen));
    match &toks[2] {
        t if matches!(t.kind, TokenKind::StrLit(_)) => {
            assert!(t.span.start_line == 1);
            assert!(t.span.start_col >= 1);
            // end should be >= start
            assert!(t.span.end_col >= t.span.start_col);
        }
        _ => panic!("expected StrLit token"),
    }
    assert!(matches!(toks[3].kind, TokenKind::RParen));
    assert!(matches!(toks[4].kind, TokenKind::Semi));
}
