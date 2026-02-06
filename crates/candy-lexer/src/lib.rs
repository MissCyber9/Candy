use candy_diagnostics::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String),
    IntLit(i64),

    KwFn,
    KwLet,
    KwReturn,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semi,
    Eq,
    Arrow, // ->

    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    file: String,
    src: &'a str,
    i: usize,  // byte offset
    line: u32, // 1-based
    col: u32,  // 1-based
}

impl<'a> Lexer<'a> {
    pub fn new(file: impl Into<String>, src: &'a str) -> Self {
        Self {
            file: file.into(),
            src,
            i: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn lex_all(mut self) -> Vec<Token> {
        let mut out = Vec::new();
        loop {
            let t = self.next_token();
            let is_eof = matches!(t.kind, TokenKind::Eof);
            out.push(t);
            if is_eof {
                break;
            }
        }
        out
    }

    fn peek(&self) -> Option<char> {
        self.src[self.i..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        let len = ch.len_utf8();
        self.i += len;

        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn mk_span(&self, sl: u32, sc: u32, el: u32, ec: u32) -> Span {
        Span {
            file: self.file.clone(),
            start_line: sl,
            start_col: sc,
            end_line: el,
            end_col: ec,
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(ch) if ch.is_whitespace()) {
            self.bump();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_ws();

        let sl = self.line;
        let sc = self.col;

        let Some(ch) = self.peek() else {
            return Token {
                kind: TokenKind::Eof,
                span: self.mk_span(sl, sc, sl, sc),
            };
        };

        // Single-char symbols
        match ch {
            '(' => {
                self.bump();
                return Token {
                    kind: TokenKind::LParen,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            ')' => {
                self.bump();
                return Token {
                    kind: TokenKind::RParen,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            '{' => {
                self.bump();
                return Token {
                    kind: TokenKind::LBrace,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            '}' => {
                self.bump();
                return Token {
                    kind: TokenKind::RBrace,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            ':' => {
                self.bump();
                return Token {
                    kind: TokenKind::Colon,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            ';' => {
                self.bump();
                return Token {
                    kind: TokenKind::Semi,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            '=' => {
                self.bump();
                return Token {
                    kind: TokenKind::Eq,
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            '-' => {
                self.bump();
                if self.peek() == Some('>') {
                    self.bump();
                    return Token {
                        kind: TokenKind::Arrow,
                        span: self.mk_span(sl, sc, self.line, self.col),
                    };
                }
                // v0.2 minimal: treat lone '-' as identifier to keep lexer total.
                return Token {
                    kind: TokenKind::Ident("-".into()),
                    span: self.mk_span(sl, sc, self.line, self.col),
                };
            }
            _ => {}
        }

        // int literal
        if ch.is_ascii_digit() {
            let mut s = String::new();
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                s.push(self.bump().unwrap());
            }
            let v = s.parse::<i64>().unwrap_or(0);
            return Token {
                kind: TokenKind::IntLit(v),
                span: self.mk_span(sl, sc, self.line, self.col),
            };
        }

        // identifier / keyword
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut s = String::new();
            while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '_') {
                s.push(self.bump().unwrap());
            }

            let kind = match s.as_str() {
                "fn" => TokenKind::KwFn,
                "let" => TokenKind::KwLet,
                "return" => TokenKind::KwReturn,
                _ => TokenKind::Ident(s),
            };

            return Token {
                kind,
                span: self.mk_span(sl, sc, self.line, self.col),
            };
        }

        // unknown: consume 1 char, return as Ident to avoid lexer failure in v0.2
        self.bump();
        Token {
            kind: TokenKind::Ident(ch.to_string()),
            span: self.mk_span(sl, sc, self.line, self.col),
        }
    }
}
