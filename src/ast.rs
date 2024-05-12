use crate::{ident::IdentName, lexer::Span};

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Program<'a> {
    pub stmt_list: StmtList<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StmtList<'a> {
    pub span: Span,
    pub stmts: Vec<Stmt<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt<'a> {
    pub span: Span,
    pub kind: StmtKind<'a>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind<'a> {
    Pass,
    Expr(Expr<'a>),
    VarDef(VarDef<'a>),
    FuncDef(FuncDef<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr<'a> {
    pub span: Span,
    pub kind: ExprKind<'a>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind<'a> {
    Ident(Ident<'a>),
    Lit(Lit),
    BinOp(BinOp<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDef<'a> {
    pub span: Span,
    pub name: Ident<'a>,
    pub param_list: ParamList<'a>,
    pub result_ty: Option<Expr<'a>>,
    pub body: StmtList<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamList<'a> {
    pub span: Span,
    pub params: Vec<IdentDef<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDef<'a> {
    pub def: IdentDef<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentDef<'a> {
    pub span: Span,
    pub name: Ident<'a>,
    pub ty: TySpec<'a>,
    pub val: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a> {
    pub span: Span,
    pub name: IdentName<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TySpec<'a> {
    Any,
    Inferred,
    Ty(Expr<'a>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    Int(i128),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp<'a> {
    op: BinOpKind,
    lhs: Box<Expr<'a>>,
    rhs: Box<Expr<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add, Sub, Mul, Div,
}
