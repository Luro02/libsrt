use crate::utils::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
enum ParseTagErrorKind {
    MissingBrackets,
    /// expected open tag, found close tag
    ExpectedOpenTag,
    /// expected close tag, found open tag
    ExpectedCloseTag,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseTagError(Spanned<ParseTagErrorKind>);

impl ParseTagError {
    #[inline]
    #[must_use]
    fn new(kind: ParseTagErrorKind, range: impl Into<Span>) -> Self {
        Self(Spanned::new(kind).with_span(range))
    }

    #[inline]
    #[must_use]
    pub(crate) fn missing_brackets(range: impl Into<Span>) -> Self {
        Self::new(ParseTagErrorKind::MissingBrackets, range)
    }

    #[inline]
    #[must_use]
    pub(crate) fn expected_open_tag(range: impl Into<Span>) -> Self {
        Self::new(ParseTagErrorKind::ExpectedOpenTag, range)
    }

    #[inline]
    #[must_use]
    pub(crate) fn expected_close_tag(range: impl Into<Span>) -> Self {
        Self::new(ParseTagErrorKind::ExpectedCloseTag, range)
    }
}
