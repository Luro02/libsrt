use core::str::pattern::{DoubleEndedSearcher, Pattern, ReverseSearcher};
use core::{array, fmt};
use core::{num::ParseIntError, ops::RangeFrom};
use core::{
    ops::{Bound, Deref, DerefMut, Range, RangeBounds},
    str::pattern::Searcher,
};

use super::split::SplitInternal;
use super::{IteratorExt, Span, Spannable, SplitIter, SplitIterN};

pub struct SplitTerminator<'a, P: Pattern<'a>>(pub(super) SplitInternal<'a, P>);

impl<'a, P: Pattern<'a>> Iterator for SplitTerminator<'a, P> {
    type Item = Spanned<&'a str>;

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

impl<'a, P: Pattern<'a>> SplitTerminator<'a, P> {
    pub fn as_str(&self) -> &'a str { self.0.as_str() }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spanned<T> {
    inner: T,
    span: Option<Span>,
}

impl<T> Spanned<T> {
    #[inline]
    #[must_use]
    pub const fn new(inner: T) -> Self { Self { inner, span: None } }

    #[inline]
    #[must_use]
    pub fn with_span(mut self, span: impl Into<Span>) -> Self {
        self.span = Some(span.into());
        self
    }

    #[must_use]
    pub fn map<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        let span = self.span();
        let result = Spanned::new(f(self.inner));

        if let Some(span) = span {
            result.with_span(span)
        } else {
            result
        }

        // TODO: this is how you can write it instead! (check if clippy lints
        // it) span.map_or(result, |span| result.with_span(span))
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> { self.span }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T { self.inner }

    #[must_use]
    pub fn map_with_span<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T, Option<Span>) -> (U, Option<Span>),
    {
        let (inner, span) = f(self.inner, self.span);

        let mut result = Spanned::new(inner);

        if let Some(span) = span {
            result = result.with_span(span);
        }

        result
    }

    #[must_use]
    pub fn map_span<F>(mut self, f: F) -> Spanned<T>
    where
        F: FnOnce(Option<Span>) -> Option<Span>,
    {
        self.span = f(self.span);
        self
    }
}

impl<T: Spannable> Spanned<T> {
    #[must_use]
    pub fn into_parts(self) -> (T, Span) {
        let span = self.span.unwrap_or_else(|| self.inner.span());
        (self.inner, span)
    }

    #[inline]
    #[must_use]
    pub fn range(&self) -> Range<usize> { self.span.unwrap_or_else(|| self.inner.span()).into() }
}

impl<'a> Spanned<&'a str> {
    fn splitn<P: Pattern<'a>>(
        &self,
        remaining: usize,
        pattern: P,
    ) -> impl Iterator<Item = Spanned<&'a str>> {
        SplitIterN::new_spanned(remaining, *self, pattern)
    }

    pub fn split<P: Pattern<'a>>(&self, pattern: P) -> SplitIter<'a, P> {
        SplitIter::new_spanned(*self, pattern)
    }

    pub fn trim(&self) -> Self { self.trim_matches(char::is_whitespace) }

    fn trim_matches<P>(&self, pattern: P) -> Self
    where
        P: Pattern<'a, Searcher: DoubleEndedSearcher<'a>>,
    {
        let mut i = 0;
        let mut j = 0;

        let mut matcher = pattern.into_searcher(self);

        if let Some((a, b)) = matcher.next_reject() {
            i = a;
            j = b;
        }

        if let Some((_, b)) = matcher.next_reject_back() {
            j = b;
        }

        // SAFETY: `Searcher` is known to return valid indices.
        let string = unsafe { self.get_unchecked(i..j) };

        // TODO: adjust with self!
        (string, i..).into()
    }

    pub fn split_once<P: Pattern<'a>>(&self, pattern: P) -> (Self, Option<Self>) {
        if let [Some(first), second] = self.split_at_most::<_, 2>(pattern) {
            (first, second)
        } else {
            unreachable!("split_at_most must return at least one element")
        }
    }

    pub fn split_at_most<P, const N: usize>(&self, pattern: P) -> [Option<Self>; N]
    where
        P: Pattern<'a>,
    {
        self.splitn(N, pattern).try_collect().unwrap()
    }

    pub fn parse_radix_u8(&self, radix: u32) -> Result<u8, Spanned<ParseIntError>> {
        u8::from_str_radix(self, radix)
            //
            .map_err(|error| {
                // TODO: simplify
                let mut result = Spanned::new(error);

                if let Some(span) = self.span() {
                    result = result.with_span(span);
                }

                result
            })
    }

    pub fn split_terminator<P: Pattern<'a>>(&self, pattern: P) -> SplitTerminator<'a, P> {
        SplitTerminator(SplitInternal {
            start: 0,
            end: self.len(),
            matcher: pattern.into_searcher(self),
            allow_trailing_empty: false,
            finished: false,
        })
    }

    // we can not use SliceIndex as trait bound, because all methods are unstable
    pub fn get<R: RangeBounds<usize>>(&self, range: R) -> Option<Self> {
        let range = self.resolve_bound(range);
        let sub_span = Span::from(range);

        // TODO: correctly set the new bounds // TODO: test that they are correct!
        let mut sub_str =
            Spanned::new(self.inner.get(Into::<Range<usize>>::into(sub_span))?).with_span(sub_span);

        if let Some(span) = self.span() {
            sub_str = sub_str.with_span(span.sub_span(sub_span)?);
        }

        Some(sub_str)
    }

    fn resolve_bound<R: RangeBounds<usize>>(&self, range: R) -> Range<usize> {
        let start = {
            match range.start_bound() {
                Bound::Excluded(index) => index + 1,
                Bound::Included(index) => *index,
                Bound::Unbounded => 0,
            }
        };

        let end = {
            match range.end_bound() {
                Bound::Excluded(index) => *index,
                Bound::Included(index) => index + 1,
                Bound::Unbounded => self.inner.len(),
            }
        };

        Range { start, end }
    }

    pub fn sub_ranges<R: RangeBounds<usize>, const N: usize>(
        &self,
        ranges: [R; N],
    ) -> [Option<Self>; N] {
        let mut result = [None; N];

        for (i, range) in array::IntoIter::new(ranges).enumerate() {
            result[i] = self.get(range);
        }

        result
    }

    pub fn remove_start_end(&self, start: char, end: char) -> Option<Self> {
        if self.starts_with(start) && self.ends_with(end) {
            return self.get(start.len_utf8()..self.len() - end.len_utf8());
        } else if self.starts_with(start) {
            // find the `end` char closest to the end of the string:
            let (end, _) = self
                .char_indices()
                .rev()
                .find(|(i, c)| *c == end && *i >= start.len_utf8())?;

            return self.get(start.len_utf8()..end);
        }

        None
    }

    pub fn trim_start_matches<P: Pattern<'a>>(&self, pattern: P) -> Self {
        let mut index = self.len();
        let mut matcher = pattern.into_searcher(self);

        if let Some((start, _)) = matcher.next_reject() {
            index = start;
        }

        // SAFETY: `Searcher` is known to return valid indices.
        let string = unsafe { self.get_unchecked(index..self.len()) };

        // TODO: adjust with self!
        (string, index..).into()
    }

    pub fn trim_end_matches<P: Pattern<'a>>(&self, pattern: P) -> Self {
        let mut index = self.len();
        let mut matcher = pattern.into_searcher(self);

        if let Some((start, _)) = matcher.next_reject() {
            index = start;
        }

        // SAFETY: `Searcher` is known to return valid indices.
        let string = unsafe { self.get_unchecked(index..self.len()) };

        (string, index..).into()
    }

    // TODO: make use of this function/improve
    #[must_use]
    pub fn str_span(&self) -> Span { self.span.unwrap_or_else(|| (0..self.len()).into()) }
}

