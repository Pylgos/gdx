use internment::{Arena, ArenaIntern};


pub struct IdentCache {
    arena: Arena<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentName<'a>(ArenaIntern<'a, str>);

impl IdentCache {
    pub fn new() -> IdentCache {
        IdentCache { arena: Arena::new() }
    }

    pub fn new_ident_name(&self, ident: &str) -> IdentName {
        IdentName(self.arena.intern(ident))
    }
}
