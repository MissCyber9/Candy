use candy_lexer::{Lexer, TokenKind};

#[test]
fn lex_keywords_and_symbols_with_spans() {
    let src = "fn main() -> Int { return 1; }";
    let toks = Lexer::new("main.candy", src).lex_all();

    assert!(matches!(toks[0].kind, TokenKind::KwFn));
    assert_eq!(toks[0].span.start_line, 1);
    assert_eq!(toks[0].span.start_col, 1);

    // "main" should start at col 4
    assert!(matches!(toks[1].kind, TokenKind::Ident(ref s) if s == "main"));
    assert_eq!(toks[1].span.start_col, 4);

    // "(" right after main => column 8 (fn=2 + space=1 => col3, main starts 4 len 4 => ends at 8, then '(' at col 8)
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::LParen)));

    // arrow token exists
    assert!(toks.iter().any(|t| matches!(t.kind, TokenKind::Arrow)));

    // last token is EOF
    assert!(matches!(toks.last().unwrap().kind, TokenKind::Eof));
}

#[test]
fn lex_spans_across_newlines() {
    let src = "let x = 1;\nreturn x;\n";
    let toks = Lexer::new("main.candy", src).lex_all();

    // First token: let at (1,1)
    assert!(matches!(toks[0].kind, TokenKind::KwLet));
    assert_eq!(toks[0].span.start_line, 1);
    assert_eq!(toks[0].span.start_col, 1);

    // return at line 2 col 1
    let ret = toks
        .iter()
        .find(|t| matches!(t.kind, TokenKind::KwReturn))
        .unwrap();
    assert_eq!(ret.span.start_line, 2);
    assert_eq!(ret.span.start_col, 1);
}

#[test]
fn lex_int_literal_span_is_nonzero() {
    let src = "return 42;";
    let toks = Lexer::new("main.candy", src).lex_all();

    let lit = toks
        .iter()
        .find(|t| matches!(t.kind, TokenKind::IntLit(42)))
        .unwrap();
    assert_eq!(lit.span.start_line, 1);
    assert!(lit.span.start_col > 0);
    // end_col should be >= start_col
    assert!(lit.span.end_col >= lit.span.start_col);
}

#[test]
fn lex_secret_keyword() {
    let src = "let x: secret Int = 1;";
    let toks = candy_lexer::Lexer::new("main.candy", src).lex_all();
    assert!(toks
        .iter()
        .any(|t| matches!(t.kind, candy_lexer::TokenKind::KwSecret)));
}
