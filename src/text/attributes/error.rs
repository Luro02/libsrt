use crate::utils::{Span, Spanned};
use core::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseAttributeError(Spanned<ParseAttributeErrorKind>);

impl ParseAttributeError {
    #[must_use]
    fn new(kind: ParseAttributeErrorKind, range: impl Into<Span>) -> Self {
        Self(Spanned::new(kind).with_span(range))
    }

    #[must_use]
    pub(crate) fn invalid_quote(range: impl Into<Span>) -> Self {
        Self::new(ParseAttributeErrorKind::InvalidQuote, range)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ParseAttributeErrorKind {
    InvalidQuote,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ColorErrorKind {
    ParseIntError(ParseIntError),
    InvalidRgbString,
    InvalidFormat,
}

impl From<ParseIntError> for ColorErrorKind {
    fn from(value: ParseIntError) -> Self { Self::ParseIntError(value) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorError(Spanned<ColorErrorKind>);

impl ColorError {
    #[inline]
    #[must_use]
    fn new(kind: ColorErrorKind, range: impl Into<Span>) -> Self {
        Self(Spanned::new(kind).with_span(range))
    }

    #[inline]
    #[must_use]
    pub(crate) fn invalid_rgb_string(range: impl Into<Span>) -> Self {
        Self::new(ColorErrorKind::InvalidRgbString, range)
    }

    #[inline]
    #[must_use]
    pub(crate) fn invalid_format(range: impl Into<Span>) -> Self {
        Self::new(ColorErrorKind::InvalidFormat, range)
    }
}

#[doc(hidden)]
impl<E> From<Spanned<E>> for ColorError
where
    ColorErrorKind: From<E>,
{
    fn from(value: Spanned<E>) -> Self { Self(value.map(|v| v.into())) }
}
