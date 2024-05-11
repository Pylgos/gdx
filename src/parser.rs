use std::ops::Range;

use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{ast::Program, ident::IdentCache, lexer::{self, Token, TokenKind}};
lalrpop_mod!(gdx);

struct Ctx<'a> {
    src: &'a str,
    ident_cache: &'a IdentCache,
}

impl<'a> Ctx<'a> {
    fn src(&self, r: Range<u32>) -> &str {
        &self.src[r.start as usize .. r.end as usize]
    }
}

pub fn parse<'a>(source: &'a str, tokens: &[Token], ident_cache: &'a IdentCache) -> Result<Program<'a>, ParseError<u32, TokenKind, ()>> {
    let stream = tokens
        .iter()
        .map(|tok| -> Result<_, ()> { Ok((tok.span.start, tok.kind, tok.span.end)) });
    let parser = gdx::ProgramParser::new();
    let ctx = Ctx { src: source, ident_cache };
    parser.parse(&ctx, stream)
}

#[cfg(test)]
mod test {
    use crate::ident::IdentCache;

    use super::*;

    fn parse_source(source: &str) {
        let (tokens, _errors) = lexer::tokenize(source);
        let ident_cache = IdentCache::new();
        parse(source, &tokens, &ident_cache);
    }

    #[test]
    fn test() {
        parse_source("1");
    }
}