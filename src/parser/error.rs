use core::num::ParseIntError;

// TODO: implement fmt::Display for no_std, see https://github.com/dtolnay/thiserror/pull/64
#[cfg(feature = "std")]
use thiserror::Error;

use crate::utils::{Span, Spanned};

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "std", error("{0}"))]
pub struct ParserError(Spanned<ParserErrorKind>);

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ParserErrorKind {
    #[cfg_attr(feature = "std", error("{source}"))]
    ParseIntError { source: ParseIntError },
    #[cfg_attr(feature = "std", error("invalid duration"))]
    InvalidDuration,
}

impl ParserError {
    #[inline]
    #[must_use]
    fn new(kind: ParserErrorKind, range: impl Into<Span>) -> Self {
        Self(Spanned::new(kind).with_span(range))
    }

    #[inline]
    #[must_use]
    pub(crate) fn invalid_duration(range: impl Into<Span>) -> Self {
        Self::new(ParserErrorKind::InvalidDuration, range)
    }

    #[inline]
    #[must_use]
    pub(crate) fn parse_int_error(source: ParseIntError, range: impl Into<Span>) -> Self {
        Self::new(ParserErrorKind::ParseIntError { source }, range)
    }
}
