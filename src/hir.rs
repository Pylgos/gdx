use crate::{ident::IdentName, lexer::Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hir<'a> {
    pub classes: Vec<Class<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'a> {
    pub span: Span,
    pub name: Option<Ident<'a>>,
    pub methods: Vec<Method<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method<'a> {
    pub span: Span,
    pub name: Ident<'a>,
    pub signature: Signature<'a>,
    pub body: Expr<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature<'a> {
    pub span: Span,
    pub args: Vec<IdentDef<'a>>,
    pub result_ty: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentDef<'a> {
    pub span: Span,
    pub name: Ident<'a>,
    pub ty: TySpec<'a>,
    pub val: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TySpec<'a> {
    Any,
    Inferred,
    Ty(Expr<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Ident(Ident<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a> {
    pub span: Span,
    pub name: IdentName<'a>
}
