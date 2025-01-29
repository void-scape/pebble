use crate::lex::kind::*;
use crate::source::Source;
use std::ops::Range;

/// Query about a specific token with a [`TokenId`].
pub trait TokenQuery {
    fn kind(&self, token: TokenId) -> TokenKind;
    fn span(&self, token: TokenId) -> Span;
    fn ident<'a>(&'a self, token: TokenId) -> &'a str;
    fn is_terminator(&self, token: TokenId) -> bool;
}

/// Key into a buffer containing tokens generated by the lexer.
///
/// Used in [`TokenQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenId(usize);

/// Storage of the generated tokens for a given source file.
#[derive(Debug)]
pub struct TokenBuffer<'a> {
    tokens: Vec<Token>,
    source: &'a Source,
}

impl<'a> TokenBuffer<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a Source) -> Self {
        Self { tokens, source }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn source(&self) -> &Source {
        self.source
    }

    pub fn tokens(&self) -> TokenIter {
        TokenIter::new(0, self.len())
    }

    pub fn token(&self, token: TokenId) -> Option<&Token> {
        self.tokens.get(token.0)
    }

    pub fn next(&self, token: TokenId) -> Option<TokenId> {
        (self.len() > token.0 + 1).then_some(TokenId(token.0 + 1))
    }
}

impl TokenQuery for TokenBuffer<'_> {
    #[track_caller]
    fn kind(&self, token: TokenId) -> TokenKind {
        self.token(token)
            .expect("called `TokenQuery::kind` with an invalid token id")
            .kind
    }

    #[track_caller]
    fn span(&self, token: TokenId) -> Span {
        self.token(token)
            .expect("called `TokenQuery::span` with an invalid token id")
            .span
    }

    #[track_caller]
    fn ident<'a>(&'a self, token: TokenId) -> &'a str {
        if !matches!(self.kind(token), TokenKind::Ident) {
            panic!(
                "called `TokenQuery::ident` on a {:?} token",
                self.kind(token)
            );
        }

        &self.source.as_str()[self.span(token).range()]
    }

    fn is_terminator(&self, token: TokenId) -> bool {
        self.kind(token).is_terminator()
    }
}

pub struct TokenIter {
    id_range: Range<usize>,
    index: usize,
}

impl TokenIter {
    fn new(start: usize, end: usize) -> Self {
        Self {
            id_range: start..end,
            index: 0,
        }
    }
}

impl Iterator for TokenIter {
    type Item = TokenId;

    fn next(&mut self) -> Option<Self::Item> {
        self.id_range.contains(&self.index).then(|| {
            let idx = self.index;
            self.index += 1;
            TokenId(idx)
        })
    }
}

/// Metadata about a token held within a [`TokenBuffer`].
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    //pub data: u64,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Token's position within a source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn from_range(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.start as usize..self.end as usize
    }
}
