use std::collections::{BTreeSet, HashMap};

use candy_ast::{Effect, Expr, FnDecl, Program, Stmt, Type};
use candy_diagnostics::{Diagnostic, DiagnosticReport, Span};

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
        Type::Named { .. } => Ty::Unknown,
    }
}

fn effect_name(e: Effect) -> &'static str {
    match e {
        Effect::Io => "io",
        Effect::Net => "net",
        Effect::Time => "time",
        Effect::Rand => "rand",
    }
}

fn fmt_effects_list(effs: &BTreeSet<Effect>) -> String {
    let mut v: Vec<&'static str> = effs.iter().map(|e| effect_name(*e)).collect();
    v.sort();
    v.join(", ")
}

fn effects_set_of_fn(f: &FnDecl) -> BTreeSet<Effect> {
    let mut set = BTreeSet::new();
    for s in &f.effects {
        set.insert(s.effect);
    }
    set
}

fn pretty_ret(t: &Type) -> &'static str {
    match t {
        Type::Int { .. } => "Int",
        Type::Bool { .. } => "Bool",
        Type::Unit { .. } => "Unit",
        Type::Secret { .. } => "secret ...",
        Type::Named { .. } => "...",
    }
}

fn make_effects_fix(f: &FnDecl, proposed: &BTreeSet<Effect>) -> (String, String) {
    let replace = format!("fn {}(...) -> {} {{", f.name.name, pretty_ret(&f.ret));
    let with = format!(
        "fn {}(...) -> {} effects({}) {{",
        f.name.name,
        pretty_ret(&f.ret),
        fmt_effects_list(proposed)
    );
    (replace, with)
}

