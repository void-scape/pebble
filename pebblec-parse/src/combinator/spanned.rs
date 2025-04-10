use crate::lex::buffer::{Buffer, Span, TokenQuery};
use crate::{rules::*, stream::TokenStream};
use std::ops::{Deref, DerefMut};

/// Track the span from the first to the current token in rules `T`.
#[derive(Debug, Default)]
pub struct Spanned<T>(T);

impl<'a, 's, T> ParserRule<'a> for Spanned<T>
where
    T: ParserRule<'a>,
{
    type Output = SpannedR<<T as ParserRule<'a>>::Output>;

    #[track_caller]
    fn parse(stream: &mut TokenStream<'a>) -> RResult<Self::Output> {
        let Some(first) = stream.peek().map(|t| stream.span(t)) else {
            return Err(stream.recover("expected tokens"));
        };

        T::parse(stream).map(|inner| {
            let end = stream.span(stream.prev());
            SpannedR {
                inner,
                span: Span::from_range_u32(first.start..end.end)
                    .with_source(stream.token_buffer().source_id() as u32),
            }
        })
    }
}

#[derive(Debug)]
pub struct SpannedR<T> {
    span: Span,
    inner: T,
}

impl<T> Deref for SpannedR<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SpannedR<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> SpannedR<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
