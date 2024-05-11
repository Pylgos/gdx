use crate::ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Program<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Stmt<'a> {
    Expr(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Ident(Ident<'a>),
    Lit(Lit),
    BinOp(BinOp<'a>),
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
