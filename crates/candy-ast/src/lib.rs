use candy_diagnostics::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int { span: Span },
    Bool { span: Span },
    Unit { span: Span },
    Secret { inner: Box<Type>, span: Span },
    Named { name: String, span: Span },
}

impl Type {
    pub fn span(&self) -> &Span {
        match self {
            Type::Int { span }
            | Type::Bool { span }
            | Type::Unit { span }
            | Type::Secret { span, .. } => span,
            Type::Named { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Effect {
    Io,
    Net,
    Time,
    Rand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSpec {
    pub effect: Effect,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub funcs: Vec<FnDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDecl {
    pub name: Ident,
    pub params: Vec<Param>,
    pub ret: Type,
    pub effects: Vec<EffectSpec>, // v0.4: default pure when empty
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: Ident,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        name: Ident,
        ty: Option<Type>,
        expr: Expr,
        span: Span,
    },
    Return {
        expr: Option<Expr>,
        span: Span,
    },
    If {
        cond: Expr,
        then_blk: Block,
        else_blk: Option<Block>,
        span: Span,
    },
    Expr {
        expr: Expr,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    IntLit {
        value: i64,
        span: Span,
    },
    BoolLit {
        value: bool,
        span: Span,
    },
    StrLit {
        value: String,
        span: Span,
    },

    Var {
        name: Ident,
        span: Span,
    },
    Move {
        name: Ident,
        span: Span,
    },

    Call {
        callee: Ident,
        args: Vec<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Expr::IntLit { span, .. } => span,
            Expr::BoolLit { span, .. } => span,
            Expr::StrLit { span, .. } => span,
            Expr::Var { span, .. } => span,
            Expr::Move { span, .. } => span,
            Expr::Call { span, .. } => span,
        }
    }
}

pub fn ty_int(span: Span) -> Type {
    Type::Int { span }
}
pub fn ty_bool(span: Span) -> Type {
    Type::Bool { span }
}
pub fn ty_unit(span: Span) -> Type {
    Type::Unit { span }
}
