use crate::lex::buffer::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lit<'a> {
    pub span: Span,
    pub kind: &'a LitKind<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LitKind<'a> {
    Int(i64),
    Str(&'a str),
}

impl LitKind<'_> {
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }
}
