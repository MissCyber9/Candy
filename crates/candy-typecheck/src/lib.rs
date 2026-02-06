use std::collections::HashMap;

use candy_ast::{Expr, FnDecl, Program, Stmt, Type};
use candy_diagnostics::{Diagnostic, DiagnosticReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Bool,
    Unit,
    Unknown,
}

fn ty_name(t: &Ty) -> &'static str {
    match t {
        Ty::Int => "Int",
        Ty::Bool => "Bool",
        Ty::Unit => "Unit",
        Ty::Unknown => "Unknown",
    }
}

fn lower_type(t: &Type) -> Ty {
    match t {
        Type::Int { .. } => Ty::Int,
        Type::Bool { .. } => Ty::Bool,
        Type::Unit { .. } => Ty::Unit,
        Type::Named { .. } => Ty::Unknown, // v0.2: only primitives are real
    }
}

pub fn typecheck(p: &Program) -> Result<(), DiagnosticReport> {
    let mut r = DiagnosticReport::new();

    // main() strict
    check_main(p, &mut r);

    // typecheck all functions
    for f in &p.funcs {
        typecheck_fn(f, &mut r);
    }

    if r.is_ok() {
        Ok(())
    } else {
        Err(r)
    }
}

fn check_main(p: &Program, r: &mut DiagnosticReport) {
    let mains: Vec<&FnDecl> = p.funcs.iter().filter(|f| f.name.name == "main").collect();

    if mains.is_empty() {
        r.push(Diagnostic::error(
            "main-missing",
            "Missing `fn main() -> Unit { ... }`.",
            p.span.clone(),
        ));
        return;
    }

    // If multiple mains, warn (not fatal in v0.2, but suspicious)
    if mains.len() > 1 {
        r.push(Diagnostic::warning(
            "main-duplicate",
            "Multiple `main` functions found.",
            mains[1].span.clone(),
        ));
    }

    let m = mains[0];

    if !m.params.is_empty() {
        r.push(Diagnostic::error(
            "main-invalid-signature",
            "main must have zero parameters.",
            m.span.clone(),
        ));
    }

    let ret = lower_type(&m.ret);
    if ret != Ty::Unit {
        r.push(Diagnostic::error(
            "main-invalid-signature",
            "main must return Unit.",
            m.ret.span().clone(),
        ));
    }
}

fn typecheck_fn(f: &FnDecl, r: &mut DiagnosticReport) {
    let ret = lower_type(&f.ret);

    // v0.2: params types must be primitives
    for p in &f.params {
        let pt = lower_type(&p.ty);
        if pt == Ty::Unknown {
            r.push(Diagnostic::error(
                "type-unknown",
                "Unknown parameter type (v0.2 supports Int|Bool|Unit only).",
                p.ty.span().clone(),
            ));
        }
    }

    let mut env: HashMap<String, Ty> = HashMap::new();
    for p in &f.params {
        env.insert(p.name.name.clone(), lower_type(&p.ty));
    }

    for s in &f.body.stmts {
        typecheck_stmt(s, &mut env, &ret, r);
    }
}

fn typecheck_stmt(s: &Stmt, env: &mut HashMap<String, Ty>, ret: &Ty, r: &mut DiagnosticReport) {
    match s {
        Stmt::Let {
            name,
            ty,
            expr,
            span,
        } => {
            let rhs = type_of_expr(expr, env, r);
            if let Some(ann) = ty {
                let at = lower_type(ann);
                if at == Ty::Unknown {
                    r.push(Diagnostic::error(
                        "type-unknown",
                        "Unknown annotated type (v0.2 supports Int|Bool|Unit only).",
                        ann.span().clone(),
                    ));
                } else if rhs != Ty::Unknown && rhs != at {
                    r.push(
                        Diagnostic::error(
                            "type-mismatch",
                            format!(
                                "Type mismatch: expected {}, got {}.",
                                ty_name(&at),
                                ty_name(&rhs)
                            ),
                            ann.span().clone(),
                        )
                        .with_fix(
                            "let x: T = expr;".to_string(),
                            format!("let {}: {} = ...;", name.name, ty_name(&rhs)),
                        ),
                    );
                }
                env.insert(name.name.clone(), at);
            } else {
                env.insert(name.name.clone(), rhs);
            }

            // Use span so it stays referenced (avoid clippy "unused")
            let _ = span;
        }
        Stmt::Return { expr, span } => match (ret, expr) {
            (Ty::Unit, None) => {}
            (Ty::Unit, Some(_e)) => {
                r.push(Diagnostic::error(
                    "return-mismatch",
                    "Return value provided but function returns Unit.",
                    span.clone(),
                ));
            }
            (rt, None) => {
                r.push(Diagnostic::error(
                    "return-mismatch",
                    format!("Missing return value; expected {}.", ty_name(rt)),
                    span.clone(),
                ));
            }
            (rt, Some(e)) => {
                let et = type_of_expr(e, env, r);
                if et != Ty::Unknown && et != *rt {
                    r.push(Diagnostic::error(
                        "return-mismatch",
                        format!(
                            "Return type mismatch: expected {}, got {}.",
                            ty_name(rt),
                            ty_name(&et)
                        ),
                        e.span().clone(),
                    ));
                }
            }
        },
        Stmt::Expr { expr, span } => {
            let _ = type_of_expr(expr, env, r);
            let _ = span;
        }
    }
}

fn type_of_expr(e: &Expr, env: &HashMap<String, Ty>, r: &mut DiagnosticReport) -> Ty {
    match e {
        Expr::IntLit { .. } => Ty::Int,
        Expr::BoolLit { .. } => Ty::Bool,
        Expr::Var { name, .. } => match env.get(&name.name) {
            Some(t) => t.clone(),
            None => {
                r.push(Diagnostic::error(
                    "name-unknown",
                    format!("Unknown name `{}`.", name.name),
                    name.span.clone(),
                ));
                Ty::Unknown
            }
        },
    }
}
