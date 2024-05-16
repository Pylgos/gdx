use std::ops::Range;

use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{ast::Class, context, ident::{IdentCache, IdentName}, lexer::{self, Span, Token, TokenKind}};
lalrpop_mod!(gdx);

struct Ctx<'a, 'src> {
    src: &'src str,
    main_ctx: &'a context::Ctx,
}

impl<'a, 'src> Ctx<'a, 'src> {
    fn src(&self, r: Range<u32>) -> &'src str {
        &self.src[r.start as usize .. r.end as usize]
    }

    fn span(&self, from: u32, to: u32) -> Span {
        Span::new(from, to)
    }

    fn new_ident_name(&self, s: &str) -> IdentName<'a> {
        self.main_ctx.new_ident_name(s)
    }

    fn alloc<T: Sized>(&self, val: T) -> &'a T {
        self.main_ctx.alloc(val)
    }

    pub fn slice<T: Copy>(&self, src: &[T]) -> &'a [T] {
        self.main_ctx.alloc_slice_copy(src)
    }
}

pub fn parse<'a, 'src>(source: &'src str, tokens: &[Token], ctx: &'a context::Ctx) -> Result<&'a Class<'a>, ParseError<u32, TokenKind, ()>> {
    let stream = tokens
        .iter()
        .map(|tok| -> Result<_, ()> { Ok((tok.span.start, tok.kind, tok.span.end)) });
    let parser = gdx::ClassParser::new();
    let ctx = Ctx { src: source, main_ctx: ctx };
    parser.parse(&ctx, stream)
}

#[cfg(test)]
mod test {
    use crate::ident::IdentCache;

    use super::*;

    fn parse_source(source: &str) {
        let (tokens, _errors) = lexer::tokenize(source);
        let ctx = context::Ctx::new();
        parse(source, &tokens, &ctx);
    }

    #[test]
    fn test() {
        parse_source("1");
    }
}