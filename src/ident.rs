use internment::{Arena, ArenaIntern};


pub struct IdentCache {
    arena: Arena<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident<'a>(ArenaIntern<'a, str>);

impl IdentCache {
    pub fn new() -> IdentCache {
        IdentCache { arena: Arena::new() }
    }

    pub fn new_ident(&self, ident: &str) -> Ident {
        Ident(self.arena.intern(ident))
    }
}
