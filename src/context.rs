use crate::{ident::{IdentCache, IdentName}};



pub struct Ctx {
    arena: bumpalo::Bump,
    ident_cache: IdentCache,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            arena: bumpalo::Bump::new(),
            ident_cache: IdentCache::new(),
        }
    }

    pub fn alloc<T: Sized>(&self, val: T) -> &T {
        self.arena.alloc(val)
    }

    pub fn alloc_slice_copy<'a, T: Copy>(&'a self, src: &[T]) -> &'a [T] {
        self.arena.alloc_slice_copy(src)
    }

    pub fn new_ident_name(&self, s: &str) -> IdentName {
        self.ident_cache.new_ident_name(s)
    }
}