pub fn typecheck(p: &Program) -> Result<(), DiagnosticReport> {
    let mut r = DiagnosticReport::new();

    let mut fn_effects: HashMap<String, BTreeSet<Effect>> = HashMap::new();
    for f in &p.funcs {
        fn_effects.insert(f.name.name.clone(), effects_set_of_fn(f));
    }

    check_main(p, &mut r);

    typecheck_protocols(&p.protocols, &mut r);

    for f in &p.funcs {
        typecheck_fn(f, &fn_effects, &mut r);
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

fn typecheck_fn(
    f: &FnDecl,
    fn_effects: &HashMap<String, BTreeSet<Effect>>,
    r: &mut DiagnosticReport,
) {
    let ret = lower_type(&f.ret);
    let current_effects = effects_set_of_fn(f);

    let mut env: HashMap<String, VarInfo> = HashMap::new();

    for p in &f.params {
        let pt = lower_type(&p.ty);
        if pt == Ty::Unknown {
            r.push(Diagnostic::error(
                "type-unknown",
                "Unknown parameter type (Candy supports Int|Bool|Unit and secret wrappers).",
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
        typecheck_stmt(s, &mut env, &ret, &current_effects, f, fn_effects, r);
    }
}

fn typecheck_stmt(
    s: &Stmt,
    env: &mut HashMap<String, VarInfo>,
    ret: &Ty,
    current_effects: &BTreeSet<Effect>,
    current_fn: &FnDecl,
    fn_effects: &HashMap<String, BTreeSet<Effect>>,
    r: &mut DiagnosticReport,
) {
    match s {
        Stmt::Let { name, ty, expr, .. } => {
            let rhs = type_of_expr(expr, env, current_effects, current_fn, fn_effects, r);

            let (ann_ty, ann_secret) = if let Some(ann) = ty {
                let at = lower_type(ann);
                let sec = is_secret_type(ann);
                if at == Ty::Unknown {
                    r.push(Diagnostic::error(
                        "type-unknown",
                        "Unknown annotated type (Candy supports Int|Bool|Unit and secret wrappers).",
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
                        format!(
                            "let {} = {};",
                            name.name,
                            rhs.name_hint.clone().unwrap_or("x".into())
                        ),
                        format!(
                            "let {} = move({});",
                            name.name,
                            rhs.name_hint.clone().unwrap_or("x".into())
                        ),
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
            (Ty::Unit, Some(_)) => {
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
                let et = type_of_expr(e, env, current_effects, current_fn, fn_effects, r);
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

        Stmt::If {
            cond,
            then_blk,
            else_blk,
            ..
        } => {
            let ct = type_of_expr(cond, env, current_effects, current_fn, fn_effects, r);

            if ct.ty != Ty::Bool && ct.ty != Ty::Unknown {
                r.push(Diagnostic::error(
                    "if-cond-not-bool",
                    format!("If condition must be Bool, got {}.", ty_name(&ct.ty)),
                    cond.span().clone(),
                ));
            }

            if ct.is_secret {
                r.push(Diagnostic::error(
                    "secret-branch",
                    "Branching on secret data is forbidden.",
                    cond.span().clone(),
                ));
            }

            for st in &then_blk.stmts {
                typecheck_stmt(st, env, ret, current_effects, current_fn, fn_effects, r);
            }

            if let Some(eb) = else_blk {
                for st in &eb.stmts {
                    typecheck_stmt(st, env, ret, current_effects, current_fn, fn_effects, r);
                }
            }
        }

        Stmt::Expr { expr, .. } => {
            let _ = type_of_expr(expr, env, current_effects, current_fn, fn_effects, r);
        }
    }
}

#[derive(Debug, Clone)]
struct ExprTy {
    ty: Ty,
    is_secret: bool,
    copied_secret: bool,
    name_hint: Option<String>,
}

fn require_effect(
    required: Effect,
    call_site: Span,
    current_effects: &BTreeSet<Effect>,
    current_fn: &FnDecl,
    r: &mut DiagnosticReport,
) {
    if current_effects.contains(&required) {
        return;
    }

    let mut proposed = current_effects.clone();
    proposed.insert(required);

    let (replace, with) = make_effects_fix(current_fn, &proposed);

    r.push(
        Diagnostic::error(
            "undeclared-effect",
            format!(
                "Operation requires effect `{}`; add it to function `{}`.",
                effect_name(required),
                current_fn.name.name
            ),
            call_site,
        )
        .with_fix(replace, with),
    );
}

fn require_effects_for_call(
    callee: &str,
    call_site: Span,
    current_effects: &BTreeSet<Effect>,
    current_fn: &FnDecl,
    fn_effects: &HashMap<String, BTreeSet<Effect>>,
    r: &mut DiagnosticReport,
) {
    let Some(needed) = fn_effects.get(callee) else {
        return;
    };

    let mut missing = BTreeSet::new();
    for e in needed {
        if !current_effects.contains(e) {
            missing.insert(*e);
        }
    }
    if missing.is_empty() {
        return;
    }

    let mut proposed = current_effects.clone();
    for e in &missing {
        proposed.insert(*e);
    }

    let (replace, with) = make_effects_fix(current_fn, &proposed);

    r.push(
        Diagnostic::error(
            "effect-leak",
            format!(
                "Calling `{}` requires effects ({}) in `{}`.",
                callee,
                fmt_effects_list(needed),
                current_fn.name.name
            ),
            call_site,
        )
        .with_fix(replace, with),
    );
}

fn type_of_expr(
    e: &Expr,
    env: &mut HashMap<String, VarInfo>,
    current_effects: &BTreeSet<Effect>,
    current_fn: &FnDecl,
    fn_effects: &HashMap<String, BTreeSet<Effect>>,
    r: &mut DiagnosticReport,
) -> ExprTy {
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
        Expr::StrLit { .. } => ExprTy {
            ty: Ty::Unknown,
            is_secret: false,
            copied_secret: false,
            name_hint: None,
        },

        Expr::Var { name, .. } => match env.get(&name.name) {
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
                    copied_secret: v.is_secret,
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
        },

        Expr::Move { name, .. } => match env.get_mut(&name.name) {
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
        },

        Expr::Call { callee, args, span } => {
            match callee.name.as_str() {
                "log" => {
                    require_effect(Effect::Io, span.clone(), current_effects, current_fn, r);
                    if args.len() != 1 {
                        r.push(Diagnostic::error(
                            "call-arity",
                            "log expects exactly 1 argument.",
                            callee.span.clone(),
                        ));
                    }
                    for a in args {
                        let _ = type_of_expr(a, env, current_effects, current_fn, fn_effects, r);
                    }
                    return ExprTy {
                        ty: Ty::Unit,
                        is_secret: false,
                        copied_secret: false,
                        name_hint: None,
                    };
                }
                "now" => {
                    require_effect(Effect::Time, span.clone(), current_effects, current_fn, r);
                    if !args.is_empty() {
                        r.push(Diagnostic::error(
                            "call-arity",
                            "now expects 0 arguments.",
                            callee.span.clone(),
                        ));
                    }
                    return ExprTy {
                        ty: Ty::Int,
                        is_secret: false,
                        copied_secret: false,
                        name_hint: None,
                    };
                }
                "rand" => {
                    require_effect(Effect::Rand, span.clone(), current_effects, current_fn, r);
                    if !args.is_empty() {
                        r.push(Diagnostic::error(
                            "call-arity",
                            "rand expects 0 arguments.",
                            callee.span.clone(),
                        ));
                    }
                    return ExprTy {
                        ty: Ty::Int,
                        is_secret: false,
                        copied_secret: false,
                        name_hint: None,
                    };
                }
                _ => {}
            }

            if !fn_effects.contains_key(&callee.name) {
                r.push(Diagnostic::error(
                    "name-unknown",
                    format!("Unknown name `{}`.", callee.name),
                    callee.span.clone(),
                ));
            } else {
                require_effects_for_call(
                    &callee.name,
                    span.clone(),
                    current_effects,
                    current_fn,
                    fn_effects,
                    r,
                );
            }

            for a in args {
                let _ = type_of_expr(a, env, current_effects, current_fn, fn_effects, r);
            }

            ExprTy {
                ty: Ty::Unknown,
                is_secret: false,
                copied_secret: false,
                name_hint: None,
            }
        }
    }
}

fn typecheck_protocols(protocols: &[candy_ast::ProtocolDecl], r: &mut DiagnosticReport) {
    use std::collections::{BTreeMap, BTreeSet, VecDeque};

    for proto in protocols {
        // ---- collect states + duplicate detection ----
        let mut states: BTreeSet<String> = BTreeSet::new();

        for st in &proto.states {
            let name = st.name.name.clone();
            if !states.insert(name.clone()) {
                r.push(Diagnostic::error(
                    "protocol-duplicate-state",
                    format!(
                        "Duplicate state `{}` in protocol `{}`.",
                        name, proto.name.name
                    ),
                    st.name.span.clone(),
                ));
            }
        }

        // ---- empty protocol ----
        if states.is_empty() {
            r.push(Diagnostic::error(
                "protocol-empty",
                format!("Protocol `{}` declares no states.", proto.name.name),
                proto.span.clone(),
            ));
            continue; // nothing else to do safely
        }

        // ---- init rule (v0.5.x convention) ----
        let init = "Init".to_string();
        if !states.contains(&init) {
            r.push(Diagnostic::error(
                "protocol-missing-init",
                format!("Protocol `{}` must declare state `Init`.", proto.name.name),
                proto.span.clone(),
            ));
        }

        // ---- transitions: unknown state + duplicate edge ----
        let mut seen_edges: BTreeSet<(String, String)> = BTreeSet::new();
        let mut out_deg: BTreeMap<String, usize> = BTreeMap::new();
        let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for st in states.iter() {
            out_deg.insert(st.clone(), 0);
            adj.insert(st.clone(), Vec::new());
        }

        for tr in &proto.transitions {
            let from = tr.from.name.clone();
            let to = tr.to.name.clone();

            if !states.contains(&from) {
                r.push(Diagnostic::error(
                    "protocol-unknown-state",
                    format!(
                        "Transition references unknown state `{}` in protocol `{}`.",
                        from, proto.name.name
                    ),
                    tr.from.span.clone(),
                ));
            }

            if !states.contains(&to) {
                r.push(Diagnostic::error(
                    "protocol-unknown-state",
                    format!(
                        "Transition references unknown state `{}` in protocol `{}`.",
                        to, proto.name.name
                    ),
                    tr.to.span.clone(),
                ));
            }

            let edge = (from.clone(), to.clone());
            if !seen_edges.insert(edge) {
                r.push(Diagnostic::error(
                    "protocol-duplicate-transition",
                    format!(
                        "Duplicate transition `{}` -> `{}` in protocol `{}`.",
                        from, to, proto.name.name
                    ),
                    tr.span.clone(),
                ));
            }

            // Only build graph for well-formed endpoints
            if states.contains(&from) && states.contains(&to) {
                if let Some(v) = out_deg.get_mut(&from) {
                    *v += 1;
                }
                if let Some(v) = adj.get_mut(&from) {
                    v.push(to);
                }
            }
        }

        // ---- reachability (unreachable states) ----
        if states.contains(&init) {
            let mut seen: BTreeSet<String> = BTreeSet::new();
            let mut q: VecDeque<String> = VecDeque::new();
            seen.insert(init.clone());
            q.push_back(init.clone());

            while let Some(cur) = q.pop_front() {
                let nexts = adj.get(&cur).cloned().unwrap_or_default();
                for n in nexts {
                    if seen.insert(n.clone()) {
                        q.push_back(n);
                    }
                }
            }

            for st in states.iter() {
                if st != &init && !seen.contains(st) {
                    r.push(Diagnostic::error(
                        "protocol-unreachable-state",
                        format!(
                            "State `{}` is unreachable from `Init` in protocol `{}`.",
                            st, proto.name.name
                        ),
                        proto.span.clone(),
                    ));
                }
            }
        }

        // ---- dead-end states (no outgoing transitions) ----
        for st in states.iter() {
            let deg = *out_deg.get(st).unwrap_or(&0);
            if deg == 0 {
                r.push(Diagnostic::error(
                    "protocol-dead-end-state",
                    format!(
                        "State `{}` has no outgoing transitions in protocol `{}`.",
                        st, proto.name.name
                    ),
                    proto.span.clone(),
                ));
            }
        }
    }
}
