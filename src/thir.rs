use crate::ast::*;


pub mod ty {
    use std::{collections::HashMap, ops::Deref};
    use internment::ArenaIntern;

    use crate::{ast, ident::IdentName};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Ty<'a>(ArenaIntern<'a, TyKind<'a>>);

    impl<'a> Deref for Ty<'a> {
        type Target = TyKind<'a>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a> From<ArenaIntern<'a, TyKind<'a>>> for Ty<'a> {
        fn from(value: ArenaIntern<'a, TyKind<'a>>) -> Self {
            Self(value)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum IntKind {
        I64,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TyKind<'a> {
        Int(IntKind),
        Class(Class<'a>),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Class<'a> {
        pub fields: Vec<Field<'a>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Field<'a> {
        pub name: IdentName<'a>,
        pub ty: Ty<'a>,
        pub ast: &'a ast::VarDef<'a>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FuncDef<'a> {
        pub name: IdentName<'a>,
        pub ret_ty: Ty<'a>,

    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FuncSig<'a> {
        pub params: Vec<IdentTyPair<'a>>,
        pub ret_ty: Ty<'a>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct IdentTyPair<'a> {
        pub ident: IdentName<'a>,
        pub ty: Ty<'a>,
    }
}