impl<'a> From<(&'a str, RangeFrom<usize>)> for Spanned<&'a str> {
    fn from(value: (&'a str, RangeFrom<usize>)) -> Self {
        let start = value.1.start;
        Self::new(value.0).with_span(start..start + value.0.len())
    }
}

/// Allows to compare for example a `Spanned<bool>` with a `bool`
impl<T: PartialEq> PartialEq<T> for Spanned<T> {
    fn eq(&self, other: &T) -> bool { &self.inner == other }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.inner.fmt(f) }
}

// TODO: needed?
#[cfg(feature = "std")]
impl<T: ::std::error::Error> ::std::error::Error for Spanned<T> {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> { self.inner.source() }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_spanned_remove_start_end() {
        assert_eq!(
            "(example)".remove_start_end('(', ')'),
            Some(Spanned::new("example").with_span(1.."(example".len()))
        );
        assert_eq!(
            "( example )".remove_start_end('(', ')'),
            Some(Spanned::new(" example ").with_span(1.."( example ".len()))
        );
        assert_eq!(
            "( example ) hello".remove_start_end('(', ')'),
            Some(Spanned::new(" example ").with_span(1.."( example ".len()))
        );
        assert_eq!("( example hello".remove_start_end('(', ')'), None);
        assert_eq!(
            "()".remove_start_end('(', ')'),
            Some(Spanned::new("").with_span(1..1))
        );
        assert_eq!("(".remove_start_end('(', ')'), None);
        assert_eq!(")".remove_start_end('(', ')'), None);
        assert_eq!("".remove_start_end('(', ')'), None);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get() {
        // TODO: verify that spans have the correct length! (length must be equal to the
        // string length!)
        let spanned = Spanned::new("hello").with_span(1..=5);

        assert_eq!(
            spanned.get(1..),
            Some(Spanned::new("ello").with_span(2..=5))
        );

        assert_eq!(spanned.get(2..), Some(Spanned::new("llo").with_span(3..=5)));

        assert_eq!(spanned.get(4..), Some(Spanned::new("o").with_span(5..=5)));
        // TODO: is this correct?
        assert_eq!(spanned.get(5..), Some(Spanned::new("").with_span(6..6)));

        //
        assert_eq!(spanned.get(1..2), Some(Spanned::new("e").with_span(2..3)));
        assert_eq!(spanned.get(0..5), Some(spanned));

        assert_eq!(spanned.get(1..3), Some(Spanned::new("el").with_span(2..4)));

        //
        assert_eq!(spanned.get(1..=1), Some(Spanned::new("e").with_span(2..3)));
        assert_eq!(spanned.get(0..=4), Some(spanned));

        assert_eq!(spanned.get(1..=2), Some(Spanned::new("el").with_span(2..4)));
    }
}
