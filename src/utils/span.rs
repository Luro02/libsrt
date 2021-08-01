use core::ops::{Add, Range, RangeInclusive, RangeTo, RangeToInclusive};
use core::{fmt, usize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    length: usize,
}

// TODO: implement Add for Span or concat?

impl Span {
    #[must_use]
    pub(crate) fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        let (start, end) = f(self.start(), self.end());
        assert!(start <= end);

        Self {
            start,
            length: end - start,
        }
    }

    // TODO: test me!
    #[must_use]
    pub fn sub_span(self, span: impl Into<Span>) -> Option<Self> {
        let span = span.into();

        if span.start() + self.start() > self.end() || span.length() > self.length() {
            return None;
        }

        Some(Self {
            start: self.start() + span.start(),
            length: span.length(),
        })
    }

    /// Returns the `start` of the `Span`.
    #[must_use]
    pub fn start(self) -> usize { self.start }

    /// Returns the `length` of the `Span`.
    #[must_use]
    pub fn length(self) -> usize { self.length }

    /// Returns the `end` of the `Span`.
    #[must_use]
    pub fn end(self) -> usize { self.start() + self.length() }
}

impl Add<usize> for Span {
    type Output = Self;

    fn add(self, other: usize) -> Self { self.map(|start, end| (start + other, end + other)) }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Span")
            .field(&(self.start..self.start + self.length))
            .finish()
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        assert!(range.start <= range.end);

        Self {
            start: range.start,
            length: range.end - range.start,
        }
    }
}

impl From<RangeInclusive<usize>> for Span {
    fn from(range: RangeInclusive<usize>) -> Self { Self::from(*range.start()..*range.end() + 1) }
}

impl From<RangeTo<usize>> for Span {
    fn from(range: RangeTo<usize>) -> Self { Self::from(0..range.end) }
}

impl From<RangeToInclusive<usize>> for Span {
    fn from(range: RangeToInclusive<usize>) -> Self { Self::from(0..=range.end) }
}

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::format;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sub_span() {
        assert_eq!(Span::from(1..3).sub_span(0..2), Some(Span::from(1..3)));
        assert_eq!(Span::from(1..3).sub_span(0..1), Some(Span::from(1..2)));
        assert_eq!(Span::from(1..3).sub_span(0..0), Some(Span::from(1..1)));
        assert_eq!(Span::from(1..3).sub_span(1..4), None);
        //assert_eq!(Span::from(1..3).sub_span(4..1), None);
        assert_eq!(Span::from(1..3).sub_span(4..4), None);
        assert_eq!(Span::from(1..3).sub_span(1..2), Some(Span::from(2..3)));
        assert_eq!(Span::from(1..3).sub_span(2..2), Some(Span::from(3..3)));
    }

    #[test]
    fn test_span_from_range() {
        assert_eq!(Span::from(1..3), Span::from(1..=2));
        assert_eq!(Span::from(..3), Span::from(..=2));
        assert_eq!(Span::from(..3), Span::from(0..=2));
        assert_eq!(Span::from(..=2), Span::from(0..=2));
        assert_eq!(Span::from(1..1), Span::from(1..=0));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_span_debug() {
        assert_eq!(&format!("{:?}", Span::from(11..23)), "Span(11..23)");
    }
}
