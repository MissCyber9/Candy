use candy_ast::{Block, Expr, FnDecl, Ident, Param, Program, Stmt, Type};
use candy_diagnostics::{Diagnostic, DiagnosticReport, Span};
use candy_lexer::{Lexer, Token, TokenKind};

/// v0.1 compatibility: parse from memory buffer.
pub fn parse_program(src: &str) -> Result<Program, DiagnosticReport> {
    parse_file("<memory>", src)
}

/// v0.2 canonical entrypoint: parse a file with spans.
pub fn parse_file(file: &str, src: &str) -> Result<Program, DiagnosticReport> {
    let mut p = Parser::new(file, src);
    let prog = p.parse_program();
    if p.report.is_ok() {
        Ok(prog)
    } else {
        Err(p.report)
    }
}

struct Parser<'a> {
    file: String,
    lx: Lexer<'a>,
    cur: Token,
    report: DiagnosticReport,
}

impl<'a> Parser<'a> {
    fn new(file: &str, src: &'a str) -> Self {
        let mut lx = Lexer::new(file.to_string(), src);
        let cur = lx.next_token();
        Self {
            file: file.to_string(),
            lx,
            cur,
            report: DiagnosticReport::new(),
        }
    }

    fn bump(&mut self) {
        self.cur = self.lx.next_token();
    }

    fn err(&mut self, code: &str, msg: &str, span: Span) {
        self.report.push(Diagnostic::error(code, msg, span));
    }

