use candy_lexer::{Lexer, TokenKind};

#[test]
fn lex_protocol_final_keyword() {
    let src = "protocol P { final state Done; }";
    let tokens = Lexer::new("test.candy", src).lex_all();

    let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
    assert!(kinds.contains(&TokenKind::FinalKw));
    assert!(kinds.contains(&TokenKind::StateKw));
    assert!(kinds.contains(&TokenKind::ProtocolKw));
}
