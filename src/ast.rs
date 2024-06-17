use crate::{ident::IdentName, lexer::Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node<'a> {
    Class(&'a Class<'a>),
    StmtList(&'a StmtList<'a>),
    Stmt(&'a Stmt<'a>),
    Expr(&'a Expr<'a>),
    FuncDef(&'a FuncDef<'a>),
    ParamList(&'a ParamList<'a>),
    VarDef(&'a VarDef<'a>),
    IdentDef(&'a IdentDef<'a>),
    Ident(&'a Ident<'a>),
    Lit(&'a Lit<'a>),
    BinOp(&'a BinOp<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Class<'a> {
    pub span: Span,
    pub stmt_list: &'a StmtList<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StmtList<'a> {
    pub span: Span,
    pub stmts: &'a [&'a Stmt<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stmt<'a> {
    pub span: Span,
    pub kind: StmtKind<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StmtKind<'a> {
    Pass,
    Expr(&'a Expr<'a>),
    VarDef(&'a VarDef<'a>),
    FuncDef(&'a FuncDef<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Expr<'a> {
    pub span: Span,
    pub kind: ExprKind<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExprKind<'a> {
    Ident(&'a Ident<'a>),
    Lit(&'a Lit<'a>),
    BinOp(&'a BinOp<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncDef<'a> {
    pub span: Span,
    pub name: &'a Ident<'a>,
    pub param_list: &'a ParamList<'a>,
    pub result_ty: Option<&'a Expr<'a>>,
    pub body: &'a StmtList<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParamList<'a> {
    pub span: Span,
    pub params: &'a [&'a IdentDef<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarDef<'a> {
    pub span: Span,
    pub def: &'a IdentDef<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentDef<'a> {
    pub span: Span,
    pub name: &'a Ident<'a>,
    pub ty: Option<&'a Expr<'a>>,
    pub val: Option<&'a Expr<'a>>,
    pub strict_type: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident<'a> {
    pub span: Span,
    pub name: IdentName<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lit<'a> {
    pub span: Span,
    pub kind: LitKind<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LitKind<'a> {
    Int(i128),
    Str(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinOp<'a> {
    pub span: Span,
    pub kind: BinOpKind,
    pub lhs: &'a Expr<'a>,
    pub rhs: &'a Expr<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