    fn expect_kind(&mut self, expected: TokenKind, code: &str, msg: &str) -> Option<Span> {
        if self.cur.kind == expected {
            let sp = self.cur.span.clone();
            self.bump();
            Some(sp)
        } else {
            self.err(code, msg, self.cur.span.clone());
            None
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut funcs = Vec::new();
        let start = Span::unknown(self.file.clone());

        while self.cur.kind != TokenKind::Eof {
            funcs.push(self.parse_fn());
        }

        Program { funcs, span: start }
    }

    fn parse_fn(&mut self) -> FnDecl {
        let fn_span = match self.cur.kind {
            TokenKind::KwFn => {
                let sp = self.cur.span.clone();
                self.bump();
                sp
            }
            _ => {
                let sp = self.cur.span.clone();
                self.err("parse-expected-fn", "Expected `fn`.", sp.clone());
                // recovery: consume one token, pretend fn starts here
                self.bump();
                sp
            }
        };

        let name = self.parse_ident("parse-expected-ident", "Expected function name identifier.");

        self.expect_kind(
            TokenKind::LParen,
            "parse-expected-lparen",
            "Expected `(` after function name.",
        );

        let params = self.parse_params();

        self.expect_kind(
            TokenKind::RParen,
            "parse-expected-rparen",
            "Expected `)` after parameters.",
        );

        self.expect_kind(
            TokenKind::Arrow,
            "parse-expected-arrow",
            "Expected `->` return type.",
        );

        let ret = self.parse_type();

        let body = self.parse_block();

        FnDecl {
            name,
            params,
            ret,
            body,
            span: fn_span,
        }
    }

    fn parse_params(&mut self) -> Vec<Param> {
        // v0.2 minimal: either empty list or a single param `ident : Type`
        if self.cur.kind == TokenKind::RParen {
            return vec![];
        }

        let name = self.parse_ident("parse-expected-ident", "Expected parameter name.");
        self.expect_kind(
            TokenKind::Colon,
            "parse-expected-colon",
            "Expected `:` after parameter name.",
        );
        let ty = self.parse_type();
        let sp = name.span.clone();
        vec![Param { name, ty, span: sp }]
    }

    fn parse_block(&mut self) -> Block {
        let lbrace = self
            .expect_kind(
                TokenKind::LBrace,
                "parse-expected-lbrace",
                "Expected `{` to start block.",
            )
            .unwrap_or_else(|| Span::unknown(self.file.clone()));

        let mut stmts = vec![];
        while self.cur.kind != TokenKind::RBrace && self.cur.kind != TokenKind::Eof {
            stmts.push(self.parse_stmt());
        }

        self.expect_kind(
            TokenKind::RBrace,
            "parse-expected-rbrace",
            "Expected `}` to end block.",
        );

        Block {
            stmts,
            span: lbrace,
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.cur.kind {
            TokenKind::KwLet => self.parse_let(),
            TokenKind::KwReturn => self.parse_return(),
            TokenKind::KwIf => self.parse_if(),
            _ => {
                let expr = self.parse_expr();
                let semi = self
                    .expect_kind(
                        TokenKind::Semi,
                        "parse-expected-semi",
                        "Expected `;` after expression.",
                    )
                    .unwrap_or_else(|| expr.span().clone());
                Stmt::Expr { expr, span: semi }
            }
        }
    }

    fn parse_let(&mut self) -> Stmt {
        let let_span = self.cur.span.clone();
        self.bump();

        let name = self.parse_ident("parse-expected-ident", "Expected identifier after `let`.");

        let mut ty = None;
        if self.cur.kind == TokenKind::Colon {
            self.bump();
            ty = Some(self.parse_type());
        }

        self.expect_kind(
            TokenKind::Eq,
            "parse-expected-eq",
            "Expected `=` in let binding.",
        );

        let expr = self.parse_expr();

        self.expect_kind(
            TokenKind::Semi,
            "parse-expected-semi",
            "Expected `;` after let binding.",
        );

        Stmt::Let {
            name,
            ty,
            expr,
            span: let_span,
        }
    }

    fn parse_return(&mut self) -> Stmt {
        let ret_span = self.cur.span.clone();
        self.bump();

        let expr = if self.cur.kind == TokenKind::Semi {
            None
        } else {
            Some(self.parse_expr())
        };

        self.expect_kind(
            TokenKind::Semi,
            "parse-expected-semi",
            "Expected `;` after return.",
        );

        Stmt::Return {
            expr,
            span: ret_span,
        }
    }

    fn parse_if(&mut self) -> Stmt {
        let if_span = self.cur.span.clone();
        self.bump(); // consume `if`

        self.expect_kind(
            TokenKind::LParen,
            "parse-expected-lparen",
            "Expected `(` after if.",
        );

        let cond = self.parse_expr();

        self.expect_kind(
            TokenKind::RParen,
            "parse-expected-rparen",
            "Expected `)` after if condition.",
        );

        let then_blk = self.parse_block();

        let else_blk = if self.cur.kind == TokenKind::KwElse {
            self.bump(); // consume `else`
            Some(self.parse_block())
        } else {
            None
        };

        Stmt::If {
            cond,
            then_blk,
            else_blk,
            span: if_span,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match &self.cur.kind {
            TokenKind::IntLit(v) => {
                let sp = self.cur.span.clone();
                let vv = *v;
                self.bump();
                Expr::IntLit {
                    value: vv,
                    span: sp,
                }
            }
            TokenKind::Ident(s) if s == "true" || s == "false" => {
                let sp = self.cur.span.clone();
                let b = s == "true";
                self.bump();
                Expr::BoolLit { value: b, span: sp }
            }
            TokenKind::Ident(s) if s == "move" => {
                // move(<ident>)
                let move_span = self.cur.span.clone();
                self.bump(); // consume "move"

                self.expect_kind(
                    TokenKind::LParen,
                    "parse-expected-lparen",
                    "Expected `(` after move.",
                );

                let name = self.parse_ident(
                    "parse-expected-ident",
                    "Expected identifier inside move(...).",
                );

                self.expect_kind(
                    TokenKind::RParen,
                    "parse-expected-rparen",
                    "Expected `)` after move argument.",
                );

                Expr::Move {
                    name,
                    span: move_span,
                }
            }
            TokenKind::Ident(_) => {
                let id = self.parse_ident("parse-expected-ident", "Expected identifier.");
                let sp = id.span.clone();
                Expr::Var { name: id, span: sp }
            }
            _ => {
                let sp = self.cur.span.clone();
                self.err(
                    "parse-unexpected-token",
                    "Unexpected token in expression.",
                    sp.clone(),
                );
                self.bump();
                Expr::Var {
                    name: Ident {
                        name: "_error_".into(),
                        span: sp.clone(),
                    },
                    span: sp,
                }
            }
        }
    }

    fn parse_type(&mut self) -> Type {
        match &self.cur.kind {
            TokenKind::KwSecret => {
                let sp = self.cur.span.clone();
                self.bump(); // consume `secret`
                let inner = self.parse_type();
                Type::Secret {
                    inner: Box::new(inner),
                    span: sp,
                }
            }
            TokenKind::Ident(s) => {
                let sp = self.cur.span.clone();
                let name = s.clone();
                self.bump();
                match name.as_str() {
                    "Int" => Type::Int { span: sp },
                    "Bool" => Type::Bool { span: sp },
                    "Unit" => Type::Unit { span: sp },
                    _ => Type::Named { name, span: sp },
                }
            }
            _ => {
                let sp = self.cur.span.clone();
                self.err(
                    "parse-expected-type",
                    "Expected type name (Int|Bool|Unit|...).",
                    sp.clone(),
                );
                self.bump();
                Type::Named {
                    name: "_error_".into(),
                    span: sp,
                }
            }
        }
    }

    fn parse_ident(&mut self, code: &str, msg: &str) -> Ident {
        match &self.cur.kind {
            TokenKind::Ident(s) => {
                let sp = self.cur.span.clone();
                let name = s.clone();
                self.bump();
                Ident { name, span: sp }
            }
            _ => {
                let sp = self.cur.span.clone();
                self.err(code, msg, sp.clone());
                self.bump();
                Ident {
                    name: "_error_".into(),
                    span: sp,
                }
            }
        }
    }
}
