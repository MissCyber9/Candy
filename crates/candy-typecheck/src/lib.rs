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

#[derive(Debug, Clone)]
struct VarInfo {
    ty: Ty,
    is_secret: bool,
    moved: bool,
}

fn ty_name(t: &Ty) -> &'static str {
    match t {
        Ty::Int => "Int",
        Ty::Bool => "Bool",
        Ty::Unit => "Unit",
        Ty::Unknown => "Unknown",
    }
}

fn is_secret_type(t: &Type) -> bool {
    matches!(t, Type::Secret { .. })
}

fn lower_type(t: &Type) -> Ty {
    match t {
        Type::Int { .. } => Ty::Int,
        Type::Bool { .. } => Ty::Bool,
        Type::Unit { .. } => Ty::Unit,
        Type::Secret { inner, .. } => lower_type(inner),
        Type::Named { .. } => Ty::Unknown, // v0.3: named types not supported yet
    }
}

pub fn typecheck(p: &Program) -> Result<(), DiagnosticReport> {
    let mut r = DiagnosticReport::new();

    check_main(p, &mut r);

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

    let mut env: HashMap<String, VarInfo> = HashMap::new();

    for p in &f.params {
        let pt = lower_type(&p.ty);
        if pt == Ty::Unknown {
            r.push(Diagnostic::error(
                "type-unknown",
                "Unknown parameter type (v0.3 supports Int|Bool|Unit and secret wrappers).",
                p.ty.span().clone(),
            ));
        }
        env.insert(
            p.name.name.clone(),
            VarInfo {
                ty: pt,
                is_secret: is_secret_type(&p.ty),
                moved: false,
            },
        );
    }

    for s in &f.body.stmts {
        typecheck_stmt(s, &mut env, &ret, r);
    }
}

fn typecheck_stmt(
    s: &Stmt,
    env: &mut HashMap<String, VarInfo>,
    ret: &Ty,
    r: &mut DiagnosticReport,
) {
    match s {
        Stmt::Let {
            name,
            ty,
            expr,
            span: _,
        } => {
            let rhs = type_of_expr(expr, env, r);

            let (ann_ty, ann_secret) = if let Some(ann) = ty {
                let at = lower_type(ann);
                let sec = is_secret_type(ann);
                if at == Ty::Unknown {
                    r.push(Diagnostic::error(
                        "type-unknown",
                        "Unknown annotated type (v0.3 supports Int|Bool|Unit and secret wrappers).",
                        ann.span().clone(),
                    ));
                } else if rhs.ty != Ty::Unknown && rhs.ty != at {
                    r.push(Diagnostic::error(
                        "type-mismatch",
                        format!(
                            "Type mismatch: expected {}, got {}.",
                            ty_name(&at),
                            ty_name(&rhs.ty)
                        ),
                        ann.span().clone(),
                    ));
                }
                (at, sec)
            } else {
                (rhs.ty.clone(), rhs.is_secret)
            };

            // secret-copy rule:
            // if RHS is secret and expression was a Var (not Move), then copying is forbidden.
            if rhs.is_secret && rhs.copied_secret {
                r.push(
                    Diagnostic::error(
                        "secret-copy",
                        format!(
                            "Secret value `{}` cannot be copied. Use move({}) to transfer ownership.",
                            rhs.name_hint.as_deref().unwrap_or("<secret>"),
                            rhs.name_hint.as_deref().unwrap_or("<x>")
                        ),
                        expr.span().clone(),
                    )
                    .with_fix(
                        format!("let {} = {};", name.name, rhs.name_hint.clone().unwrap_or("x".into())),
                        format!("let {} = move({});", name.name, rhs.name_hint.clone().unwrap_or("x".into())),
                    ),
                );
            }

            env.insert(
                name.name.clone(),
                VarInfo {
                    ty: ann_ty,
                    is_secret: ann_secret,
                    moved: false,
                },
            );
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
                if et.ty != Ty::Unknown && et.ty != *rt {
                    r.push(Diagnostic::error(
                        "return-mismatch",
                        format!(
                            "Return type mismatch: expected {}, got {}.",
                            ty_name(rt),
                            ty_name(&et.ty)
                        ),
                        e.span().clone(),
                    ));
                }
            }
        },

        Stmt::Expr { expr, span: _ } => {
            let _ = type_of_expr(expr, env, r);
        }
    }
}

#[derive(Debug, Clone)]
struct ExprTy {
    ty: Ty,
    is_secret: bool,
    // true only when a secret var is used as a plain Var (copy attempt)
    copied_secret: bool,
    // name for better diagnostics/fixes
    name_hint: Option<String>,
}

fn type_of_expr(e: &Expr, env: &mut HashMap<String, VarInfo>, r: &mut DiagnosticReport) -> ExprTy {
    match e {
        Expr::IntLit { .. } => ExprTy {
            ty: Ty::Int,
            is_secret: false,
            copied_secret: false,
            name_hint: None,
        },
        Expr::BoolLit { .. } => ExprTy {
            ty: Ty::Bool,
            is_secret: false,
            copied_secret: false,
            name_hint: None,
        },

        Expr::Var { name, .. } => {
            match env.get(&name.name) {
                Some(v) => {
                    if v.moved {
                        r.push(Diagnostic::error(
                            "use-after-move",
                            format!("Use of `{}` after it was moved.", name.name),
                            name.span.clone(),
                        ));
                        return ExprTy {
                            ty: Ty::Unknown,
                            is_secret: v.is_secret,
                            copied_secret: false,
                            name_hint: Some(name.name.clone()),
                        };
                    }

                    ExprTy {
                        ty: v.ty.clone(),
                        is_secret: v.is_secret,
                        copied_secret: v.is_secret, // var use is a copy attempt for secrets unless wrapped by Move
                        name_hint: Some(name.name.clone()),
                    }
                }
                None => {
                    r.push(Diagnostic::error(
                        "name-unknown",
                        format!("Unknown name `{}`.", name.name),
                        name.span.clone(),
                    ));
                    ExprTy {
                        ty: Ty::Unknown,
                        is_secret: false,
                        copied_secret: false,
                        name_hint: Some(name.name.clone()),
                    }
                }
            }
        }

        Expr::Move { name, .. } => {
            match env.get_mut(&name.name) {
                Some(v) => {
                    if v.moved {
                        r.push(Diagnostic::error(
                            "use-after-move",
                            format!("Value `{}` was already moved.", name.name),
                            name.span.clone(),
                        ));
                        return ExprTy {
                            ty: Ty::Unknown,
                            is_secret: v.is_secret,
                            copied_secret: false,
                            name_hint: Some(name.name.clone()),
                        };
                    }
                    // move consumes ownership (only meaningful for secrets, but allowed for any var)
                    v.moved = true;
                    ExprTy {
                        ty: v.ty.clone(),
                        is_secret: v.is_secret,
                        copied_secret: false,
                        name_hint: Some(name.name.clone()),
                    }
                }
                None => {
                    r.push(Diagnostic::error(
                        "name-unknown",
                        format!("Unknown name `{}`.", name.name),
                        name.span.clone(),
                    ));
                    ExprTy {
                        ty: Ty::Unknown,
                        is_secret: false,
                        copied_secret: false,
                        name_hint: Some(name.name.clone()),
                    }
                }
            }
        }
    }
}
