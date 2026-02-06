use candy_lexer::{Lexer, TokenKind};

#[test]
fn lex_protocol_keywords() {
    let src = "protocol P { state Init; transition Init -> Init; }";
    let tokens = Lexer::new("test.candy", src).lex_all();

    let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();

    assert!(kinds.contains(&TokenKind::ProtocolKw));
    assert!(kinds.contains(&TokenKind::StateKw));
    assert!(kinds.contains(&TokenKind::TransitionKw));
}
