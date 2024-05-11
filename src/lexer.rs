use std::str::CharIndices;
use unicode_ident::{is_xid_start, is_xid_continue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Span {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError {
    span: Span,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TokenStorage {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
}

pub fn tokenize(source: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut lex = Lexer::new(source);
    lex.tokenize();
    (lex.tokens, lex.errors)
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
    current: u32,
    token_start: Option<u32>,
}

impl <'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source,
            chars: source.char_indices(),
            tokens: Vec::new(),
            errors: Vec::new(),
            current: 0,
            token_start: None,
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next().map(|(i, c)| {
            self.current = i as _;
            c
        })
    }

    // fn prev_token_mut(&mut self) -> Option<&mut Result<Token, LexError>> {
    //     self.tokens.last_mut()
    // }

    fn prev_error_mut(&mut self) -> Option<&mut LexError> {
        self.errors.last_mut()
    }

    fn first(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next().map(|(_, c)| c)
    }

    // fn second(&self) -> Option<char> {
    //     let mut iter = self.chars.clone();
    //     iter.next();
    //     iter.next().map(|(_, c)| c)
    // }

    // fn third(&self) -> Option<char> {
    //     let mut iter = self.chars.clone();
    //     iter.next();
    //     iter.next();
    //     iter.next().map(|(_, c)| c)
    // }

    fn bump_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        loop {
            match self.first() {
                Some(ch) => {
                    if !pred(ch) {
                        return;
                    } else {
                        self.bump();
                    }
                }
                None => return,
            }
        }
    }

    fn begin_token(&mut self) {
        self.token_start = Some(self.current);
    }

    fn end_token_incl(&mut self, kind: TokenKind) {
        let span = Span::new(self.token_start.unwrap(), self.current + 1);
        let token = Token::new(kind, span);
        self.tokens.push(token);
    }

    // fn end_token_excl(&mut self, kind: TokenKind) {
    //     let span = Span::new(self.token_start.unwrap(), self.current);
    //     let tok = Token::new(kind, span);
    //     self.tokens.push(tok);
    // }

    fn token_str_incl(&self) -> &str {
        &self.source[(self.token_start.unwrap() as usize) ..= (self.current as usize)]
    }

    fn single_char_token(&mut self, kind: TokenKind) {
        self.begin_token();
        self.end_token_incl(kind);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    IntLit,
    StrLit,
    
    Plus,
    Minus,
    Mul,
    Div,

    PlusEq,
    MinusEq,
    MulEq,
    DivEq,

    ParLe,
    ParRi,
    Newline,

    Eof,
}

impl <'a> Lexer<'a> {
    pub fn tokenize(&mut self) {
        use TokenKind::*;
        loop {
            let Some(first_char) = self.bump() else {
                self.tokens.push(Token::new(Eof, Span::new(self.current + 1, self.current + 1)));
                return;
            };
            match first_char {
                c if is_xid_start(c) => {
                    self.begin_token();
                    self.bump_while(|c| is_xid_continue(c));
                    self.end_token_incl(Ident);
                }
                ' ' | '\t' => (),
                '+' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(PlusEq);
                    } else {
                        self.end_token_incl(Plus);
                    }
                }
                '-' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(MinusEq);
                    } else {
                        self.end_token_incl(Minus);
                    }
                }
                '*' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(MulEq);
                    } else {
                        self.end_token_incl(Mul);
                    }
                }
                '/' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(DivEq);
                    } else {
                        self.end_token_incl(Div);
                    }
                }
                '0' => {
                    self.begin_token();
                    match self.bump() {
                        Some(_) | None => {
                            self.end_token_incl(IntLit);
                        }
                    }
                }
                '1'..='9' => {
                    self.begin_token();
                    self.bump_while(|c| ('0'..'9').contains(&c));
                    match i128::from_str_radix(self.token_str_incl(), 10) {
                        Ok(val) => {
                            self.end_token_incl(IntLit);
                        }
                        Err(_) => {

                        }
                    }
                }
                '(' => self.single_char_token(ParLe),
                ')' => self.single_char_token(ParRi),
                _ => {
                    let cur = self.current;
                    if let Some(e) = self.prev_error_mut() {
                        if e.span.end == cur {
                            e.span.end = cur + 1;
                        }
                    } else {
                        self.errors.push(LexError { span: Span::new(self.current, self.current + 1) });
                    }
                }
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        macro_rules! test {
            ($src:expr; $($tok:expr, $start:expr, $end:expr);* $(;)?) => {
                {
                    let (tokens, errors) = tokenize($src);
                    assert_eq!(tokens, vec![
                        $(
                            Token::new($tok, Span::new($start, $end)),
                        )*
                    ]);
                    assert_eq!(errors, vec![]);
                }
            };
        }
        use TokenKind::*;
        test!(
            "";
            Eof, 1, 1;
        );
        test!(
            "hello0123";
            Ident, 0, 9;
            Eof, 9, 9;
        );
        test!(
            "0";
            IntLit, 0, 1;
            Eof, 1, 1;
        );
        test!(
            "2147483647";
            IntLit, 0, 10;
            Eof, 10, 10;
        );
        test!(
            "( 100 )";
            ParLe, 0, 1;
            IntLit, 2, 5;
            ParRi, 6, 7;
            Eof, 7, 7;
        );
        test!(
            "1 + 2 - 3 * 4 / 5";
            IntLit, 0, 1;
            Plus, 2, 3;
            IntLit, 4, 5;
            Minus, 6, 7;
            IntLit, 8, 9;
            Mul, 10, 11;
            IntLit, 12, 13;
            Div, 14, 15;
            IntLit, 16, 17;
            Eof, 17, 17;
        )
    }
}
