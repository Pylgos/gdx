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

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{:}:{:}", self.kind, self.span.start, self.span.end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    span: Span,
    kind: LexErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnexpectedEof,
    OddIndentation,
    InconsistentIndentation,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndentKind {
    Tab,
    Space
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Indentation {
    elements: Vec<IndentKind>,
}

impl Indentation {
    fn new(elements: Vec<IndentKind>) -> Self {
        assert_ne!(elements.len(), 0);
        Self { elements }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
    current_pos: u32,
    current_char: Option<char>,
    token_start: Option<u32>,
    indent_stack: Vec<Indentation>,
    nesting: u32,
}

impl <'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source,
            chars: source.char_indices(),
            tokens: Vec::new(),
            errors: Vec::new(),
            current_pos: 0,
            current_char: None,
            token_start: None,
            indent_stack: Vec::new(),
            nesting: 0,
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next().map(|(i, c)| {
            self.current_pos = i as _;
            self.current_char = Some(c);
            c
        })
    }

    fn prev_token(&mut self) -> Option<&Token> {
        self.tokens.last()
    }

    fn prev_error_mut(&mut self) -> Option<&mut LexError> {
        self.errors.last_mut()
    }

    fn current(&mut self) -> char {
        self.current_char.unwrap()
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
        self.token_start = Some(self.current_pos);
    }

    fn end_token_incl(&mut self, kind: TokenKind) {
        let span = Span::new(self.token_start.unwrap(), self.current_pos + 1);
        let token = Token::new(kind, span);
        self.tokens.push(token);
    }

    fn zero_length_token(&mut self, kind: TokenKind, offset: i64) {
        let span = Span::new((self.current_pos as i64 + offset) as _, (self.current_pos as i64 + offset) as _);
        let token = Token::new(kind, span);
        self.tokens.push(token);
    }

    // fn end_token_excl(&mut self, kind: TokenKind) {
    //     let span = Span::new(self.token_start.unwrap(), self.current);
    //     let tok = Token::new(kind, span);
    //     self.tokens.push(tok);
    // }

    fn token_str_incl(&self) -> &str {
        &self.source[(self.token_start.unwrap() as usize) ..= (self.current_pos as usize)]
    }

    fn single_char_token(&mut self, kind: TokenKind) {
        self.begin_token();
        self.end_token_incl(kind);
    }

    fn error(&mut self, kind: LexErrorKind) {
        self.errors.push(LexError { span: Span::new(self.current_pos, self.current_pos + 1), kind });
    }

    fn nest(&mut self) {
        self.nesting += 1;
    }

    fn unnest(&mut self) {
        self.nesting = self.nesting.saturating_sub(1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Basic
    Annotation,
    Ident,
    IntLit,
    StrLit,

    // Comparison
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    BangEqual,

    // Logical
    And,
    Or,
    Not,
    AmpAmp,
    PipePipe,
    Bang,

    // Bitwise
    Amp,
    Pipe,
    Tilde,
    Caret,
    LessLess,
    GreaterGreater,

    // Math
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Percent,
    
    // Assignment
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    StarStarEqual,
    SlashEqual,
    PercentEqual,
    LessLessEqual,
    GreaterGreaterEqual,
    AmpEqual,
    PipeEqual,
    CaretEqual,

    // Control flow
    If,
    Elif,
    Else,
    For,
    While,
    Break,
    Continue,
    Pass,
    Return,
    Match,
    When,

    // Keywords
    As,
    Assert,
    Await,
    Breakpoint,
    Class,
    ClassName,
    Const,
    Enum,
    Extends,
    Func,
    In,
    Is,
    Namespace,
    Preload,
    Self_,
    Signal,
    Static,
    Super,
    Trait,
    Var,
    Void,
    Yield,
    
    // Punctuation
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    ParenthesisOpen,
    ParenthesisClose,
    Comma,
    SemiColon,
    Period,
    PeriodPeriod,
    Colon,
    Dollar,
    ForwardArrow,
    Underscore,

    // Whitespace
    Newline,
    Indent,
    Dedent,

    // Constants
    // CONST_PI,
    // CONST_TAU,
    // CONST_INF,
    // CONST_NAN,

    // Error message improvement
    // VCS_CONFLICT_MARKER,
    // BACKTICK,
    // QUESTION_MARK,

    // Special
    // ERROR,
    // TK_EOF, // "EOF" is reserved
    Eof
    // TK_MAX
    // Var,

    
    // Colon,

    // Eq,

    // Plus,
    // Minus,
    // Mul,
    // Div,

    // PlusEq,
    // MinusEq,
    // MulEq,
    // DivEq,

    // ParLe,
    // ParRi,
    // Newline,

    // Eof,
}

impl <'a> Lexer<'a> {
    pub fn tokenize(&mut self) {
        use TokenKind::*;
        loop {
            let Some(current_char) = self.bump() else {
                match self.prev_token() {
                    Some(x) if x.kind != Dedent && x.kind != Newline => self.zero_length_token(Newline, 1),
                    _ => (),
                }
                while let Some(_) = self.indent_stack.pop() {
                    self.zero_length_token(Dedent, 1);
                }
                self.zero_length_token(Eof, 1);
                return;
            };
            match current_char {
                ' ' | '\t' => (),
                c if is_xid_start(c) => {
                    self.begin_token();
                    self.bump_while(|c| is_xid_continue(c));
                    let kind = match self.token_str_incl() {
                        "if" => If,
                        "elif" => Elif,
                        "else" => Else,
                        "for" => For,
                        "while" => While,
                        "break" => Break,
                        "continue" => Continue,
                        "pass" => Pass,
                        "return" => Return,
                        "match" => Match,
                        "when" => When,
                        "as" => As,
                        "assert" => Assert,
                        "await" => Await,
                        "breakpoint" => Breakpoint,
                        "class" => Class,
                        "class_name" => ClassName,
                        "const" => Const,
                        "enum" => Enum,
                        "extends" => Extends,
                        "func" => Func,
                        "in" => In,
                        "is" => Is,
                        "namespace" => Namespace,
                        "preload" => Preload,
                        "self" => Self_,
                        "signal" => Signal,
                        "static" => Static,
                        "super" => Super,
                        "trait" => Trait,
                        "var" => Var,
                        "void" => Void,
                        "yield" => Yield,
                        _ => Ident,
                    };
                    self.end_token_incl(kind);
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
                        Ok(_) => {
                            self.end_token_incl(IntLit);
                        }
                        Err(_) => {

                        }
                    }
                }
                '+' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(PlusEqual);
                    } else {
                        self.end_token_incl(Plus);
                    }
                }
                '-' => {
                    self.begin_token();
                    match self.first() {
                        Some('=') => {
                            self.bump();
                            self.end_token_incl(MinusEqual);
                        }
                        Some('>') => {
                            self.bump();
                            self.end_token_incl(ForwardArrow);
                        }
                        _ => self.end_token_incl(Minus),
                    }
                }
                '*' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(StarEqual);
                    } else {
                        self.end_token_incl(Star);
                    }
                }
                '/' => {
                    self.begin_token();
                    if let Some('=') = self.first() {
                        self.bump();
                        self.end_token_incl(SlashEqual);
                    } else {
                        self.end_token_incl(Slash);
                    }
                }
                '=' => {
                    self.begin_token();
                    let kind = match self.first() {
                        Some('=') => {
                            self.bump();
                            EqualEqual
                        },
                        _ => Equal,
                    };
                    self.end_token_incl(kind);
                }
                '(' => self.bracket_open(ParenthesisOpen),
                ')' => self.bracket_close(ParenthesisClose),
                ':' => self.single_char_token(Colon),
                '\n' | '\r' => self.newline(),
                c => {
                    self.error(LexErrorKind::UnexpectedChar(c));
                }
            }
        }
    }

    fn bracket_open(&mut self, kind: TokenKind) {
        self.single_char_token(kind);
        self.nest();
    }

    fn bracket_close(&mut self, kind: TokenKind) {
        self.single_char_token(kind);
        self.unnest();
    }

    fn newline(&mut self) {
        if self.nesting > 0 {
            return;
        }
        self.begin_token();
        loop {
            match (self.current(), self.first()) {
                ('\n', Some('\n')) => { self.bump(); }
                ('\n', Some('\r')) => { self.bump(); }
                ('\r', Some('\n')) => { self.bump(); }
                ('\r', Some(c)) => {
                    self.error(LexErrorKind::UnexpectedChar(c));
                    return;
                }
                ('\r', None) => {
                    self.error(LexErrorKind::UnexpectedEof);
                    return;
                }
                _ => break,
            }
        }
        self.end_token_incl(TokenKind::Newline);
        
        let mut current_ind = Vec::new();
        loop {
            current_ind.push(match self.first() {
                Some(' ') => IndentKind::Space,
                Some('\t') => IndentKind::Tab,
                _ => break,
            });
            self.bump();
        }
        let mut dedent_count = 0;
        let mut cur_iter = current_ind.into_iter().peekable();
        for prev in self.indent_stack.clone().iter() {
            if cur_iter.peek() == None {
                dedent_count += 1;
                continue;
            }
            for e in prev.elements.iter() {
                match (e, cur_iter.next()) {
                    (a, Some(b)) if *a == b => (),
                    (_, None) => {
                        // odd dedentation
                        self.error(LexErrorKind::OddIndentation);
                        return;
                    },
                    _ => {
                        // indent element mismatch
                        self.error(LexErrorKind::InconsistentIndentation);
                        return;
                    }
                }
            }
        }
        for _ in 0..dedent_count {
            self.indent_stack.pop();
            self.zero_length_token(TokenKind::Dedent, 1);
        }
        let new_ind: Vec<_> = cur_iter.collect();
        if new_ind.len() != 0 {
            self.indent_stack.push(Indentation::new(new_ind));
            self.zero_length_token(TokenKind::Indent, 0);
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
        macro_rules! test_err {
            ($src:expr; $($tok:expr, $start:expr, $end:expr);* $(;)?) => {
                {
                    let (tokens, errors) = tokenize($src);
                    assert_eq!(tokens, vec![
                        $(
                            Token::new($tok, Span::new($start, $end)),
                        )*
                    ]);
                    assert_ne!(errors, vec![]);
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
            Newline, 9, 9;
            Eof, 9, 9;
        );
        test!(
            "0";
            IntLit, 0, 1;
            Newline, 1, 1;
            Eof, 1, 1;
        );
        test!(
            "2147483647";
            IntLit, 0, 10;
            Newline, 10, 10;
            Eof, 10, 10;
        );
        test!(
            "( 100 )";
            ParenthesisOpen, 0, 1;
            IntLit, 2, 5;
            ParenthesisClose, 6, 7;
            Newline, 7, 7;
            Eof, 7, 7;
        );
        test!(
            "1 + 2 - 3 * 4 / 5";
            IntLit, 0, 1;
            Plus, 2, 3;
            IntLit, 4, 5;
            Minus, 6, 7;
            IntLit, 8, 9;
            Star, 10, 11;
            IntLit, 12, 13;
            Slash, 14, 15;
            IntLit, 16, 17;
            Newline, 17, 17;
            Eof, 17, 17;
        );
        test!(
            "\n";
            Newline, 0, 1;
            Eof, 1, 1;
        );
        test!(
            "\n\n";
            Newline, 0, 2;
            Eof, 2, 2;
        );
        test!(
            "\n\n\nhello";
            Newline, 0, 3;
            Ident, 3, 8;
            Newline, 8, 8;
            Eof, 8, 8;
        );
        test!(
            "\r\n";
            Newline, 0, 2;
            Eof, 2, 2;
        );
        test!(
            "\r\n\r\n\n";
            Newline, 0, 5;
            Eof, 5, 5;
        );
        test_err!(
            "\r";
            Eof, 1, 1;
        );
        test!(
            "\n    hello\n    world\n";
            Newline, 0, 1;
            Indent, 4, 4;
            Ident, 5, 10;
            Newline, 10, 11;
            Ident, 15, 20;
            Newline, 20, 21;
            Dedent, 21, 21;
            Eof, 21, 21;
        );
        test!(
            "\n \t  hello\n";
            Newline, 0, 1;
            Indent, 4, 4;
            Ident, 5, 10;
            Newline, 10, 11;
            Dedent, 11, 11;
            Eof, 11, 11;
        )
    }
}
